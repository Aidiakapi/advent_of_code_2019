#![allow(dead_code)]
use crate::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;

#[inline(always)]
pub fn flood<N, FN, NI>(start: N, mut next: FN)
where
    N: Eq + Hash,
    FN: FnMut(&N) -> NI,
    NI: IntoIterator<Item = N>,
{
    let mut visited = HashMap::new();
    flood_impl(&mut visited, start, &mut next);
}

fn flood_impl<N, FN, NI>(visited: &mut HashMap<N, ()>, current: N, next: &mut FN)
where
    N: Eq + Hash,
    FN: FnMut(&N) -> NI,
    NI: IntoIterator<Item = N>,
{
    let entry = visited.entry(current);
    if let Entry::Occupied(_) = &entry {
        return;
    }
    let entry = entry.insert(());
    let current = entry.key();

    for successor in next(current) {
        flood_impl(visited, successor, next);
    }
}
