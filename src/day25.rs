use crate::direction::Direction;
use crate::graph::AStar;
use crate::intcode::{growing_memory, util::parse_intcode, GrowingMemory, State, Value, VM};
use crate::{Atom, HashMap, HashSet};
use arrayvec::ArrayVec;
use std::collections::hash_map::Entry;

module!(pt1: parse_intcode);

#[derive(Debug, Clone)]
struct AdventureVM {
    vm: VM<GrowingMemory>,
    out_buff: String,
    write_buff: String,
    input_buff: Vec<Value>,
}

impl AdventureVM {
    fn new(memory: Vec<Value>) -> AdventureVM {
        AdventureVM {
            vm: VM::new(growing_memory(memory)),
            out_buff: String::new(),
            write_buff: String::new(),
            input_buff: Vec::new(),
        }
    }

    fn resume<F>(&mut self, mut take_step: F) -> Result<bool>
    where
        F: FnMut(&String, &mut String) -> Result<bool>,
    {
        loop {
            match self.vm.state {
                State::Halted => {
                    return Ok(true);
                }
                State::Reading => {
                    if let Some(c) = self.input_buff.pop() {
                        self.vm.state = State::Idle;
                        self.vm.registers.pending_in = Some(c);
                        continue;
                    } else {
                        self.write_buff.clear();
                        if !take_step(&self.out_buff, &mut self.write_buff)? {
                            return Ok(false);
                        }
                        self.out_buff.clear();
                        self.input_buff.clear();
                        for c in self.write_buff.chars() {
                            if c != '\n' && c != ' ' && !c.is_ascii_alphanumeric() {
                                return Err(AoCError::Logic("command contains invalid characters"));
                            }
                            self.input_buff.push(c as u8 as Value);
                        }
                        self.input_buff.push(b'\n' as Value);
                        self.input_buff.reverse();
                    }
                }
                State::Writing => {
                    let value = self.vm.registers.pending_out.unwrap();
                    if value < 0 || value >= 128 {
                        return Err(AoCError::IncorrectInput(
                            "program outputted non-ascii character",
                        ));
                    }
                    self.out_buff.push(value as u8 as char);
                    self.vm.registers.pending_out = None;
                    self.vm.state = State::Idle;
                }
                State::Idle => {
                    self.vm.run_one()?;
                }
            }
        }
    }

    fn resume_no_exit<F>(&mut self, mut take_step: F) -> Result<()>
    where
        F: FnMut(&String, &mut String) -> Result<bool>,
    {
        if self.resume(&mut take_step)? {
            return Err(AoCError::Logic("VM terminated prematurely"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Adventure {
    vm: AdventureVM,
    inventory: HashSet<Atom>,
    rooms: HashMap<Atom, Room>,
    astar: AStar<Atom, usize>,
    current_room: Atom,
}

#[derive(Debug, Clone)]
struct Room {
    name: Atom,
    description: Atom,
    connections: ArrayVec<[(Direction, Atom); 4]>,
    items: Vec<Atom>,
}

impl Adventure {
    fn new(memory: Vec<Value>) -> Adventure {
        Adventure {
            vm: AdventureVM::new(memory),
            inventory: HashSet::new(),
            rooms: HashMap::new(),
            astar: AStar::new(),
            current_room: Atom::from("Hull Breach"),
        }
    }

    fn map_and_collect(&mut self) -> Result<()> {
        self.rooms.clear();
        let mut stack: Vec<(Direction, Atom)> = Vec::new();
        let rooms = &mut self.rooms;
        let current_room = &mut self.current_room;
        let inventory = &mut self.inventory;
        let start_room = current_room.clone();
        let mut did_loot = false;
        self.vm.resume_no_exit(|input, output| {
            if !did_loot {
                let mut parsed_room = parse_room(input)?;
                // Update room connections
                if !stack.is_empty() {
                    let (dir, prev_name) = stack.last().unwrap();
                    let prev_room = rooms.get_mut(prev_name).unwrap();
                    // Update previous room
                    prev_room
                        .connections
                        .iter_mut()
                        .find(|(d, _)| *d == *dir)
                        .unwrap()
                        .1 = parsed_room.name.clone();

                    // Update current room
                    let dir = dir.reverse();
                    parsed_room
                        .connections
                        .iter_mut()
                        .find(|(d, _)| *d == dir)
                        .unwrap()
                        .1 = prev_name.clone();
                }
                *current_room = parsed_room.name.clone();
                // Insert current room (if necessary)
                match rooms.entry(parsed_room.name.clone()) {
                    Entry::Vacant(slot) => {
                        slot.insert(parsed_room);
                    }
                    Entry::Occupied(slot) => {
                        assert_eq!(parsed_room.name, slot.get().name);
                    }
                }
            };

            let room = rooms.get_mut(current_room).unwrap();
            if let Some(idx) =
                room.items
                    .iter()
                    .position(|item| match <Atom as std::ops::Deref>::deref(item) {
                        "escape pod" => false,
                        "giant electromagnet" => false,
                        "infinite loop" => false,
                        "molten lava" => false,
                        "photons" => false,
                        _ => true,
                    })
            {
                let item = room.items.remove(idx);
                output.push_str("take ");
                output.push_str(&item);
                assert!(inventory.insert(item));
                did_loot = true;
                return Ok(true);
            }

            let into_dir = room
                .connections
                .iter()
                .filter_map(
                    |(dir, next_name)| {
                        if next_name == "" {
                            Some(*dir)
                        } else {
                            None
                        }
                    },
                )
                .next()
                .filter(|_| &room.name != "Security Checkpoint");

            let dir = if let Some(dir) = into_dir {
                stack.push((dir, room.name.clone()));
                dir
            } else if let Some((dir, _)) = stack.pop() {
                dir.reverse()
            } else {
                return Ok(false);
            };

            output.push_str(dir_to_str(dir));
            did_loot = false;
            Ok(true)
        })?;
        if &start_room != current_room {
            return Err(AoCError::Logic(
                "scanning map didn't return to the room it started at",
            ));
        }
        Ok(())
    }

    fn clear_security_checkpoint(&mut self) -> Result<String> {
        self.move_to(Atom::from("Security Checkpoint"))?;
        let move_dir = dir_to_str(
            self.rooms[&self.current_room]
                .connections
                .iter()
                .find(|(_, room)| room == "")
                .unwrap()
                .0,
        );

        assert!(self.inventory.len() < 32);
        let mut carrying = (1u32 << self.inventory.len()) - 1;
        let inventory = self.inventory.iter().cloned().collect::<Vec<_>>();
        let mut is_moving = false;

        if self.vm.resume(|_input, output| {
            is_moving = !is_moving;
            if is_moving {
                output.push_str(move_dir);
                Ok(true)
            } else if carrying == 0 {
                Ok(false)
            } else {
                let diffs = carrying ^ (carrying - 1);
                assert_ne!(diffs, 0);
                carrying -= 1;
                for (idx, item) in inventory.iter().enumerate() {
                    if (diffs >> idx) & 1 == 1 {
                        output.push_str(if (carrying >> idx) & 1 == 1 {
                            "take "
                        } else {
                            "drop "
                        });
                        output.push_str(item);
                        output.push('\n');
                    }
                }
                output.pop();
                Ok(true)
            }
        })? {
            let beg = self
                .vm
                .out_buff
                .find("Oh, hello! You should be able to get in by typing ")
                .ok_or(AoCError::IncorrectInput("unexpected outcome"))?;
            let end = self
                .vm
                .out_buff
                .find(" on the keypad at the main airlock.")
                .ok_or(AoCError::IncorrectInput("unexpected outcome"))?;
            let code = &self.vm.out_buff[beg + 50..end];

            let mut carried = inventory
                .iter()
                .enumerate()
                .filter_map(|(idx, item)| {
                    if (carrying >> idx) & 1 == 1 {
                        Some(<Atom as std::ops::Deref>::deref(item))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            carried.sort_unstable();
            return Ok(format!(
                "Code: {}, with items: {}",
                code,
                carried.join(", ")
            ));
        }

        Err(AoCError::Logic(
            "couldn't get past pressure sensitive floor",
        ))
    }

    fn move_to(&mut self, room: Atom) -> Result<()> {
        let rooms = &self.rooms;
        let astar = &mut self.astar;
        let path = astar
            .solve(
                self.current_room.clone(),
                |name| {
                    rooms[name]
                        .connections
                        .iter()
                        .map(|(_, room)| (room.clone(), 1))
                },
                |_| 0,
                |name| name == &room,
            )
            .ok_or(AoCError::Logic("pathfinding failed"))?;
        let mut steps = path
            .iter()
            .zip(path.iter().skip(1))
            .map(|((prev, _), (next, _))| {
                rooms[&prev]
                    .connections
                    .iter()
                    .find(|(_, name)| name == next)
                    .cloned()
                    .unwrap()
            });
        let vm = &mut self.vm;
        let current_room = &mut self.current_room;
        vm.resume_no_exit(|_input, output| {
            Ok(if let Some((dir, name)) = steps.next() {
                output.push_str(dir_to_str(dir));
                *current_room = name;
                true
            } else {
                false
            })
        })
    }
}

fn dir_to_str(dir: Direction) -> &'static str {
    match dir {
        Direction::North => "north",
        Direction::South => "south",
        Direction::West => "west",
        Direction::East => "east",
    }
}

fn pt1(memory: Vec<Value>) -> Result<String> {
    if std::env::args_os()
        .find(|arg| arg == "--interactive")
        .is_some()
    {
        use crate::intcode::ascii::interactive;
        println!();
        let mut vm = VM::new(growing_memory(memory));
        interactive(&mut vm)?;
        Ok(String::new())
    } else {
        let mut adventure = Adventure::new(memory);
        adventure.map_and_collect()?;
        adventure.clear_security_checkpoint()
    }
}

fn parse_room(s: &str) -> Result<Room> {
    use crate::module::ToModuleResult;
    use parsers::*;

    let name = delimited(
        tag("== "),
        verify(printable1::<_, (_, ErrorKind)>, |s: &str| {
            s.len() > 3 && s.ends_with(" ==")
        }),
        char('\n'),
    );
    let description = terminated(printable1, char('\n'));
    let door = delimited(tag("- "), printable1, char('\n'));
    let doors = preceded(tag("\nDoors here lead:\n"), many1(door));
    let item = delimited(tag("- "), printable1, char('\n'));
    let items = opt(preceded(tag("\nItems here:\n"), many1(item)));
    let command = tag("\nCommand?\n");

    let room_tuple = delimited(
        fold_many1(char('\n'), (), |_, _| ()),
        tuple((name, description, doors, items)),
        command,
    );
    let (name, description, connections, items) = room_tuple(s).to_module_result()?;
    Ok(Room {
        name: Atom::from(&name[..name.len() - 3]),
        description: Atom::from(description),
        connections: connections
            .into_iter()
            .map(|s| match s {
                "north" => Ok(Direction::North),
                "south" => Ok(Direction::South),
                "west" => Ok(Direction::West),
                "east" => Ok(Direction::East),
                _ => Err(AoCError::IncorrectInput(
                    "direction isn't one of north, south, west or east",
                )),
            })
            .map(|dir| Ok((dir?, Atom::from(""))))
            .collect::<Result<_>>()?,
        items: items
            .unwrap_or(Vec::new())
            .into_iter()
            .map(|s: &str| Atom::from(s))
            .collect(),
    })
}
