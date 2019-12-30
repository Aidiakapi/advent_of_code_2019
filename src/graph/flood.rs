#![allow(dead_code)]
use crate::HashSet;
use std::hash::Hash;

#[inline(always)]
pub fn flood<N, FN, NI>(start: N, mut next: FN)
where
    N: Eq + Hash + Clone, // TODO: Remove requirement for clone
    FN: FnMut(&N) -> NI,
    NI: IntoIterator<Item = N>,
{
    let mut visited = HashSet::new();
    flood_impl(&mut visited, start, &mut next);
}

fn flood_impl<N, FN, NI>(visited: &mut HashSet<N>, current: N, next: &mut FN)
where
    N: Eq + Hash + Clone, // TODO: Remove requirement for clone
    FN: FnMut(&N) -> NI,
    NI: IntoIterator<Item = N>,
{
    if !visited.insert(current.clone()) {
        return;
    }
    let current = visited.get(&current).unwrap();

    for successor in next(current) {
        flood_impl(visited, successor, next);
    }
}
