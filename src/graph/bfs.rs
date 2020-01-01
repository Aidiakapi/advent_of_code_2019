#![allow(dead_code)]
use crate::HashMap;
use std::collections::{hash_map::Entry, VecDeque};
use std::hash::Hash;

#[inline(always)]
pub fn bfs<N, FN, FD, NI>(start: N, mut next: FN, mut done: FD)
where
    N: Hash + Eq,
    FN: FnMut(&N) -> NI,
    FD: FnMut(&N) -> bool,
    NI: IntoIterator<Item = N>,
{
    bfs_meta(
        (start, ()),
        |node, ()| next(node).into_iter().map(|n| (n, ())),
        |node, ()| done(node),
    )
}

pub fn bfs_meta<N, M, FN, FD, NI>(start: (N, M), mut next: FN, mut done: FD)
where
    N: Hash + Eq,
    FN: FnMut(&N, &M) -> NI,
    FD: FnMut(&N, &M) -> bool,
    NI: IntoIterator<Item = (N, M)>,
{
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();
    let mut current = start;
    loop {
        // Insert item if new
        let entry = visited.entry(current.0);
        if let Entry::Occupied(_) = entry {
        } else {
            let entry = entry.insert(current.1);
            let node = entry.key();
            let meta = entry.get();
            if done(node, meta) {
                return;
            }
            queue.extend(next(node, meta));
        }

        // Next item
        if let Some(next) = queue.pop_front() {
            current = next;
        } else {
            break;
        }
    }
}
