#![allow(dead_code)]

use crate::HashMap;
use num::traits::Zero;
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::collections::BinaryHeap;
use std::hash::Hash;
use std::ops::Add;

pub trait Node = Clone + Eq + Hash;
pub trait Cost = Clone + Ord + Add + Zero;

#[derive(Debug, Clone)]
pub struct AStar<N: Node, C: Cost> {
    meta: HashMap<N, Meta<N, C>>,
    open: BinaryHeap<Open<N, C>>,
    path: Vec<(N, C)>,
}

#[derive(Debug, Clone)]
struct Meta<N: Node, C: Cost> {
    is_closed: bool,
    heuristic: C,
    path: C,
    parent: Option<N>,
}

#[derive(Debug, Clone, Eq)]
struct Open<N: Node, C: Cost> {
    cost: C,
    node: N,
    counter: usize,
}

impl<N: Node, C: Cost> PartialEq for Open<N, C> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<N: Node, C: Cost> PartialOrd for Open<N, C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<N: Node, C: Cost> Ord for Open<N, C> {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then(self.counter.cmp(&other.counter))
    }
}

impl<N: Node, C: Cost> AStar<N, C> {
    pub fn new() -> Self {
        AStar {
            meta: HashMap::new(),
            open: BinaryHeap::new(),
            path: Vec::new(),
        }
    }

    pub fn into_last_path(self) -> Vec<(N, C)> {
        self.path
    }

    pub fn solve<FN, FH, FD, NI>(
        &mut self,
        init: N,
        mut next: FN,
        mut heuristic: FH,
        mut is_done: FD,
    ) -> Option<&Vec<(N, C)>>
    where
        FN: FnMut(&N) -> NI,
        FH: FnMut(&N) -> C,
        FD: FnMut(&N) -> bool,
        NI: IntoIterator<Item = (N, C)>,
    {
        // Used to get FIFO behaviour from the open set
        let mut counter = 0;
        self.path.clear();
        let init_heuristic = heuristic(&init);
        let init_meta = Meta {
            is_closed: false,
            path: C::zero(),
            heuristic: init_heuristic.clone(),
            parent: None,
        };
        self.meta.insert(init.clone(), init_meta);
        let init_open = Open {
            node: init,
            cost: init_heuristic,
            counter,
        };
        self.open.push(init_open);

        while let Some(open) = self.open.pop() {
            let meta = self.meta.get_mut(&open.node).unwrap();
            // This can happen if the same node was inserted multiple times into the
            // open set, because a later found route to the same node actually had a
            // shorter total length.
            if meta.is_closed {
                continue;
            }
            meta.is_closed = true;

            if is_done(&open.node) {
                // Reconstruct the path
                let mut current_node = Some(&open.node);
                while let Some(n) = current_node {
                    let meta = &self.meta[&n];
                    self.path.push((n.clone(), meta.path.clone()));
                    current_node = meta.parent.as_ref();
                }

                self.path.reverse();

                self.open.clear();
                self.meta.clear();
                return Some(&self.path);
            }
            let path_cost = meta.path.clone();
            for (node, edge_cost) in next(&open.node) {
                let cost = match self.meta.get_mut(&node) {
                    Some(meta) => {
                        // If the node was already seen, and is in closed,
                        // the shortest route is already established, and
                        // there is no need to revisit the node.
                        if meta.is_closed {
                            continue;
                        }
                        // If the other node is already in the open set
                        // but the cost through this parent node is cheaper
                        // it has to be updated.
                        let path_cost = edge_cost + path_cost.clone();
                        if meta.path <= path_cost {
                            continue;
                        }
                        // Update price
                        meta.path = path_cost.clone();
                        meta.parent = Some(open.node.clone());
                        path_cost
                    }
                    // New node
                    None => {
                        let path_cost = edge_cost + path_cost.clone();
                        let heuristic_cost = heuristic(&node);
                        self.meta.insert(
                            node.clone(),
                            Meta {
                                is_closed: false,
                                path: path_cost.clone(),
                                heuristic: heuristic_cost.clone(),
                                parent: Some(open.node.clone()),
                            },
                        );
                        path_cost + heuristic_cost
                    }
                };
                counter += 1;
                self.open.push(Open {
                    node: node,
                    cost,
                    counter,
                });
            }
        }

        self.open.clear();
        self.meta.clear();
        None
    }
}

pub fn astar_once<N, C, FN, FH, FD, NI>(
    init: N,
    next: FN,
    heuristic: FH,
    is_done: FD,
) -> Option<Vec<(N, C)>>
where
    N: Node,
    C: Cost,
    FN: FnMut(&N) -> NI,
    FH: FnMut(&N) -> C,
    FD: FnMut(&N) -> bool,
    NI: IntoIterator<Item = (N, C)>,
{
    let mut state = AStar::new();
    if state.solve(init, next, heuristic, is_done).is_some() {
        Some(state.into_last_path())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{mat2::Mat2, vec2::Vec2us};
    use arrayvec::ArrayVec;
    const TEST_FILE: &'static str = include_str!("astar_tests.txt");

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestCase {
        name: &'static str,
        maze: Mat2<bool>,
        start: Vec2us,
        end: Vec2us,
        path_length: Option<usize>,
    }

    fn test_cases() -> Vec<TestCase> {
        use crate::parsers::*;
        use nom::{bytes::complete::take_while, combinator::all_consuming};

        fn maze_char1(s: &str) -> IResult<&str, &str> {
            let mut matched_count = 0;
            let mut chars = s.chars();
            while let Some(c) = chars.next() {
                if c == '#' || c == '.' || c == 'S' || c == 'E' {
                    matched_count += 1;
                } else {
                    break;
                }
            }

            if matched_count == 0 {
                Err(nom::Err::Error((s, ErrorKind::Many1)))
            } else {
                Ok((&s[matched_count..], &s[0..matched_count]))
            }
        }

        let maze_meta = pair(
            opt(delimited(char('['), usize_str, tag("] "))),
            terminated(take_while(|c: char| c != '\r' && c != '\n'), line_ending),
        );
        let maze_grid = map_res(
            separated_list(line_ending, maze_char1),
            |lines: Vec<&str>| {
                let linelen = lines[0].len();
                if !lines.iter().skip(1).all(|line| line.len() == linelen) {
                    return Err(Err::Error((lines, ErrorKind::Verify)));
                }
                Ok(lines)
            },
        );
        let maze = map_res(
            pair(maze_meta, maze_grid),
            |((path_length, name), lines)| {
                let mut start = None;
                let mut end = None;
                let mut grid = Mat2::new(false, (lines[0].len(), lines.len()).into());
                for (y, line) in lines.iter().enumerate() {
                    for (x, c) in line.chars().enumerate() {
                        let pos = Vec2us::new(x, y);
                        match c {
                            '#' => grid[pos] = true,
                            'S' => {
                                if start.is_none() {
                                    start = Some(pos);
                                } else {
                                    return Err(Err::Failure(ErrorKind::Verify));
                                }
                            }
                            'E' => {
                                if end.is_none() {
                                    end = Some(pos);
                                } else {
                                    return Err(Err::Failure(ErrorKind::Verify));
                                }
                            }
                            _ => {}
                        }
                    }
                }

                if start.is_none() || end.is_none() {
                    return Err(Err::Failure(ErrorKind::Verify));
                }
                Ok(TestCase {
                    path_length,
                    name,
                    maze: grid,
                    start: start.unwrap(),
                    end: end.unwrap(),
                })
            },
        );
        all_consuming(terminated(
            separated_list(many1(line_ending), maze),
            many0(line_ending),
        ))(TEST_FILE)
        .unwrap()
        .1
    }

    #[test]
    fn astar_test_file() {
        let mut astar = AStar::new();
        for TestCase {
            name,
            maze,
            start,
            end,
            path_length,
        } in test_cases()
        {
            println!("Pathfinding: {}", name);
            let solution = astar.solve(
                start,
                |&pos| {
                    let mut next: ArrayVec<[Vec2us; 4]> = ArrayVec::new();

                    if pos.x > 0 && !maze[pos.x - 1][pos.y] {
                        next.push((pos.x - 1, pos.y).into());
                    }
                    if pos.x < maze.width() - 1 && !maze[pos.x + 1][pos.y] {
                        next.push((pos.x + 1, pos.y).into());
                    }

                    if pos.y > 0 && !maze[pos.x][pos.y - 1] {
                        next.push((pos.x, pos.y - 1).into());
                    }
                    if pos.y < maze.height() - 1 && !maze[pos.x][pos.y + 1] {
                        next.push((pos.x, pos.y + 1).into());
                    }

                    next.into_iter().map(|n| (n, 1))
                },
                |&pos| {
                    let delta = end.delta(pos);
                    delta.x + delta.y
                },
                |&pos| pos == end,
            );
            if let Some(path) = solution {
                println!("Found path of length: {}", path.last().unwrap().1);
            } else {
                println!("No path found");
            }
            let found_path_length = solution.map(|path| path.last().unwrap().1);
            assert_eq!(path_length, found_path_length);
        }
    }
}
