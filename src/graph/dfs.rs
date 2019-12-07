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
