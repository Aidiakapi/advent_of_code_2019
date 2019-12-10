module!(pt1: parse, pt2: parse);

use crate::mat2::Mat2;
use crate::vec2::{Vec2i, Vec2us};
use num::integer::Integer;

fn visibility_from_point(position: Vec2i, asteroids: &[Vec2i], visibility_mask: &mut Mat2<bool>) {
    // Reset visibility mask
    for v in visibility_mask.data.iter_mut() {
        *v = true;
    }

    for &goal_pos in asteroids {
        if position == goal_pos {
            continue;
        }

        let delta = goal_pos - position;
        let offset = delta / delta.x.abs().gcd(&delta.y.abs());
        // println!("from {:?} to {:?} => {:?}, {:?}", start_pos, goal_pos, delta, offset);

        let mut current = delta;
        loop {
            current += offset;
            let pos = match (position + current).convert::<usize>() {
                Ok(pos) => pos,
                Err(_) => break,
            };
            if pos.x >= visibility_mask.width() || pos.y >= visibility_mask.height() {
                break;
            }
            visibility_mask[pos] = false;
        }
    }
}

fn create_asteroids(grid: &Mat2<bool>) -> Vec<Vec2i> {
    grid.iter()
        .filter(|(_, value)| **value)
        .map(|(pos, _)| pos.convert::<i32>().unwrap())
        .collect()
}

fn create_visibility_mat(grid: &Mat2<bool>) -> (Mat2<u32>, Vec<Vec2i>) {
    let asteroids = create_asteroids(grid);
    let mut output = Mat2::new(0, grid.size());
    let mut visibility_mask = Mat2::new(true, grid.size());
    for &start_pos in &asteroids {
        visibility_from_point(start_pos, &asteroids, &mut visibility_mask);

        // let mut vis = String::new();
        // for y in 0..output.width() {
        //     for x in 0..output.height() {
        //         vis.push(if visibility_mask[x][y] { '#' } else { '.' });
        //     }
        //     vis.push('\n');
        // }
        // println!("at {:?}\n{}", start_pos, vis);
        output[start_pos.convert::<usize>().unwrap()] = asteroids
            .iter()
            .filter(|&&goal_pos| {
                start_pos != goal_pos && visibility_mask[goal_pos.convert::<usize>().unwrap()]
            })
            .count() as u32;
    }

    (output, asteroids)
}

fn pt1(grid: Mat2<bool>) -> u32 {
    let (visibility_mat, asteroids) = create_visibility_mat(&grid);
    asteroids
        .iter()
        .map(|&asteroid| visibility_mat[asteroid.convert::<usize>().unwrap()])
        .max()
        .unwrap()
}

fn calc_angle(offset: Vec2i) -> f32 {
    let res = (offset.x as f32).atan2(-offset.y as f32);
    if res < -0.00001f32 {
        res + std::f32::consts::PI * 2f32
    } else {
        res
    }
}

fn create_vaporize_order(mut grid: Mat2<bool>) -> Vec<Vec2us> {
    let mut vaporization_order = Vec::new();
    let mut visibility_mask = Mat2::new(false, grid.size());

    let (visibility_mat, mut asteroids) = create_visibility_mat(&grid);
    let laser_pos = *asteroids
        .iter()
        .max_by_key(|&asteroid| visibility_mat[asteroid.convert::<usize>().unwrap()])
        .unwrap();
    loop {
        visibility_from_point(laser_pos, &asteroids, &mut visibility_mask);
        let mut vaporize_this_round = asteroids
            .iter()
            .cloned()
            .filter(|&asteroid| {
                asteroid != laser_pos && visibility_mask[asteroid.convert::<usize>().unwrap()]
            })
            .map(|asteroid| (asteroid, calc_angle(asteroid - laser_pos)))
            .collect::<Vec<_>>();

        vaporize_this_round.sort_unstable_by(|(_, a), (_, b)| a.partial_cmp(&b).unwrap());
        vaporization_order.extend(
            vaporize_this_round
                .iter()
                .map(|&(asteroid, _)| asteroid.convert::<usize>().unwrap()),
        );
        for &(asteroid, _) in &vaporize_this_round {
            grid[asteroid.convert::<usize>().unwrap()] = false;
        }
        asteroids = create_asteroids(&grid);
        if asteroids.len() == 1 {
            break;
        }
    }

    vaporization_order
}

fn pt2(grid: Mat2<bool>) -> usize {
    let order = create_vaporize_order(grid);
    order[199].x * 100 + order[199].y
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
    let (vis_mat, _) = create_visibility_mat(
        &parse(
            "\
.#..#
.....
#####
....#
...##",
        )?
        .1,
    );
    assert_eq!(
        vis_mat.data.as_slice(),
        &[0, 0, 6, 0, 0, 7, 0, 7, 0, 0, 0, 0, 7, 0, 0, 0, 0, 7, 0, 8, 7, 0, 5, 7, 7]
    );

    let vape_order = create_vaporize_order(
        parse(
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
    );
    assert_eq!(vape_order[0], Vec2us::new(11, 12));
    assert_eq!(vape_order[1], Vec2us::new(12, 1));
    assert_eq!(vape_order[2], Vec2us::new(12, 2));
    assert_eq!(vape_order[9], Vec2us::new(12, 8));
    assert_eq!(vape_order[19], Vec2us::new(16, 0));
    assert_eq!(vape_order[49], Vec2us::new(16, 9));
    assert_eq!(vape_order[99], Vec2us::new(10, 16));
    assert_eq!(vape_order[198], Vec2us::new(9, 6));
    assert_eq!(vape_order[199], Vec2us::new(8, 2));
    assert_eq!(vape_order[200], Vec2us::new(10, 9));
    assert_eq!(vape_order[298], Vec2us::new(11, 1));

    Ok(())
}
