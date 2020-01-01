#![allow(dead_code)]
use crate::vec2::Vec2;
use num::{CheckedAdd, CheckedSub, One};
use std::iter::TrustedLen;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    #[inline]
    pub fn vertical() -> impl Iterator<Item = Direction> + ExactSizeIterator + TrustedLen + Clone {
        [Direction::North, Direction::South].iter().cloned()
    }
    #[inline]
    pub fn horizontal() -> impl Iterator<Item = Direction> + ExactSizeIterator + TrustedLen + Clone {
        [Direction::West, Direction::East].iter().cloned()
    }
    #[inline]
    pub fn each() -> impl Iterator<Item = Direction> + ExactSizeIterator + TrustedLen + Clone {
        [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ]
        .iter()
        .cloned()
    }

    #[inline]
    pub fn vertical_arr<T, F>(mut f: F) -> [T; 2]
    where
        T: Sized,
        F: FnMut(Direction) -> T,
    {
        [f(Direction::North), f(Direction::South)]
    }
    #[inline]
    pub fn horizontal_arr<T, F>(mut f: F) -> [T; 2]
    where
        T: Sized,
        F: FnMut(Direction) -> T,
    {
        [f(Direction::West), f(Direction::East)]
    }
    #[inline]
    pub fn each_arr<T, F>(mut f: F) -> [T; 4]
    where
        T: Sized,
        F: FnMut(Direction) -> T,
    {
        [
            f(Direction::North),
            f(Direction::South),
            f(Direction::West),
            f(Direction::East),
        ]
    }

    #[inline]
    pub fn reverse(&self) -> Direction {
        use Direction::*;
        match *self {
            North => South,
            South => North,
            West => East,
            East => West,
        }
    }

    pub fn clockwise(&self) -> Direction {
        use Direction::*;
        match *self {
            North => East,
            South => West,
            West => North,
            East => South,
        }
    }
    pub fn counterclockwise(&self) -> Direction {
        use Direction::*;
        match *self {
            North => West,
            South => East,
            West => South,
            East => North,
        }
    }
}

pub trait MoveInDirection: Sized {
    type Bound;
    type Number: Ord + One;
    fn move_within_bounds(
        &self,
        direction: Direction,
        distance: Self::Number,
        min: Self::Bound,
        max: Self::Bound,
    ) -> Option<Self>;
    fn move_in_direction_checked(
        &self,
        direction: Direction,
        distance: Self::Number,
    ) -> Option<Self>;
    #[inline]
    fn step_in_direction_checked(&self, direction: Direction) -> Option<Self> {
        self.move_in_direction_checked(direction, Self::Number::one())
    }

    #[inline]
    fn move_in_direction(&self, direction: Direction, distance: Self::Number) -> Self {
        self.move_in_direction_checked(direction, distance)
            .expect("integer overflow")
    }
    #[inline]
    fn step_in_direction(&self, direction: Direction) -> Self {
        self.step_in_direction_checked(direction)
            .expect("integer overflow")
    }
}

impl<I> MoveInDirection for Vec2<I>
where
    I: Clone + Ord + One + CheckedAdd + CheckedSub,
{
    type Bound = Self;
    type Number = I;
    #[inline]
    fn move_within_bounds(
        &self,
        direction: Direction,
        distance: Self::Number,
        min: Self,
        max: Self,
    ) -> Option<Self> {
        self.move_in_direction_checked(direction, distance)
            .filter(|pos| pos.x >= min.x && pos.x <= max.x && pos.y >= min.y && pos.y <= max.y)
    }
    #[inline]
    fn move_in_direction_checked(&self, direction: Direction, distance: I) -> Option<Self> {
        let (x, y) = match direction {
            Direction::North => (self.x.clone(), self.y.checked_sub(&distance)?),
            Direction::South => (self.x.clone(), self.y.checked_add(&distance)?),
            Direction::West => (self.x.checked_sub(&distance)?, self.y.clone()),
            Direction::East => (self.x.checked_add(&distance)?, self.y.clone()),
        };
        Some(Vec2::new(x, y))
    }
}
