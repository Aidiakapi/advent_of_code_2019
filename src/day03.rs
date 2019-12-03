module!(pt1: parse, pt2: parse);

use hashbrown::{HashMap, HashSet};
use num::traits::Zero;
use std::iter::FromIterator;

type Vec2 = crate::vec2::Vec2<i64>;
type Wire = Vec<Command>;

struct WireIter<I>
where
    I: Iterator<Item = Command>,
{
    iter: I,
    curr: Option<Command>,
    pos: Vec2,
}

impl<I> Iterator for WireIter<I>
where
    I: Iterator<Item = Command>,
{
    type Item = Vec2;
    fn next(&mut self) -> Option<Vec2> {
        let mut curr = self.curr.as_mut()?;
        while curr.distance == 0 {
            self.curr = self.iter.next();
            curr = self.curr.as_mut()?;
        }
        curr.distance -= 1;
        curr.direction.modify_position(&mut self.pos, 1);
        Some(self.pos)
    }
}

fn iter_wire<I>(iter: I) -> WireIter<I::IntoIter>
where
    I: IntoIterator<Item = Command>,
{
    let mut iter = iter.into_iter();
    let curr = iter.next();
    WireIter {
        iter,
        curr,
        pos: Vec2::zero(),
    }
}

fn pt1(wires: (Wire, Wire)) -> Result<i64> {
    let wire1_positions: HashSet<Vec2> = HashSet::from_iter(iter_wire(wires.0));
    iter_wire(wires.1)
        .filter(|pos| wire1_positions.contains(pos))
        .map(|pos| pos.x.abs() + pos.y.abs())
        .min()
        .ok_or(AoCError::NoSolution)
}

fn pt2(wires: (Wire, Wire)) -> Result<usize> {
    let wire1_positions: HashMap<Vec2, usize> =
        HashMap::from_iter(iter_wire(wires.0).enumerate().map(|(idx, pos)| (pos, idx)));
    iter_wire(wires.1)
        .enumerate()
        .filter_map(|(dist2, pos)| wire1_positions.get(&pos).map(|dist1| dist1 + dist2))
        .min()
        // add 2 to correct for .enumerate() starting at 0 instead of 1
        .map(|distance| distance + 2)
        .ok_or(AoCError::NoSolution)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Command {
    direction: Direction,
    distance: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    pub fn modify_position(&self, pos: &mut Vec2, distance: i64) {
        match *self {
            Direction::Left => pos.x -= distance,
            Direction::Up => pos.y -= distance,
            Direction::Right => pos.x += distance,
            Direction::Down => pos.y += distance,
        }
    }
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
    let command = map(pair(direction, usize_str), |(direction, distance)| {
        Command {
            direction,
            distance,
        }
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

#[cfg(test)]
mod test {
    use super::*;
    use ::test::{black_box, Bencher};

    #[bench]
    fn day03_pt1(b: &mut Bencher) {
        let input = std::fs::read_to_string("./data/day03.txt").unwrap();
        let input = input.trim();
        b.iter(|| pt1(parse(black_box(input)).unwrap().1));
    }

    #[bench]
    fn day03_pt2(b: &mut Bencher) {
        let input = std::fs::read_to_string("./data/day03.txt").unwrap();
        let input = input.trim();
        b.iter(|| pt2(parse(black_box(input)).unwrap().1));
    }
}
