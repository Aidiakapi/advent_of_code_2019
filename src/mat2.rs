// Matrix 2D backed by a vector
#![allow(dead_code)]
use crate::vec2::Vec2us;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Mat2<T: Clone> {
    pub data: Vec<T>,
    size: Vec2us,
}

impl<T: Clone> Mat2<T> {
    pub fn new(item: T, size: Vec2us) -> Self {
        Mat2 {
            data: vec![item; size.x * size.y],
            size,
        }
    }

    #[rustfmt::skip] #[inline(always)] pub fn size(&self) -> Vec2us { self.size }
    #[rustfmt::skip] #[inline(always)] pub fn width(&self) -> usize { self.size.x }
    #[rustfmt::skip] #[inline(always)] pub fn height(&self) -> usize { self.size.y }

    #[rustfmt::skip] #[inline(always)] pub fn iter<'s>(&'s self) -> Mat2Iter<'s, T> { self.into_iter() }
    #[rustfmt::skip] #[inline(always)] pub fn iter_mut<'s>(&'s mut self) -> Mat2IterMut<'s, T> { self.into_iter() }
}

impl<T: Clone> Index<usize> for Mat2<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &[T] {
        assert!(index < self.size.x);
        let base = index * self.size.y;
        &self.data[base..base + self.size.y]
    }
}

impl<T: Clone> IndexMut<usize> for Mat2<T> {
    fn index_mut(&mut self, index: usize) -> &mut [T] {
        assert!(index < self.size.x);
        let base = index * self.size.y;
        &mut self.data[base..base + self.size.y]
    }
}

impl<T: Clone> Index<Vec2us> for Mat2<T> {
    type Output = T;

    fn index(&self, index: Vec2us) -> &T {
        assert!(index.x < self.size.x);
        &self.data[index.x * self.size.y + index.y]
    }
}

impl<T: Clone> IndexMut<Vec2us> for Mat2<T> {
    fn index_mut(&mut self, index: Vec2us) -> &mut T {
        assert!(index.x < self.size.x);
        &mut self.data[index.x * self.size.y + index.y]
    }
}

macro_rules! implement_iterator {
    (
        $name:ident,
        derive[$($derive:ident),*$(,)?],
        generic_params[$($generic_params:tt)+],
        constraints[$($constraints:tt)*],
        base_iter[$($base_iter:tt)+],
        item[$($item:tt)+],
        into_iter_for[$($into_iter_for:tt)+],
        into_iter_fn[$($into_iter_fn:tt)+],$(,)?
    ) => {
        #[allow(unused_attributes)]
        #[derive($($derive),*)]
        pub struct $name $($generic_params)+ ($($base_iter)+, usize, usize) $($constraints)*;
        impl $($generic_params)+ Iterator for $name $($generic_params)+ $($constraints)* {
            type Item = (Vec2us, $($item)+);

            fn next(&mut self) -> Option<Self::Item> {
                let data = self.0.next()?;
                let pos = Vec2us::new(self.1 / self.2, self.1 % self.2);
                self.1 += 1;
                Some((pos, data))
            }

            #[inline(always)]
            fn size_hint(&self) -> (usize, Option<usize>) {
                self.0.size_hint()
            }
        }

        impl $($generic_params)+ ExactSizeIterator for $name $($generic_params)+ $($constraints)*  {
            #[inline(always)]
            fn len(&self) -> usize {
                self.0.len()
            }
        }

        impl $($generic_params)+ IntoIterator for $($into_iter_for)+ $($constraints)* {
            type IntoIter = $name $($generic_params)+;
            type Item = <Self::IntoIter as Iterator>::Item;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                $name(self.data.$($into_iter_fn)+(), 0, self.size.y)
            }
        }
    };
}

implement_iterator!(
    Mat2IntoIter,
    derive[Clone],
    generic_params[<T>],
    constraints[where T: Clone],
    base_iter[std::vec::IntoIter<T>],
    item[T],
    into_iter_for[Mat2<T>],
    into_iter_fn[into_iter],
);
implement_iterator!(
    Mat2Iter,
    derive[Clone],
    generic_params[<'a, T>],
    constraints[where T: Clone],
    base_iter[std::slice::Iter<'a, T>],
    item[&'a T],
    into_iter_for[&'a Mat2<T>],
    into_iter_fn[iter],
);
implement_iterator!(
    Mat2IterMut,
    derive[],
    generic_params[<'a, T>],
    constraints[where T: Clone],
    base_iter[std::slice::IterMut<'a, T>],
    item[&'a mut T],
    into_iter_for[&'a mut Mat2<T>],
    into_iter_fn[iter_mut],
);
