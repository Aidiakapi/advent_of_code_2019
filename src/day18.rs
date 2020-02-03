use crate::direction::{Direction, MoveInDirection};
use crate::graph::{astar_once, AStar};
use crate::mat2::Mat2;
use crate::vec2::Vec2us;
use crate::HashMap;
use arrayvec::ArrayVec;
use num::{One, Zero};

module!(pt1: parse, pt2: parse);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct KeyState(u32);
impl KeyState {
    #[inline]
    fn new(key_count: u8) -> KeyState {
        assert!(key_count <= 32);
        KeyState(match key_count {
            32 => !0u32,
            v => (1u32 << v) - 1,
        })
    }

    #[inline]
    fn with_key(self, idx: u8) -> KeyState {
        debug_assert!(idx < 32);
        KeyState(self.0 & !(1u32 << idx))
    }
    #[inline]
    fn has_key(self, idx: u8) -> bool {
        debug_assert!(idx < 32);
        self.0 & (1u32 << idx) == 0
    }

    #[inline]
    fn keys_left(self) -> u32 {
        self.0.count_ones()
    }
    #[inline]
    fn collected_all(self) -> bool {
        self.0 == 0
    }

    #[inline]
    fn combine(self, other: KeyState) -> KeyState {
        KeyState(self.0 & other.0)
    }
}

fn find_entrance(layout: &Mat2<Cell>) -> Vec2us {
    layout
        .iter()
        .find(|(_, cell)| **cell == Cell::Entrance)
        .unwrap()
        .0
}

#[derive(Debug)]
struct Node {
    neighbors: ArrayVec<[(Vec2us, usize); 4]>,
    cell: Cell,
}

/// Turns the grid-based map layout into a graph, preserving only
/// interesting cells (endpoints, intersection, keys, doors).
/// It runs two passes. The first pass converts the grid into
/// nodes, but also creates nodes for corner pieces (nodes with
/// two neighbors in different directions).
/// The second pass removes these corners.
///
/// On my specific input, this reduces the walkable parts of the
/// maze from 6561 to 404 on pt1, and from 6561 to 399 for pt2.
fn grid_to_graph(layout: &Mat2<Cell>) -> HashMap<Vec2us, Node> {
    let mut nodes: HashMap<Vec2us, Node> = HashMap::new();
    let mut node_north: Option<Vec2us> = None;
    let mut nodes_west: Vec<Option<Vec2us>> = Vec::with_capacity(layout.height());
    nodes_west.resize(layout.height(), None);
    for x in 0..layout.width() {
        let column = &layout[x];
        for y in 0..layout.height() {
            let cell = column[y];
            if cell == Cell::Wall {
                debug_assert!(node_north.is_none());
                debug_assert!(nodes_west[y].is_none());
                continue;
            }
            let pos = Vec2us::new(x, y);
            // NSEW
            let neighbors = Direction::each_arr(|dir| {
                pos.move_in_bounds(dir, 1, Vec2us::zero(), layout.size() - Vec2us::one())
                    .map(|pos| match layout[pos] {
                        Cell::Wall => false,
                        _ => true,
                    })
                    .unwrap_or(false)
            });
            // A straight section with no branches doesn't need a node
            // except when it's a door, key, or the entrance.
            if match cell {
                Cell::Door(_) | Cell::Key(_) | Cell::Entrance => false,
                _ => true,
            } && neighbors[0] == neighbors[1]
                && neighbors[2] == neighbors[3]
                && neighbors[0] != neighbors[2]
            {
                continue;
            }
            let mut neighbor_positions = ArrayVec::new();
            if neighbors[0] {
                let other = node_north.unwrap();
                let dist = y - other.y;
                neighbor_positions.push((other, dist));
                nodes.get_mut(&other).unwrap().neighbors.push((pos, dist));
            }
            node_north = if neighbors[1] { Some(pos) } else { None };
            if neighbors[2] {
                let other = nodes_west[y].unwrap();
                let dist = x - other.x;
                neighbor_positions.push((other, dist));
                nodes.get_mut(&other).unwrap().neighbors.push((pos, dist));
            }
            nodes_west[y] = if neighbors[3] { Some(pos) } else { None };
            nodes.insert(
                pos,
                Node {
                    neighbors: neighbor_positions,
                    cell,
                },
            );
        }
    }

    // Simplify further by removing nodes with exactly 2 neighbors,
    // unless they are a special node.
    for point in nodes.keys().cloned().collect::<Vec<_>>() {
        let node = nodes.get_mut(&point).unwrap();
        if node.cell != Cell::Open || node.neighbors.len() != 2 {
            continue;
        }
        let dist = node.neighbors[0].1 + node.neighbors[1].1;
        let a = node.neighbors[0].0;
        let b = node.neighbors[1].0;
        nodes.remove(&point);

        for (a, b) in [(a, b), (b, a)].iter().cloned() {
            let n = nodes.get_mut(&a).unwrap();
            for neighbor in &mut n.neighbors {
                if neighbor.0 == point {
                    neighbor.0 = b;
                    neighbor.1 = dist;
                }
            }
        }
    }

    nodes
}

fn next<'l>(
    layout: &'l HashMap<Vec2us, Node>,
    &(pos, key_state): &(Vec2us, KeyState),
) -> impl Iterator<Item = ((Vec2us, KeyState), usize)> + 'l {
    layout[&pos]
        .neighbors
        .iter()
        .filter_map(move |&(next_pos, dist)| match layout[&next_pos].cell {
            Cell::Wall => None,
            Cell::Entrance | Cell::Open => Some(((next_pos, key_state), dist)),
            Cell::Door(idx) => {
                if key_state.has_key(idx) {
                    Some(((next_pos, key_state), dist))
                } else {
                    None
                }
            }
            Cell::Key(idx) => Some(((next_pos, key_state.with_key(idx)), dist)),
        })
}
fn heuristic((_, key_state): &(Vec2us, KeyState)) -> usize {
    key_state.keys_left() as usize
}
fn collected_all((_, key_state): &(Vec2us, KeyState)) -> bool {
    key_state.collected_all()
}

fn pt1(map: Map) -> Result<usize> {
    let Map { layout, key_count } = map;
    let entrance = find_entrance(&layout);
    if key_count > 32 {
        return Err(AoCError::Logic("cannot handle more than 32 keys"));
    }
    let layout = grid_to_graph(&layout);
    let path = astar_once(
        (entrance, KeyState::new(key_count)),
        |s| next(&layout, s),
        heuristic,
        collected_all,
    )
    .ok_or(AoCError::NoSolution)?;
    Ok(path.last().unwrap().1)
}

/// The solution relies on an assumption that the optimal path in each
/// quadrant is not blocked by any of the keys in any of the other
/// quadrants. This is true for my input, and will probably be true
/// for all inputs, but it is an assumption.
fn pt2(map: Map) -> Result<usize> {
    let Map {
        mut layout,
        key_count,
    } = map;
    let entrance = find_entrance(&layout);
    if key_count > 32 {
        return Err(AoCError::Logic("cannot handle more than 32 keys"));
    }

    // Update the map
    let entrances = [
        Vec2us::new(entrance.x - 1, entrance.y - 1),
        Vec2us::new(entrance.x - 1, entrance.y + 1),
        Vec2us::new(entrance.x + 1, entrance.y - 1),
        Vec2us::new(entrance.x + 1, entrance.y + 1),
    ];
    for x in entrance.x - 1..=entrance.x + 1 {
        for y in entrance.y - 1..=entrance.y + 1 {
            layout[x][y] = Cell::Wall;
        }
    }
    for &entrance in entrances.iter() {
        layout[entrance] = Cell::Entrance;
    }

    let layout = grid_to_graph(&layout);
    let mut keys_after_quadrant = [KeyState::new(key_count); 4];
    layout
        .iter()
        .filter_map(|(pos, node)| {
            if let Cell::Key(idx) = node.cell {
                Some((pos, idx))
            } else {
                None
            }
        })
        .for_each(|(pos, idx)| {
            let keys = &mut keys_after_quadrant[match (pos.x > entrance.x, pos.y > entrance.y) {
                (false, false) => 0,
                (false, true) => 1,
                (true, false) => 2,
                (true, true) => 3,
            }];
            *keys = keys.with_key(idx);
        });

    let mut astar = AStar::new();
    let mut total_cost = 0;
    for (entrance_idx, entrance) in entrances.iter().cloned().enumerate() {
        let mut start_state = KeyState::new(key_count);
        keys_after_quadrant
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx != entrance_idx)
            .for_each(|(_, keys)| start_state = start_state.combine(*keys));
        let path = astar
            .solve(
                (entrance, start_state),
                |s| next(&layout, s),
                heuristic,
                collected_all,
            )
            .ok_or(AoCError::NoSolution)?;
        total_cost += path.last().unwrap().1;
    }

    Ok(total_cost)
}

fn parse(s: &str) -> IResult<&str, Map> {
    use parsers::*;
    fn cell(s: &str) -> IResult<&str, Cell> {
        let c = s.chars().next().ok_or(Err::Error((s, ErrorKind::Eof)))?;
        Ok((
            &s[1..],
            match c {
                'A'..='Z' => Cell::Door(c as u8 - b'A'),
                'a'..='z' => Cell::Key(c as u8 - b'a'),
                '.' => Cell::Open,
                '#' => Cell::Wall,
                '@' => Cell::Entrance,
                _ => return Err(Err::Error((s, ErrorKind::OneOf))),
            },
        ))
    }

    map_res(
        terminated(many1(terminated(many1(cell), line_ending_or_eof)), eof),
        |maze| {
            let size = Vec2us::new(maze[0].len(), maze.len());
            if maze[1..].iter().any(|row| row.len() != size.x) {
                return Err(AoCError::IncorrectInput(
                    "cannot have different line widths",
                ));
            }
            let mut cells = Mat2::new(Cell::Wall, size);
            for (y, row) in maze.into_iter().enumerate() {
                for (x, cell) in row.into_iter().enumerate() {
                    cells[Vec2us::new(x, y)] = cell;
                }
            }
            let mut door_count = 0;
            let mut doors = Vec::new();
            let mut has_entrance = false;
            for cell in &cells.data {
                match cell {
                    Cell::Door(v) => {
                        let v = *v as usize;
                        if v >= doors.len() {
                            doors.resize(v + 1, false);
                        }
                        if doors[v] {
                            return Err(AoCError::IncorrectInput(
                                "multiple doors for the same key",
                            ));
                        }
                        door_count += 1;
                        doors[v] = true;
                    }
                    Cell::Entrance => {
                        if has_entrance {
                            return Err(AoCError::IncorrectInput("multiple entrances"));
                        }
                        has_entrance = true;
                    }
                    _ => {}
                }
            }
            if !has_entrance {
                return Err(AoCError::IncorrectInput("no entrance in input"));
            }
            if door_count == 0 {
                return Err(AoCError::IncorrectInput("no keys/doors in input"));
            }
            if door_count != doors.len() {
                return Err(AoCError::IncorrectInput("non-consecutive door numbers"));
            }
            let mut key_count = 0;
            for cell in &cells.data {
                if let Cell::Key(v) = cell {
                    let v = *v as usize;
                    if v >= doors.len() || !doors[v] {
                        return Err(AoCError::IncorrectInput(
                            "multiple keys for door or key without door",
                        ));
                    }
                    doors[v] = false;
                    key_count += 1;
                }
            }
            if door_count != key_count {
                return Err(AoCError::IncorrectInput("not every door has a key"));
            }

            Ok(Map {
                layout: cells,
                key_count: key_count as u8,
            })
        },
    )(s)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    layout: Mat2<Cell>,
    key_count: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Door(u8),
    Key(u8),
    Open,
    Wall,
    Entrance,
}

#[test]
fn day18() -> Result<()> {
    assert_eq!(
        8,
        pt1(parse(
            "\
#########
#b.A.@.a#
B########"
        )?
        .1)?
    );

    Ok(())
}
