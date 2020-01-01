use crate::direction::{Direction, MoveInDirection};
use crate::graph::{astar_once, bfs_meta};
use crate::intcode::{
    sparse_memory, util::parse_intcode, Error as icError, IoOperation, Value, VM,
};
use crate::vec2::AabbIteratorEx;
use crate::HashMap;
use arrayvec::ArrayVec;
use std::convert::{TryFrom, TryInto};

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

                position = position.step_in_direction(direction);
                *value = Some(match direction {
                    Direction::North => 1,
                    Direction::South => 2,
                    Direction::West => 3,
                    Direction::East => 4,
                });
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
                        position = position.step_in_direction(dir.reverse());
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
    let mut max_dist = 0;
    bfs_meta(
        (oxygen_position, 0),
        |&pos, &dist| {
            neighbors(&map, pos)
                .into_iter()
                .map(move |pos| (pos, dist + 1))
        },
        |_, &dist| {
            max_dist = dist;
            false
        },
    );

    Ok(max_dist)
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
