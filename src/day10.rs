module!(pt1: parse, pt2: parse);

use crate::mat2::Mat2;
use crate::vec2::Vec2us;
use crate::HashSet;
use num::integer::Integer;
use std::convert::Into;

type Vec2 = crate::vec2::Vec2i;

struct CloserToCenterIter {
    pos: Vec2,
    offset: Vec2,
}
impl Iterator for CloserToCenterIter {
    type Item = Vec2;
    fn next(&mut self) -> Option<Vec2> {
        if self.pos == (0, 0).into() {
            return None;
        }
        let prev = self.pos;
        self.pos -= self.offset;
        Some(prev)
    }
}

fn closer_to_center(pos: Vec2) -> CloserToCenterIter {
    let offset = pos / pos.x.abs().gcd(&pos.y.abs()).max(1);
    CloserToCenterIter {
        pos: pos - offset,
        offset,
    }
}

fn asteroid_positions<'a>(grid: &'a Mat2<bool>) -> impl Iterator<Item = Vec2> + Clone + 'a {
    grid.iter()
        .filter(|&(_, &is_asteroid)| is_asteroid)
        .map(|(pos, _)| pos.convert().unwrap())
}

fn find_ideal_spot(asteroids: &HashSet<Vec2>) -> (Vec2, usize) {
    asteroids
        .iter()
        .map(|&asteroid1| {
            (
                asteroid1,
                asteroids
                    .iter()
                    .filter(|&&asteroid2| {
                        closer_to_center(asteroid2 - asteroid1)
                            .all(|pos| !asteroids.contains(&(pos + asteroid1)))
                    })
                    .count()
                    // subtract 1, because pos itself is also visible
                    - 1,
            )
        })
        .max_by_key(|(_, visible_count)| *visible_count)
        .unwrap()
}

fn vaporization_order(mut asteroids: HashSet<Vec2>) -> (Vec2, Vec<Vec2>) {
    let laser = find_ideal_spot(&asteroids).0;
    asteroids.remove(&laser);

    let mut vaporization_order: Vec<_> =
        asteroids.iter().map(|&asteroid| asteroid - laser).collect();
    vaporization_order.sort_unstable_by_key(|&asteroid| {
        let closer_to_center_count = closer_to_center(asteroid)
            .filter(|&pos| asteroids.contains(&(pos + laser)))
            .count();
        (
            closer_to_center_count,
            ORIENTATION_TABLE[(asteroid.y + 30) as usize][(asteroid.x + 30) as usize],
        )
    });
    for asteroid in &mut vaporization_order {
        *asteroid += laser;
    }

    (laser, vaporization_order)
}

fn pt1(grid: Mat2<bool>) -> usize {
    find_ideal_spot(&asteroid_positions(&grid).collect()).1
}

fn pt2(grid: Mat2<bool>) -> Result<i32> {
    vaporization_order(asteroid_positions(&grid).collect())
        .1
        .get(199)
        .map(|pos| pos.x * 100 + pos.y)
        .ok_or(AoCError::NoSolution)
}

fn parse(s: &str) -> IResult<&str, Mat2<bool>> {
    use parsers::*;
    let line = many1(map(one_of(".#"), |c| c == '#'));
    let grid = separated_list(line_ending, line);
    map_res(grid, |lines| {
        let height = lines.len();
        if height == 0 {
            return Err(AoCError::NomParse("empty input".to_owned()));
        }
        let width = lines[0].len();
        if lines[1..].iter().any(|row| row.len() != width) {
            return Err(AoCError::NomParse("inconsistent row width".to_owned()));
        }
        let mut grid = Mat2::new(false, Vec2us::new(width, height));
        for (y, line) in lines.iter().enumerate() {
            for (x, &value) in line.iter().enumerate() {
                grid[Vec2us::new(x, y)] = value;
            }
        }
        Ok(grid)
    })(s)
}

#[test]
fn day10() -> Result<()> {
    let vape_order = vaporization_order(
        asteroid_positions(
            &parse(
                "\
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##",
            )?
            .1,
        )
        .collect(),
    )
    .1;
    assert_eq!(vape_order[0], Vec2::new(11, 12));
    assert_eq!(vape_order[1], Vec2::new(12, 1));
    assert_eq!(vape_order[2], Vec2::new(12, 2));
    assert_eq!(vape_order[9], Vec2::new(12, 8));
    assert_eq!(vape_order[19], Vec2::new(16, 0));
    assert_eq!(vape_order[49], Vec2::new(16, 9));
    assert_eq!(vape_order[99], Vec2::new(10, 16));
    assert_eq!(vape_order[198], Vec2::new(9, 6));
    assert_eq!(vape_order[199], Vec2::new(8, 2));
    assert_eq!(vape_order[200], Vec2::new(10, 9));
    assert_eq!(vape_order[298], Vec2::new(11, 1));

    Ok(())
}

include!("day10_table.rs");
