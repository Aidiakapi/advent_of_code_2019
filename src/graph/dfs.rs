#![allow(dead_code)]
#[inline(always)]
pub fn dfs<N, FN, NI>(start: N, mut next: FN)
where
    FN: FnMut(N) -> NI,
    NI: IntoIterator<Item = N>,
{
    dfs_impl(start, &mut next);
}

fn dfs_impl<N, FN, NI>(current: N, next: &mut FN)
where
    FN: FnMut(N) -> NI,
    NI: IntoIterator<Item = N>,
{
    for successor in next(current) {
        dfs_impl(successor, next);
    }
}

pub fn dfs_no_recursion<N, FN, FD, NI>(start: N, mut next: FN, mut done: FD) -> Option<N>
where
    FN: FnMut(N) -> NI,
    FD: FnMut(&N) -> bool,
    NI: IntoIterator<Item = N>,
{
    if done(&start) {
        return Some(start);
    }

    let mut stack = Vec::new();
    stack.push(next(start).into_iter());

    while let Some(last) = stack.last_mut() {
        if let Some(next_node) = last.next() {
            if done(&next_node) {
                return Some(next_node);
            }

            stack.push(next(next_node).into_iter());
        } else {
            stack.pop();
        }
    }

    None
}
