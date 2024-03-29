#![feature(
    auto_traits,
    drain_filter,
    entry_insert,
    hash_set_entry,
    is_sorted,
    iter_intersperse,
    negative_impls,
    stmt_expr_attributes,
    test,
    trait_alias,
    trusted_len
)]

extern crate test;

pub(crate) mod direction;
pub(crate) mod error;
pub(crate) mod graph;
pub(crate) mod intcode;
pub(crate) mod mat2;
#[macro_use]
pub(crate) mod module;
pub(crate) mod parsers;
pub(crate) mod vec2;
pub(crate) mod vec3;

#[allow(dead_code)]
type Atom = string_cache::DefaultAtom;
#[allow(dead_code)]
type HashMap<K, V> = ahash::AHashMap<K, V>;
#[allow(dead_code)]
type HashSet<T> = ahash::AHashSet<T>;

generate_main!(
    day01
    day02
    day03
    day04
    day05
    day06
    day07
    day08
    day09
    day10
    day11
    day12
    day13
    day14
    day15
    day16
    day17
    day18
    day19
    day20
    day21
    day22
    day23
    day24
    day25
);
