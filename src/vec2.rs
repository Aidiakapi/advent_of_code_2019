#![allow(dead_code)]

use num::traits::{
    identities::{One, Zero},
    sign::{Signed, Unsigned},
    Num,
};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;
use std::iter::{ExactSizeIterator, IntoIterator, Iterator, TrustedLen};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use std::str::FromStr;

pub type Vec2i = Vec2<i32>;
pub type Vec2us = Vec2<usize>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Hash)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    #[inline(always)]
    pub const fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }

    #[inline(always)]
    pub fn with_x(self, x: T) -> Self {
        Vec2 { x, y: self.y }
    }
    #[inline(always)]
    pub fn with_y(self, y: T) -> Self {
        Vec2 { x: self.x, y }
    }

    #[inline(always)]
    pub fn all<F>(&self, other: &Self, mut f: F) -> bool
    where
        F: FnMut(&T, &T) -> bool,
    {
        f(&self.x, &other.x) && f(&self.y, &other.y)
    }

    #[inline(always)]
    pub fn any<F>(&self, other: &Self, mut f: F) -> bool
    where
        F: FnMut(&T, &T) -> bool,
    {
        f(&self.x, &other.x) || f(&self.y, &other.y)
    }

    pub fn convert<U>(self) -> Result<Vec2<U>, <U as TryFrom<T>>::Error>
    where
        U: TryFrom<T>,
    {
        Ok(Vec2 {
            x: U::try_from(self.x)?,
            y: U::try_from(self.y)?,
        })
    }
}

impl<T> Vec2<T>
where
    T: PartialOrd + Sub<Output = T>,
{
    pub fn delta(self, other: Self) -> Self {
        Vec2 {
            x: if self.x >= other.x {
                self.x - other.x
            } else {
                other.x - self.x
            },
            y: if self.y >= other.y {
                self.y - other.y
            } else {
                other.y - self.y
            },
        }
    }
}

pub auto trait IsNotVec2 {}
impl<T> !IsNotVec2 for Vec2<T> {}
pub auto trait NotEq {}
impl<X> !NotEq for (X, X) {}

impl<T> From<T> for Vec2<T>
where
    T: Clone + IsNotVec2,
{
    fn from(value: T) -> Self {
        Vec2 {
            x: value.clone(),
            y: value,
        }
    }
}
impl<T> From<(T, T)> for Vec2<T> {
    fn from(value: (T, T)) -> Self {
        Vec2 {
            x: value.0,
            y: value.1,
        }
    }
}

impl<T, U> From<Vec2<U>> for Vec2<T>
where
    T: From<U>,
    (T, U): NotEq,
{
    fn from(value: Vec2<U>) -> Self {
        Vec2 {
            x: T::from(value.x),
            y: T::from(value.y),
        }
    }
}

impl<T, U> TryFrom<Vec2<U>> for Vec2<T>
where
    T: TryFrom<U>,
    (T, U): NotEq,
{
    type Error = <T as TryFrom<U>>::Error;
    fn try_from(value: Vec2<U>) -> Result<Vec2<T>, Self::Error> {
        Ok(Vec2 {
            x: T::try_from(value.x)?,
            y: T::try_from(value.y)?,
        })
    }
}

impl<T> Vec2<T>
where
    T: Mul,
    T::Output: Add,
{
    pub fn dot(self, other: Self) -> <T::Output as Add>::Output {
        self.x * other.x + self.y * other.y
    }
}

macro_rules! impl_op {
    ($trait:ident, $fn:ident, $assign_trait:ident, $assign_fn:ident) => {
        impl<T> $trait for Vec2<T>
        where
            T: $trait<Output = T>,
        {
            type Output = Self;
            fn $fn(self, other: Self) -> Self {
                Vec2 {
                    x: T::$fn(self.x, other.x),
                    y: T::$fn(self.y, other.y),
                }
            }
        }

        impl<T> $assign_trait for Vec2<T>
        where
            T: $assign_trait,
        {
            fn $assign_fn(&mut self, other: Self) {
                T::$assign_fn(&mut self.x, other.x);
                T::$assign_fn(&mut self.y, other.y);
            }
        }
    };
}

macro_rules! impl_scalar {
    ($trait:ident, $fn:ident, $assign_trait:ident, $assign_fn:ident) => {
        impl<T> $trait<T> for Vec2<T>
        where
            T: $trait<Output = T> + Clone,
        {
            type Output = Self;
            fn $fn(self, other: T) -> Self {
                Vec2 {
                    x: T::$fn(self.x, other.clone()),
                    y: T::$fn(self.y, other),
                }
            }
        }

        impl<T> $assign_trait<T> for Vec2<T>
        where
            T: $assign_trait + Clone,
        {
            fn $assign_fn(&mut self, other: T) {
                T::$assign_fn(&mut self.x, other.clone());
                T::$assign_fn(&mut self.y, other);
            }
        }
    };
}

impl_op!(Add, add, AddAssign, add_assign);
impl_op!(Sub, sub, SubAssign, sub_assign);
impl_op!(Mul, mul, MulAssign, mul_assign);
impl_op!(Div, div, DivAssign, div_assign);
impl_op!(Rem, rem, RemAssign, rem_assign);

impl_scalar!(Mul, mul, MulAssign, mul_assign);
impl_scalar!(Div, div, DivAssign, div_assign);
impl_scalar!(Rem, rem, RemAssign, rem_assign);

impl<T> Neg for Vec2<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self {
        Vec2 {
            x: T::neg(self.x),
            y: T::neg(self.y),
        }
    }
}

impl<T> Zero for Vec2<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Vec2 {
            x: T::zero(),
            y: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        T::is_zero(&self.x) && T::is_zero(&self.y)
    }
}

impl<T> One for Vec2<T>
where
    T: One,
{
    fn one() -> Self {
        Vec2 {
            x: T::one(),
            y: T::one(),
        }
    }
}

impl<T> Num for Vec2<T>
where
    T: Num,
{
    type FromStrRadixErr = T::FromStrRadixErr;
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        let mut iter = s.split(',');
        if let Some(value) = iter.next() {
            let x = T::from_str_radix(value.trim(), radix)?;
            if let Some(value) = iter.next() {
                let y = T::from_str_radix(value.trim(), radix)?;
                if iter.next().is_none() {
                    return Ok(Vec2 { x, y });
                }
            }
        }

        if let Err(e) = T::from_str_radix("", radix) {
            return Err(e);
        }
        unreachable!()
    }
}

impl<T> Signed for Vec2<T>
where
    T: Signed + Clone,
{
    fn abs(&self) -> Self {
        Vec2 {
            x: T::abs(&self.x),
            y: T::abs(&self.y),
        }
    }

    fn abs_sub(&self, other: &Self) -> Self {
        Vec2 {
            x: self.x.abs_sub(&other.x),
            y: self.y.abs_sub(&other.y),
        }
    }

    fn signum(&self) -> Self {
        Vec2 {
            x: T::signum(&self.x),
            y: T::signum(&self.y),
        }
    }

    fn is_positive(&self) -> bool {
        T::is_positive(&self.x) && T::is_positive(&self.y)
    }

    fn is_negative(&self) -> bool {
        T::is_negative(&self.x) && T::is_negative(&self.y)
    }
}

impl<T> Unsigned for Vec2<T> where T: Unsigned {}

impl<T> FromStr for Vec2<T>
where
    T: Num,
{
    type Err = <Self as Num>::FromStrRadixErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as Num>::from_str_radix(s, 10)
    }
}

impl<T> fmt::Display for Vec2<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IterState {
    X,
    Y,
    None,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2Iter<T> {
    x: Option<T>,
    y: Option<T>,
}

impl<T> IntoIterator for Vec2<T> {
    type Item = T;
    type IntoIter = Vec2Iter<T>;
    fn into_iter(self) -> Vec2Iter<T> {
        Vec2Iter {
            x: Some(self.x),
            y: Some(self.y),
        }
    }
}
impl<T> Iterator for Vec2Iter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let mut v = None;
        std::mem::swap(&mut v, &mut self.x);
        if let Some(v) = v {
            std::mem::swap(&mut self.x, &mut self.y);
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let l = self.len();
        (l, Some(l))
    }
}
impl<T> ExactSizeIterator for Vec2Iter<T> {
    #[inline]
    fn len(&self) -> usize {
        match (self.x.is_some(), self.y.is_some()) {
            (true, true) => 2,
            (true, false) => 1,
            (false, true) => 1,
            (false, false) => 0,
        }
    }
}
unsafe impl<T> TrustedLen for Vec2Iter<T> {}

pub trait AabbIteratorEx<T>: Iterator {
    fn aabb(self) -> Option<(Vec2<T>, Vec2<T>)>;
}

impl<T, V> AabbIteratorEx<V> for T
where
    T: Iterator<Item = Vec2<V>>,
    V: Ord + Clone,
{
    fn aabb(mut self) -> Option<(Vec2<V>, Vec2<V>)> {
        if let Some(first) = self.next() {
            let mut min = first.clone();
            let mut max = first;
            for next in self {
                match next.x.cmp(&min.x) {
                    Ordering::Less => min.x = next.x.clone(),
                    _ => {
                        if let Ordering::Greater = next.x.cmp(&max.x) {
                            max.x = next.x.clone();
                        }
                    }
                }
                match next.y.cmp(&min.y) {
                    Ordering::Less => min.y = next.y.clone(),
                    _ => {
                        if let Ordering::Greater = next.y.cmp(&max.y) {
                            max.y = next.y.clone();
                        }
                    }
                }
            }
            Some((min, max))
        } else {
            None
        }
    }
}
