#![feature(optin_builtin_traits, test)]

extern crate test;

pub(crate) mod error;
pub(crate) mod intcode;
pub(crate) mod mat2;
#[macro_use]
pub(crate) mod module;
pub(crate) mod parsers;
pub(crate) mod vec2;
pub(crate) mod vec3;

generate_main!(
    // day01
    // day02
    // day03
    // day04
    // day05
    day06
);
