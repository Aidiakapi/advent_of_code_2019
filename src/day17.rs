use crate::direction::{Direction, MoveInDirection};
use crate::intcode::{
    ascii::{Ascii, AsciiOp},
    growing_memory,
    util::{parse_intcode, reading_not_supported},
    Error as icError, Value, VM,
};
use crate::HashSet;
use itertools::Itertools;
type Vec2 = crate::vec2::Vec2<usize>;

module!(pt1: parse_intcode, pt2: parse_intcode);

fn get_scaffolding_image(memory: Vec<Value>) -> Result<Image> {
    use num::ToPrimitive;
    let mut vm = VM::new(growing_memory(memory));

    let mut walls = HashSet::new();
    let mut pos = Vec2::new(0, 0);
    let mut robot = None;
    vm.run_all(reading_not_supported, |c| {
        match c.to_u8() {
            Some(b'#') => {
                walls.insert(pos);
            }
            Some(c @ b'^') | Some(c @ b'v') | Some(c @ b'<') | Some(c @ b'>') => {
                if let Some((prev_pos, _)) = robot {
                    return Err(icError::Custom(format!(
                        "multiple robots in output, first at {}, second at {}",
                        prev_pos, pos
                    )));
                }
                walls.insert(pos);
                robot = Some((
                    pos,
                    match c {
                        b'^' => Direction::North,
                        b'v' => Direction::South,
                        b'<' => Direction::West,
                        b'>' => Direction::East,
                        _ => unreachable!(),
                    },
                ));
            }
            Some(b'.') => {}
            Some(b'\n') => {
                pos.x = 0;
                pos.y += 1;
                return Ok(());
            }
            _ => {
                return Err(icError::Custom(format!(
                    "value of '{}' is not a valid character",
                    c
                )))
            }
        }
        pos.x += 1;
        Ok(())
    })?;

    Ok(Image {
        walls,
        robot: robot.ok_or(AoCError::IncorrectInput("no robot in input"))?,
    })
}

fn directions_with_scaffold<'img>(
    pos: Vec2,
    image: &'img Image,
) -> impl Iterator<Item = (Direction, Vec2)> + 'img {
    Direction::each().filter_map(move |dir| {
        pos.step_in_direction_checked(dir)
            .filter(|next_pos| image.walls.contains(&next_pos))
            .map(|next_pos| (dir, next_pos))
    })
}

fn pt1(memory: Vec<Value>) -> Result<usize> {
    let image = get_scaffolding_image(memory)?;
    Ok(image
        .walls
        .iter()
        .filter(|&&pos| directions_with_scaffold(pos, &image).count() == 4)
        .map(|pos| pos.x * pos.y)
        .sum())
}

fn pt2(mut memory: Vec<Value>) -> Result<Value> {
    let image = get_scaffolding_image(memory.clone())?;
    let (main, funcs) = create_path_and_program(&image)?.ok_or(AoCError::NoSolution)?;

    let mut input = String::new();
    input.extend(
        main.into_iter()
            .map(|idx| match idx {
                0 => 'A',
                1 => 'B',
                2 => 'C',
                _ => unreachable!(),
            })
            .intersperse(','),
    );
    input.push('\n');
    for func in funcs.iter() {
        for (cmd, grp) in func.iter().cloned().group_by(|cmd| *cmd).into_iter() {
            let count = grp.count();
            match cmd {
                Command::Left => {
                    for _ in 0..count {
                        input.push_str("L,");
                    }
                }
                Command::Right => {
                    for _ in 0..count {
                        input.push_str("R,");
                    }
                }
                Command::Move => {
                    use std::fmt::Write;
                    write!(input, "{},", count).unwrap();
                }
            }
        }
        // Remove last comma
        input.pop();
        input.push('\n');
    }
    input.push_str("n\n");
    // input.push_str("y\n");

    memory[0] = 2;
    let mut vm = VM::new(growing_memory(memory));
    let mut dust = 0;
    vm.run_ascii(|op| match op {
        AsciiOp::Read(out) => {
            if input.is_empty() {
                reading_not_supported()?;
            }
            std::mem::swap(out, &mut input);
            Ok(())
        }
        AsciiOp::WriteAscii(_) => Ok(()),
        AsciiOp::Write(value) => {
            if value < 0 {
                Err(icError::Custom(format!(
                    "value {} is not a valid output",
                    value
                )))
            } else {
                dust += value;
                Ok(())
            }
        }
    })?;

    Ok(dust)
}

type Program = (Vec<usize>, [Vec<Command>; 3]);

fn create_path_and_program(image: &Image) -> Result<Option<Program>> {
    let mut path = create_path(image)?;

    Ok(create_movement_program(&path).or_else(|| {
        if path.len() >= 2 && path[0] == Command::Right && path[1] == Command::Right {
            path[0] = Command::Left;
            path[1] = Command::Left;
            create_movement_program(&path)
        } else {
            None
        }
    }))
}

fn create_path(image: &Image) -> Result<Vec<Command>> {
    let mut walls_around_robot = directions_with_scaffold(image.robot.0, &image);
    let (start_dir, start_pos) = walls_around_robot
        .next()
        .ok_or(AoCError::IncorrectInput("robot cannot move anywhere"))?;
    if walls_around_robot.next().is_some() {
        return Err(AoCError::IncorrectInput(
            "expected robot to start at the endpoint of a scaffold",
        ));
    }

    let mut path = Vec::with_capacity(image.walls.len() + 32);
    if start_dir == image.robot.1 {
    } else if start_dir == image.robot.1.clockwise() {
        path.push(Command::Right);
    } else if start_dir == image.robot.1.counterclockwise() {
        path.push(Command::Left);
    } else {
        path.push(Command::Right);
        path.push(Command::Right);
    }

    let mut pos = start_pos;
    let mut dir = start_dir;
    let mut current_forward_dist = 1;
    loop {
        match pos
            .step_in_direction_checked(dir)
            .filter(|next_pos| image.walls.contains(&next_pos))
        {
            Some(next_pos) => {
                pos = next_pos;
                current_forward_dist += 1;
            }
            None => {
                path.resize(path.len() + current_forward_dist, Command::Move);
                let right_pos = pos
                    .step_in_direction_checked(dir.clockwise())
                    .filter(|next_pos| image.walls.contains(&next_pos));
                let left_pos = pos
                    .step_in_direction_checked(dir.counterclockwise())
                    .filter(|next_pos| image.walls.contains(&next_pos));
                match (right_pos, left_pos) {
                    (Some(next_pos), None) => {
                        path.push(Command::Right);
                        dir = dir.clockwise();
                        pos = next_pos;
                    }
                    (None, Some(next_pos)) => {
                        path.push(Command::Left);
                        dir = dir.counterclockwise();
                        pos = next_pos;
                    }
                    (Some(_), Some(_)) => {
                        return Err(AoCError::IncorrectInput("T junction in input"))
                    }
                    (None, None) => {
                        break;
                    }
                }
                current_forward_dist = 1;
            }
        }
    }

    Ok(path)
}

fn program_length(prog: &[Command]) -> usize {
    if prog.is_empty() {
        return 0;
    }
    let mut first_move_n_ago = 0;
    let mut total = 0;
    for cmd in prog {
        if cmd == &Command::Move {
            if first_move_n_ago == 0 {
                total += 2;
            } else if first_move_n_ago == 9 {
                total += 1;
            }
            first_move_n_ago += 1;
        } else {
            total += 2;
            first_move_n_ago = 0;
        }
    }
    total - 1
}

fn create_movement_program(path: &[Command]) -> Option<Program> {
    type Range = std::ops::Range<usize>;
    #[derive(Debug, Clone, PartialEq, Eq)]
    enum Part {
        Slice(Range),
        Prog(usize),
    }
    use Part::*;

    fn for_each_substitution_impl<F>(
        index: usize,
        starts: &[usize],
        length: usize,
        buffer: &mut Vec<usize>,
        callback: &mut F,
    ) where
        F: FnMut(&[usize]),
    {
        if index >= starts.len() {
            callback(buffer);
            return;
        }
        buffer.push(starts[index]);
        let mut next_idx = index + 1;
        while next_idx < starts.len() && starts[next_idx] < starts[index] + length {
            next_idx += 1;
        }
        for_each_substitution_impl(next_idx, starts, length, buffer, callback);
        // Special case, the first entry always has to be included
        if index == 0 {
            return;
        }
        buffer.pop();
        // If there was an overlap, also check substitutions without this entry
        for_each_substitution_impl(index + 1, starts, length, buffer, callback);
    }

    fn for_each_substitution<F>(
        starts: &[usize],
        length: usize,
        buffer: &mut Vec<usize>,
        mut callback: F,
    ) where
        F: FnMut(&[usize]),
    {
        if starts.is_empty() {
            return;
        }
        buffer.clear();
        for_each_substitution_impl(0, starts, length, buffer, &mut callback);
    }

    let mut progs: [Option<Range>; 3] = [None, None, None];
    let mut pts = vec![Slice(0..path.len())];
    fn search_path(
        path: &[Command],
        progs: &mut [Option<Range>; 3],
        pts: &mut Vec<Part>,
        prog_idx: usize,
    ) -> bool {
        // Recursive exit condition
        if prog_idx == 3 {
            // Check if there has been full substitution
            if pts.iter().any(|pt| match pt {
                Slice(_) => true,
                Prog(_) => false,
            }) {
                return false;
            }
            // Validate that all path lenghts are below 20
            if pts.len() > 10
                || progs
                    .iter()
                    .any(|prog| program_length(&path[prog.as_ref().unwrap().clone()]) > 20)
            {
                return false;
            }
            return true;
        }

        let path_ranges = pts
            .iter()
            .filter_map(|pt| match pt {
                Slice(range) => Some(range),
                Prog(_) => None,
            })
            .cloned();
        let path_cmds = path_ranges.clone().flat_map(|range| {
            path[range.clone()]
                .iter()
                .enumerate()
                .map(move |(idx, cmd)| (range.clone(), idx + range.start, *cmd))
        });

        let first_range = if let Some(range) = path_ranges.clone().next() {
            range
        } else {
            return false;
        };
        let mut matches: Vec<(Range, usize)> = path_cmds
            .clone()
            .filter(|(_, _, cmd)| cmd == &path[first_range.start])
            .map(|(range, idx, _)| (range, idx))
            .skip(1) // Skips the pattenr to be matched
            .collect();

        if matches.is_empty() {
            return false;
        }
        debug_assert!(matches.is_sorted_by_key(|(_, idx)| *idx));

        let mut starts = Vec::new();
        let mut buffer = Vec::new();
        let mut new_pts = Vec::with_capacity(pts.len() * 2);
        for offset in 1..first_range.len() {
            if program_length(&path[first_range.start..=first_range.start + offset]) > 20 {
                break;
            }
            // Prune matches to accomodate for the next command
            let next_cmd = path[first_range.start + offset];
            matches.drain_filter(|(range, idx)| {
                let i = *idx + offset;
                !range.contains(&i) || path[i] != next_cmd
            });
            // Don't bother checking substitutions that would consume
            // more than 4 invocations, since there's 3 functions, and
            // a total of 10 invocations possible.
            if matches.len() > 3 {
                continue;
            }
            if matches.is_empty() {
                break;
            }

            starts.clear();
            starts.push(first_range.start);
            starts.extend(matches.iter().map(|(_, idx)| *idx));
            let mut path_found = false;
            for_each_substitution(&starts, offset + 1, &mut buffer, |starts| {
                if path_found {
                    return;
                }
                new_pts.clear();
                let mut iter = starts.iter().cloned();
                let mut next = iter.next();
                'outer: for pt in pts.iter().cloned() {
                    match pt {
                        Prog(p) => new_pts.push(Prog(p)),
                        Slice(mut range) => {
                            loop {
                                // Next item doesn't exist or isn't in range
                                let idx = match next.clone().filter(|idx| *idx < range.end) {
                                    Some(x) => x,
                                    None => {
                                        new_pts.push(Slice(range));
                                        continue 'outer;
                                    }
                                };
                                // Slice before
                                if range.start != idx {
                                    new_pts.push(Slice(range.start..idx));
                                }
                                // Invoke substitution
                                new_pts.push(Prog(prog_idx));
                                // Update range
                                range = idx + offset + 1..range.end;
                                next = iter.next();
                                if range.is_empty() {
                                    continue 'outer;
                                }
                            }
                        }
                    }
                }
                // Recursively substitute the next programs
                progs[prog_idx] = Some(first_range.start..first_range.start + offset + 1);
                if search_path(path, progs, &mut new_pts, prog_idx + 1) {
                    std::mem::swap(pts, &mut new_pts);
                    path_found = true;
                } else {
                    progs[prog_idx] = None;
                }
            });
            if path_found {
                return true;
            }
        }

        false
    }

    if search_path(&path, &mut progs, &mut pts, 0) {
        let main = pts
            .into_iter()
            .map(|pt| match pt {
                Slice(_) => unreachable!(),
                Prog(idx) => idx,
            })
            .collect();
        let progs: [Vec<Command>; 3] = [
            path[progs[0].as_ref().unwrap().clone()].to_vec(),
            path[progs[1].as_ref().unwrap().clone()].to_vec(),
            path[progs[2].as_ref().unwrap().clone()].to_vec(),
        ];
        Some((main, progs))
    } else {
        None
    }
}

#[derive(Debug, Clone)]
struct Image {
    walls: HashSet<Vec2>,
    robot: (Vec2, Direction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Command {
    Left,
    Right,
    Move,
}

#[test]
fn day17() -> Result<()> {
    let mut walls = HashSet::new();
    let robot = (Vec2::new(0, 6), Direction::North);
    for (y, line) in "\
#######...#####
#.....#...#...#
#.....#...#...#
......#...#...#
......#...###.#
......#.....#.#
^########...#.#
......#.#...#.#
......#########
........#...#..
....#########..
....#...#......
....#...#......
....#...#......
....#####......"
        .lines()
        .enumerate()
    {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' | '^' | 'v' | '<' | '>' => {
                    walls.insert(Vec2::new(x, y));
                }
                '.' => {}
                _ => unreachable!(),
            }
        }
    }

    let image = Image { walls, robot };
    assert!(create_path_and_program(&image)?.is_some());

    Ok(())
}
