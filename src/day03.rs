module!(pt1: parse, pt2: parse);

use num::traits::Zero;
use std::collections::{HashMap, HashSet};

type Vec2 = crate::vec2::Vec2<i64>;
type Wire = Vec<Command>;

fn for_each_position<I, F>(commands: I, mut callback: F)
where
    I: IntoIterator<Item = Command>,
    F: FnMut(Vec2) -> (),
{
    let mut position = Vec2::zero();
    for command in commands {
        for _ in 0..command.distance {
            match command.direction {
                Direction::Left => position.x -= 1,
                Direction::Up => position.y -= 1,
                Direction::Right => position.x += 1,
                Direction::Down => position.y += 1,
            }
            callback(position);
        }
    }
}

fn pt1(wires: (Wire, Wire)) -> Result<i64> {
    let mut wire1_positions = HashSet::new();
    let mut collisions = HashSet::new();

    for_each_position(wires.0.into_iter(), |position| {
        wire1_positions.insert(position);
    });

    for_each_position(wires.1.into_iter(), |position| {
        if wire1_positions.contains(&position) {
            collisions.insert(position);
        }
    });

    collisions
        .iter()
        .map(|position| position.x.abs() + position.y.abs())
        .min()
        .ok_or(AoCError::NoSolution)
}

fn pt2(wires: (Wire, Wire)) -> Result<i64> {
    let mut wire1_positions = HashMap::new();
    let mut collisions = HashMap::new();

    let mut distance = 0;
    for_each_position(wires.0.into_iter(), |position| {
        distance += 1;
        wire1_positions.entry(position).or_insert(distance);
    });
    distance = 0;
    for_each_position(wires.1.into_iter(), |position| {
        distance += 1;
        if wire1_positions.contains_key(&position) {
            collisions.entry(position).or_insert(distance);
        }
    });

    collisions
        .into_iter()
        .map(|(position, distance2)| {
            let distance1 = wire1_positions[&position];
            distance1 + distance2
        })
        .min()
        .ok_or(AoCError::NoSolution)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Command {
    direction: Direction,
    distance: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

fn parse(s: &str) -> IResult<&str, (Wire, Wire)> {
    use parsers::*;
    let direction = map(one_of("LURD"), |c: char| match c {
        'L' => Direction::Left,
        'U' => Direction::Up,
        'R' => Direction::Right,
        'D' => Direction::Down,
        _ => unreachable!(),
    });
    let command = map(pair(direction, u32_str), |(direction, distance)| Command {
        direction,
        distance: distance as i64,
    });
    let wire_commands = separated_list(char(','), command);
    map_res(separated_list(line_ending, wire_commands), |wires| {
        if wires.len() != 2 {
            return Err(AoCError::IncorrectInput("expected 2 wires"));
        }
        let mut iter = wires.into_iter();
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        return Ok((a, b));
    })(s)
}
