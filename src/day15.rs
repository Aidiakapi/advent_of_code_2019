use crate::graph::{astar_once, dfs};
use crate::intcode::{
    sparse_memory, util::parse_intcode, Error as icError, IoOperation, Value, VM,
};
use crate::vec2::AabbIteratorEx;
use crate::HashMap;
use arrayvec::ArrayVec;
use std::collections::hash_map::Entry;
use std::convert::{Into, TryFrom, TryInto};

type Vec2 = crate::vec2::Vec2<Value>;
type Map = HashMap<Vec2, Tile>;

module!(pt1: parse_intcode, pt2: parse_intcode);

fn map_area(memory: &Vec<Value>) -> Result<(Map, Vec2)> {
    let mut map = HashMap::new();
    let mut stack: Vec<Direction> = Vec::new();
    let mut position = Vec2::default();

    let mut oxygen_position = None;

    let mut vm = VM::new(sparse_memory(memory.iter().cloned()));
    vm.run_all_async(|io| {
        match io {
            IoOperation::Read(value) => {
                let direction = if !map.contains_key(&Vec2::new(position.x, position.y - 1)) {
                    stack.push(Direction::North);
                    Direction::North
                } else if !map.contains_key(&Vec2::new(position.x, position.y + 1)) {
                    stack.push(Direction::South);
                    Direction::South
                } else if !map.contains_key(&Vec2::new(position.x - 1, position.y)) {
                    stack.push(Direction::West);
                    Direction::West
                } else if !map.contains_key(&Vec2::new(position.x + 1, position.y)) {
                    stack.push(Direction::East);
                    Direction::East
                } else if let Some(prev_direction) = stack.pop() {
                    prev_direction.reverse()
                } else {
                    // Exit condition
                    return Ok(());
                };

                direction.modify_position(&mut position);
                *value = Some(direction.into());
            }
            IoOperation::Write(value) => {
                // Update map
                let tile: Tile = value.try_into()?;
                #[allow(unused_variables)]
                let prev = map.insert(position, tile);
                debug_assert!(prev.is_none() || prev.unwrap() == tile);

                match tile {
                    // Undo if bumped into a wall
                    Tile::Wall => {
                        let dir = stack.pop().unwrap();
                        dir.reverse().modify_position(&mut position);
                    }
                    Tile::OxygenSystem => {
                        if let Some(prev) = oxygen_position {
                            if prev != position {
                                return Err(icError::Custom(format!(
                                    "multiple oxygen systems encountered ({} and {})",
                                    prev, position
                                )));
                            }
                        }
                        oxygen_position = Some(position);
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    })?;
    let oxygen_position =
        oxygen_position.ok_or(AoCError::IncorrectInput("no oxygen system encountered"))?;

    Ok((map, oxygen_position))
}

#[allow(unused)]
fn visualize_map(map: &Map) -> String {
    let (min, max) = map.keys().cloned().aabb().unwrap();
    let mut s = String::new();
    for y in min.y..=max.y {
        for x in min.x..=max.x {
            s.push(match map.get(&Vec2::new(x, y)) {
                Some(Tile::Open) => '.',
                Some(Tile::OxygenSystem) => 'O',
                Some(Tile::Wall) => '#',
                None => ' ',
            });
        }
        s.push('\n');
    }
    s.pop();
    s
}

fn neighbors(map: &Map, pos: Vec2) -> ArrayVec<[Vec2; 4]> {
    let mut neighbors: ArrayVec<[Vec2; 4]> = ArrayVec::new();
    neighbors.push(Vec2::new(pos.x, pos.y - 1));
    neighbors.push(Vec2::new(pos.x, pos.y + 1));
    neighbors.push(Vec2::new(pos.x - 1, pos.y));
    neighbors.push(Vec2::new(pos.x + 1, pos.y));
    for i in (0..4).rev() {
        match map.get(&neighbors[i]) {
            None | Some(Tile::Wall) => {
                neighbors.swap_remove(i);
            }
            Some(Tile::Open) | Some(Tile::OxygenSystem) => {}
        }
    }
    neighbors
}

fn pt1(memory: Vec<Value>) -> Result<usize> {
    let (map, oxygen_position) = map_area(&memory)?;

    Ok(astar_once(
        Vec2::new(0, 0),
        |pos| neighbors(&map, *pos).into_iter().map(|p| (p, 1)),
        |pos| {
            let delta = pos.delta(oxygen_position);
            (delta.x + delta.y) as usize
        },
        |pos| *pos == oxygen_position,
    )
    .unwrap()
    .last()
    .unwrap()
    .1)
}

fn pt2(memory: Vec<Value>) -> Result<usize> {
    let (map, oxygen_position) = map_area(&memory)?;
    let mut dist_map = HashMap::new();
    dfs((oxygen_position, 0), |(pos, dist)| {
        let mut neighbors = neighbors(&map, pos);
        match dist_map.entry(pos) {
            Entry::Occupied(mut prev_dist) => {
                let prev_dist = prev_dist.get_mut();
                // Already visited this position with a more optimal route
                if *prev_dist <= dist {
                    neighbors.clear();
                }
                // Revisiting from an optimized route
                else {
                    *prev_dist = dist;
                }
            }
            Entry::Vacant(slot) => {
                slot.insert(dist);
            }
        }

        neighbors.into_iter().map(move |pos| (pos, dist + 1))
    });

    Ok(dist_map.values().cloned().max().unwrap())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl From<Direction> for Value {
    fn from(dir: Direction) -> Value {
        use Direction::*;
        match dir {
            North => 1,
            South => 2,
            West => 3,
            East => 4,
        }
    }
}

impl Direction {
    #[inline]
    fn reverse(&self) -> Direction {
        use Direction::*;
        match *self {
            North => South,
            South => North,
            West => East,
            East => West,
        }
    }

    fn modify_position(&self, position: &mut Vec2) {
        use Direction::*;
        match *self {
            North => position.y -= 1,
            South => position.y += 1,
            West => position.x -= 1,
            East => position.x += 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Open,
    OxygenSystem,
}

impl TryFrom<Value> for Tile {
    type Error = icError;
    fn try_from(value: Value) -> std::result::Result<Tile, icError> {
        Ok(match value {
            0 => Tile::Wall,
            1 => Tile::Open,
            2 => Tile::OxygenSystem,
            _ => return Err(icError::Custom(format!("invalid tile ({})", value))),
        })
    }
}
