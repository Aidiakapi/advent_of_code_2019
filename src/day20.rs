use crate::direction::{Direction, MoveInDirection};
use crate::graph::bfs_meta;
use crate::vec2::{AabbIteratorEx, Vec2us};
use crate::HashMap;
use std::collections::hash_map::Entry;
use std::fmt;
use std::hash::{Hash, Hasher};

module!(pt1: parse, pt2: parse);

fn pt1(map: Map) -> Result<usize> {
    let mut res = None;
    bfs_meta(
        (map.entrance, 0),
        |&pos, &dist| {
            Direction::each()
                .filter_map(move |dir| pos.step_in_direction_checked(dir))
                .filter(|pos| match map.layout.get(&pos) {
                    Some(Cell::Wall) => false,
                    None => false,
                    Some(_) => true,
                })
                .chain(std::iter::once(map.tps.get(&pos).cloned()).filter_map(|x| x))
                .map(move |pos| (pos, dist + 1))
        },
        |&pos, &dist| {
            if pos == map.exit {
                res = Some(dist);
                true
            } else {
                false
            }
        },
    );
    res.ok_or(AoCError::NoSolution)
}

fn pt2(map: Map) -> Result<usize> {
    let (min, max) = map.layout.keys().cloned().aabb().unwrap();

    let is_outer_teleport =  move |pos: Vec2us| -> bool {
        pos.x == min.x || pos.y == min.y || pos.x == max.x || pos.y == max.y
    };
    for (&from, &to) in &map.tps {
        if is_outer_teleport(from) == is_outer_teleport(to) {
            return Err(AoCError::IncorrectInput("teleport points do not lie on border and inner side"));
        }
    }
    
    let mut res = None;
    bfs_meta(
        ((map.entrance, 0), 0),
        |&(pos, layer), &dist| {
            let tp_to_pos = map.tps.get(&pos).cloned().and_then(|p| {
                if is_outer_teleport(pos) {
                    if layer == 0 {
                        None
                    } else {
                        Some((p, layer - 1))
                    }
                } else {
                    Some((p, layer + 1))
                }
            });

            Direction::each()
                .filter_map(move |dir| pos.step_in_direction_checked(dir))
                .filter(|pos| match map.layout.get(&pos) {
                    Some(Cell::Wall) => false,
                    None => false,
                    Some(_) => true,
                })
                .map(move |pos| (pos, layer))
                .chain(std::iter::once(tp_to_pos).filter_map(|x| x))
                .map(move |n| (n, dist + 1))
        },
        |&(pos, layer), &dist| {
            if layer == 0 && pos == map.exit {
                res = Some(dist);
                true
            } else {
                false
            }
        },
    );
    res.ok_or(AoCError::NoSolution)
}

fn parse(s: &str) -> Result<Map> {
    let mut tp_chars = Vec::new();
    let mut layout = HashMap::new();
    for (y, line) in s.as_bytes().split(|&c| c == b'\n').enumerate() {
        for (x, c) in line.iter().cloned().enumerate() {
            let pos = Vec2us::new(x, y);
            match c {
                b' ' => {}
                b'#' => layout.insert(pos, Cell::Wall).unwrap_none(),
                b'.' => layout.insert(pos, Cell::Open).unwrap_none(),
                b'A'..=b'Z' => tp_chars.push((pos, c)),
                _ => return Err(AoCError::IncorrectInput("unexpected char in input")),
            }
        }
    }

    tp_chars.sort_unstable_by_key(|(pos, _)| *pos);

    let mut tp_points = HashMap::new();
    for (pos, c) in tp_chars.iter().cloned() {
        let mut valid_neighbors = Direction::each()
            .filter_map(|dir| pos.step_in_direction_checked(dir))
            .filter_map(|pos| {
                tp_chars
                    .binary_search_by_key(&pos, |(pos, _)| *pos)
                    .ok()
                    .map(|idx| tp_chars[idx])
            });

        if valid_neighbors.clone().count() == 0 {
            println!("{}", pos);
        }
        let (neighbor_pos, neighbor_c) = valid_neighbors
            .next()
            .ok_or(AoCError::IncorrectInput("teleport point with no neighbors"))?;
        if valid_neighbors.next().is_some() {
            return Err(AoCError::IncorrectInput(
                "teleport point with multiple neighbors",
            ));
        }

        if pos >= neighbor_pos {
            if !tp_points.contains_key(&neighbor_pos) {
                return Err(AoCError::IncorrectInput(
                    "teleport point that forms a chain",
                ));
            }
            continue;
        }

        let name = Name(c, neighbor_c);
        // Vertical
        let (before, after) = if pos.x == neighbor_pos.x {
            (Direction::North, Direction::South)
        } else {
            (Direction::West, Direction::East)
        };

        let connection_pos = match (
            pos.step_in_direction_checked(before)
                .and_then(|p| layout.get(&p).cloned()),
            layout.get(&pos.move_in_direction(after, 2)).cloned(),
        ) {
            (None, None) => return Err(AoCError::IncorrectInput("teleport has no connected cell")),
            (Some(_), Some(_)) => {
                return Err(AoCError::IncorrectInput("teleport has two connected cells"))
            }
            (Some(p), None) if p == Cell::Open => pos.step_in_direction(before),
            (None, Some(p)) if p == Cell::Open => pos.move_in_direction(after, 2),
            _ => return Err(AoCError::IncorrectInput("teleport cell isn't open")),
        };

        tp_points.insert(pos, (connection_pos, name));
    }

    let mut tp_by_name: HashMap<Name, (Vec2us, Option<Vec2us>)> = HashMap::new();
    for (_, (connection_pos, name)) in tp_points {
        match tp_by_name.entry(name) {
            Entry::Occupied(mut slot) => {
                let v = slot.get_mut();
                if v.1.is_some() {
                    return Err(AoCError::IncorrectInput("teleport to more than 1 point"));
                }
                v.1 = Some(connection_pos);
            }
            Entry::Vacant(slot) => {
                slot.insert((connection_pos, None));
            }
        }
    }

    let mut tps = HashMap::new();
    let mut entrance = None;
    let mut exit = None;
    for (name, (a, b)) in tp_by_name {
        if let Some(b) = b {
            *layout.get_mut(&a).unwrap() = Cell::TeleportPoint(name);
            *layout.get_mut(&b).unwrap() = Cell::TeleportPoint(name);
            tps.insert(a, b);
            tps.insert(b, a);
        } else if name.is_entrance() {
            *layout.get_mut(&a).unwrap() = Cell::Entrance;
            entrance = Some(a);
        } else if name.is_exit() {
            *layout.get_mut(&a).unwrap() = Cell::Exit;
            exit = Some(a);
        } else {
            return Err(AoCError::IncorrectInput("teleport without match"));
        }
    }

    let entrance = entrance.ok_or(AoCError::IncorrectInput("no entrance"))?;
    let exit = exit.ok_or(AoCError::IncorrectInput("no exit"))?;

    Ok(Map {
        layout,
        tps,
        entrance,
        exit,
    })
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Name(u8, u8);

impl Name {
    fn is_entrance(&self) -> bool {
        *self == Name(b'A', b'A')
    }
    fn is_exit(&self) -> bool {
        *self == Name(b'Z', b'Z')
    }
}

impl Hash for Name {
    fn hash<H: Hasher>(&self, h: &mut H) {
        h.write_u16((self.0 as u16) << 8 | (self.1 as u16));
    }
}

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name({}{})", self.0 as char, self.1 as char)
    }
}
impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use fmt::Write;
        f.write_char(self.0 as char)?;
        f.write_char(self.1 as char)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Wall,
    Open,
    TeleportPoint(Name),
    Entrance,
    Exit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    layout: HashMap<Vec2us, Cell>,
    tps: HashMap<Vec2us, Vec2us>,
    entrance: Vec2us,
    exit: Vec2us,
}

#[test]
fn day20() -> Result<()> {
    const EXAMPLE: &'static str = "
             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     ";

    let map = parse(EXAMPLE)?;
    assert_eq!(pt2(map)?, 396);

    Ok(())
}
