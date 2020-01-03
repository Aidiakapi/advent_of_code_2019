use crate::HashSet;
use std::fmt;

module!(pt1: parse, pt2: parse);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ErisBugs(u32);

impl ErisBugs {
    fn grow_with_rules<F>(&self, get_neighbor_count: F) -> ErisBugs
    where
        F: Fn(usize) -> u32,
    {
        let mut new = 0u32;
        for y in 0..5 {
            for x in 0..5 {
                let i = y * 5 + x;
                let neighbor_count = get_neighbor_count(i);
                let will_be_infested = if (self.0 >> i) & 1 == 1 {
                    neighbor_count == 1
                } else {
                    neighbor_count == 1 || neighbor_count == 2
                };
                if will_be_infested {
                    new |= 1 << i;
                }
            }
        }
        ErisBugs(new)
    }

    fn basic_rule(&self, i: usize) -> u32 {
        let (x, y) = (i % 5, i / 5);
        let mut neighbor_count = 0u32;
        if y > 0 {
            neighbor_count += (self.0 >> (i - 5)) & 1; // North
        }
        if y < 4 {
            neighbor_count += (self.0 >> (i + 5)) & 1; // South
        }
        if x > 0 {
            neighbor_count += (self.0 >> (i - 1)) & 1; // West
        }
        if x < 4 {
            neighbor_count += (self.0 >> (i + 1)) & 1; // East
        }
        neighbor_count
    }

    fn grow(&self) -> ErisBugs {
        self.grow_with_rules(|i| self.basic_rule(i))
    }

    fn grow_recursive(&self, outer: &ErisBugs, inner: &ErisBugs) -> ErisBugs {
        const MASK: u32 = 0b1111111111110111111111111;
        let mut bugs = ErisBugs(self.0 & MASK);
        bugs = bugs.grow_with_rules(|i| {
            let (x, y) = (i % 5, i / 5);
            let mut neighbor_count = bugs.basic_rule(i);

            // Outer neighbors
            if y == 0 {
                neighbor_count += (outer.0 >> 7) & 1;
            }
            if y == 4 {
                neighbor_count += (outer.0 >> 17) & 1;
            }
            if x == 0 {
                neighbor_count += (outer.0 >> 11) & 1;
            }
            if x == 4 {
                neighbor_count += (outer.0 >> 13) & 1;
            }

            // Inner neighbors
            match (x, y) {
                (2, 1) => neighbor_count += (inner.0 & 0b0000000000000000000011111).count_ones(),
                (2, 3) => neighbor_count += (inner.0 & 0b1111100000000000000000000).count_ones(),
                (1, 2) => neighbor_count += (inner.0 & 0b0000100001000010000100001).count_ones(),
                (3, 2) => neighbor_count += (inner.0 & 0b1000010000100001000010000).count_ones(),
                _ => {}
            }
            neighbor_count
        });
        ErisBugs(bugs.0 & MASK)
    }
}

impl fmt::Display for ErisBugs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use fmt::Write;
        for y in 0..5 {
            for x in 0..5 {
                let i = x + y * 5;
                f.write_char(if (self.0 >> i) & 1 == 1 { '#' } else { '.' })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn pt1(mut bugs: ErisBugs) -> u32 {
    let mut seen = HashSet::new();
    loop {
        if !seen.insert(bugs) {
            break bugs.0;
        }
        bugs = bugs.grow();
    }
}

fn trim_left_matches<T, F>(mut slice: &[T], mut predicate: F) -> &[T]
where
    F: FnMut(&T) -> bool,
{
    while let Some(first) = slice.first() {
        if predicate(first) {
            slice = &slice[1..];
        } else {
            break;
        }
    }
    slice
}

fn trim_right_matches<T, F>(mut slice: &[T], mut predicate: F) -> &[T]
where
    F: FnMut(&T) -> bool,
{
    while let Some(last) = slice.last() {
        if predicate(last) {
            slice = &slice[..slice.len() - 1];
        } else {
            break;
        }
    }
    slice
}

fn trim_matches<T, F>(mut slice: &[T], mut predicate: F) -> &[T]
where
    F: FnMut(&T) -> bool,
{
    slice = trim_left_matches(slice, &mut predicate);
    slice = trim_right_matches(slice, &mut predicate);
    slice
}

fn grow_recursive_levels(levels: &mut Vec<ErisBugs>, buffer: &mut Vec<ErisBugs>) {
    // Back up the current state, and ensure that there's exactly 1 empty layer
    // at the start and end.
    buffer.clear();
    buffer.push(ErisBugs(0));
    let slice = trim_matches(&levels, |bugs| bugs.0 == 0);
    buffer.extend_from_slice(slice);
    buffer.push(ErisBugs(0));

    // Construct the new layer
    levels.clear();
    levels.push(buffer[0].grow_recursive(&ErisBugs(0), &buffer[1]));

    for (idx, bugs) in buffer.iter().enumerate().skip(1).take(buffer.len() - 2) {
        levels.push(bugs.grow_recursive(&buffer[idx - 1], &buffer[idx + 1]));
    }
    levels.push(buffer[buffer.len() - 1].grow_recursive(&buffer[buffer.len() - 2], &ErisBugs(0)));
}

fn pt2(init: ErisBugs) -> u32 {
    let mut levels = Vec::new();
    let mut buffer = Vec::new();
    levels.push(init);
    for _ in 0..200 {
        grow_recursive_levels(&mut levels, &mut buffer);
    }
    levels.iter().map(|bugs| bugs.0.count_ones()).sum()
}

fn parse(s: &str) -> Result<ErisBugs> {
    let mut chars = s.chars();
    let mut state = 0;
    for y in 0..5 {
        for x in 0..5 {
            match chars.next() {
                Some('.') => {}
                Some('#') => state |= 1 << (x + y * 5),
                Some('?') if x == 2 && y == 2 => {}
                _ => return Err(AoCError::IncorrectInput("invalid char, expected . or #")),
            }
        }
        if y == 4 {
            if chars.next().is_some() {
                return Err(AoCError::IncorrectInput("expected end of input"));
            }
        } else {
            match chars.next() {
                Some('\n') => {}
                _ => return Err(AoCError::IncorrectInput("expected end of line")),
            }
        }
    }
    Ok(ErisBugs(state))
}

#[test]
fn day24() -> Result<()> {
    #[rustfmt::skip] assert_eq!(
        parse("\
.....
.....
.....
#....
.#...")?,
        ErisBugs(2129920)
    );

    #[rustfmt::skip] assert_eq!(
        parse("\
....#
#..#.
#..##
..#..
#....")?.grow(),
        parse("\
#..#.
####.
###.#
##.##
.##..")?
    );

    #[rustfmt::skip] let input = parse("\
....#
#..#.
#..##
..#..
#....")?;
    assert_eq!(pt1(input), 2129920);

    #[rustfmt::skip] let input = parse("\
....#
#..#.
#..##
..#..
#....")?;
    let mut levels = vec![input];
    let mut buffer = Vec::new();
    for _ in 0..10 {
        grow_recursive_levels(&mut levels, &mut buffer);
    }
    let levels = trim_matches(&levels, |bugs| bugs.0 == 0);
    #[rustfmt::skip] assert_eq!(
        levels,
        vec![
            parse("\
..#..
.#.#.
..?.#
.#.#.
..#..")?,
            parse("\
...#.
...##
..?..
...##
...#.")?,
            parse("\
#.#..
.#...
..?..
.#...
#.#..")?,
            parse("\
.#.##
....#
..?.#
...##
.###.")?,
            parse("\
#..##
...##
..?..
...#.
.####")?,
            parse("\
.#...
.#.##
.#?..
.....
.....")?,
            parse("\
.##..
#..##
..?.#
##.##
#####")?,
            parse("\
###..
##.#.
#.?..
.#.##
#.#..")?,
            parse("\
..###
.....
#.?..
#....
#...#")?,
            parse("\
.###.
#..#.
#.?..
##.#.
.....")?,
            parse("\
####.
#..#.
#.?#.
####.
.....")?,
        ]
        .as_slice()
    );

    Ok(())
}
