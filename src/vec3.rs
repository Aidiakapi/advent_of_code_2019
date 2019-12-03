#![allow(dead_code)]

use num::traits::{
    identities::{One, Zero},
    sign::{Signed, Unsigned},
    Num,
};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use std::str::FromStr;

pub type Vec3i = Vec3<i32>;
pub type Vec3us = Vec3<usize>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Hash)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    #[inline(always)]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }

    #[inline(always)]
    pub fn with_x(self, x: T) -> Self {
        Vec3 {
            x,
            y: self.y,
            z: self.z,
        }
    }
    #[inline(always)]
    pub fn with_y(self, y: T) -> Self {
        Vec3 {
            x: self.x,
            y,
            z: self.z,
        }
    }
    #[inline(always)]
    pub fn with_z(self, z: T) -> Self {
        Vec3 {
            x: self.x,
            y: self.y,
            z,
        }
    }

    #[inline(always)]
    pub fn all<F>(&self, other: &Self, mut f: F) -> bool
    where
        F: FnMut(&T, &T) -> bool,
    {
        f(&self.x, &other.x) && f(&self.y, &other.y) && f(&self.z, &other.z)
    }

    #[inline(always)]
    pub fn any<F>(&self, other: &Self, mut f: F) -> bool
    where
        F: FnMut(&T, &T) -> bool,
    {
        f(&self.x, &other.x) || f(&self.y, &other.y) || f(&self.z, &other.z)
    }
}

impl<T> From<T> for Vec3<T>
where
    T: Clone,
{
    fn from(value: T) -> Self {
        Vec3 {
            x: value.clone(),
            y: value.clone(),
            z: value,
        }
    }
}
impl<T> From<(T, T, T)> for Vec3<T>
where
    T: Clone,
{
    fn from(value: (T, T, T)) -> Self {
        Vec3 {
            x: value.0.clone(),
            y: value.1,
            z: value.2,
        }
    }
}

macro_rules! impl_op {
    ($trait:ident, $fn:ident, $assign_trait:ident, $assign_fn:ident) => {
        impl<T> $trait for Vec3<T>
        where
            T: $trait<Output = T>,
        {
            type Output = Self;
            fn $fn(self, other: Self) -> Self {
                Vec3 {
                    x: T::$fn(self.x, other.x),
                    y: T::$fn(self.y, other.y),
                    z: T::$fn(self.z, other.z),
                }
            }
        }

        impl<T> $assign_trait for Vec3<T>
        where
            T: $assign_trait,
        {
            fn $assign_fn(&mut self, other: Self) {
                T::$assign_fn(&mut self.x, other.x);
                T::$assign_fn(&mut self.y, other.y);
                T::$assign_fn(&mut self.z, other.z);
            }
        }
    };
}

macro_rules! impl_scalar {
    ($trait:ident, $fn:ident, $assign_trait:ident, $assign_fn:ident) => {
        impl<T> $trait<T> for Vec3<T>
        where
            T: $trait<Output = T> + Clone,
        {
            type Output = Self;
            fn $fn(self, other: T) -> Self {
                Vec3 {
                    x: T::$fn(self.x, other.clone()),
                    y: T::$fn(self.y, other.clone()),
                    z: T::$fn(self.z, other),
                }
            }
        }

        impl<T> $assign_trait<T> for Vec3<T>
        where
            T: $assign_trait + Clone,
        {
            fn $assign_fn(&mut self, other: T) {
                T::$assign_fn(&mut self.x, other.clone());
                T::$assign_fn(&mut self.y, other.clone());
                T::$assign_fn(&mut self.z, other);
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

impl<T> Neg for Vec3<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self {
        Vec3 {
            x: T::neg(self.x),
            y: T::neg(self.y),
            z: T::neg(self.z),
        }
    }
}

impl<T> Zero for Vec3<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Vec3 {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        T::is_zero(&self.x) && T::is_zero(&self.y) && T::is_zero(&self.z)
    }
}

impl<T> One for Vec3<T>
where
    T: One,
{
    fn one() -> Self {
        Vec3 {
            x: T::one(),
            y: T::one(),
            z: T::one(),
        }
    }
}

impl<T> Num for Vec3<T>
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
                if let Some(value) = iter.next() {
                    let z = T::from_str_radix(value.trim(), radix)?;
                    if let None = iter.next() {
                        return Ok(Vec3 { x, y, z });
                    }
                }
            }
        }

        if let Err(e) = T::from_str_radix("", radix) {
            return Err(e);
        }
        unreachable!()
    }
}

impl<T> Signed for Vec3<T>
where
    T: Signed + Clone,
{
    fn abs(&self) -> Self {
        Vec3 {
            x: T::abs(&self.x),
            y: T::abs(&self.y),
            z: T::abs(&self.z),
        }
    }

    fn abs_sub(&self, other: &Self) -> Self {
        let delta = self.clone() - other.clone();
        <Self as Signed>::abs(&delta)
    }

    fn signum(&self) -> Self {
        Vec3 {
            x: T::signum(&self.x),
            y: T::signum(&self.y),
            z: T::signum(&self.z),
        }
    }

    fn is_positive(&self) -> bool {
        T::is_positive(&self.x) && T::is_positive(&self.y) && T::is_positive(&self.z)
    }

    fn is_negative(&self) -> bool {
        T::is_negative(&self.x) && T::is_negative(&self.y) && T::is_negative(&self.z)
    }
}

impl<T> Unsigned for Vec3<T> where T: Unsigned {}

impl<T> FromStr for Vec3<T>
where
    T: Num,
{
    type Err = <Self as Num>::FromStrRadixErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as Num>::from_str_radix(s, 10)
    }
}

impl<T> fmt::Display for Vec3<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

pub trait AabbIteratorEx<T>: Iterator {
    fn aabb(self) -> Option<(Vec3<T>, Vec3<T>)>;
}

impl<T, V> AabbIteratorEx<V> for T
where
    T: Iterator<Item = Vec3<V>>,
    V: Ord + Clone,
{
    fn aabb(mut self) -> Option<(Vec3<V>, Vec3<V>)> {
        if let Some(first) = self.next() {
            let mut min = first.clone();
            let mut max = first;
            while let Some(next) = self.next() {
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
                match next.z.cmp(&min.z) {
                    Ordering::Less => min.z = next.z.clone(),
                    _ => {
                        if let Ordering::Greater = next.z.cmp(&max.z) {
                            max.z = next.z.clone();
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
