module!(pt1: parse, pt2: parse);

use crate::HashSet;
use num::{Integer, Signed};
use std::convert::Into;

type Vec3 = crate::vec3::Vec3i;

#[derive(Debug, Clone, PartialEq, Eq)]
struct CelestialBody {
    position: Vec3,
    velocity: Vec3,
}

impl CelestialBody {
    fn potential_energy(&self) -> i32 {
        let p = self.position.abs();
        p.x + p.y + p.z
    }

    fn kinetic_energy(&self) -> i32 {
        let v = self.velocity.abs();
        v.x + v.y + v.z
    }
}

fn move_bodies(bodies: &mut [CelestialBody]) {
    for a in 0..bodies.len() - 1 {
        for b in a + 1..bodies.len() {
            let pa = bodies[a].position;
            let pb = bodies[b].position;
            let mut v = pb - pa;
            v.x = v.x.min(1).max(-1);
            v.y = v.y.min(1).max(-1);
            v.z = v.z.min(1).max(-1);
            bodies[a].velocity += v;
            bodies[b].velocity -= v;
        }
    }
    for body in bodies {
        body.position += body.velocity;
    }
}

fn calculate_energy(bodies: &[CelestialBody]) -> i32 {
    bodies
        .iter()
        .map(|body| body.potential_energy() * body.kinetic_energy())
        .sum()
}

fn pt1(mut bodies: Vec<CelestialBody>) -> i32 {
    for _ in 0..1000 {
        move_bodies(&mut bodies);
    }
    calculate_energy(&bodies)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BodyPart {
    position: i32,
    velocity: i32,
}

fn calculate_cycle_time(initial_parts: [BodyPart; 4]) -> Result<u64> {
    let mut parts = initial_parts;
    let mut map = HashSet::new();
    let mut iteration = 0;
    loop {
        if !map.insert(parts) {
            break if parts != initial_parts {
                Err(AoCError::IncorrectInput(
                    "cycle doesn't start from the initial position",
                ))
            } else {
                Ok(iteration)
            };
        }
        iteration += 1;

        // Update velocity
        for i in 0..3 {
            let pa = parts[i].position;
            for j in i + 1..4 {
                let pb = parts[j].position;
                let v = (pb - pa).min(1).max(-1);
                parts[i].velocity += v;
                parts[j].velocity -= v;
            }
        }
        // Update position
        for part in &mut parts {
            part.position += part.velocity;
        }
    }
}

fn pt2(bodies: Vec<CelestialBody>) -> Result<u64> {
    if bodies.len() != 4 {
        return Err(AoCError::IncorrectInput("expected 4 celestial bodies"));
    }

    let mut parts = [[BodyPart {
        position: 0,
        velocity: 0,
    }; 4]; 3];
    for (i, body) in bodies.into_iter().enumerate() {
        parts[0][i].position = body.position.x;
        parts[0][i].velocity = body.velocity.x;
        parts[1][i].position = body.position.y;
        parts[1][i].velocity = body.velocity.y;
        parts[2][i].position = body.position.z;
        parts[2][i].velocity = body.velocity.z;
    }

    let cycle_times = [
        calculate_cycle_time(parts[0])?,
        calculate_cycle_time(parts[1])?,
        calculate_cycle_time(parts[2])?,
    ];
    Ok(cycle_times[0].lcm(&cycle_times[1]).lcm(&cycle_times[2]))
}

fn parse(s: &str) -> IResult<&str, Vec<CelestialBody>> {
    use parsers::*;
    let celestial_body = map(
        tuple((
            preceded(tag("<x="), i32_str),
            preceded(tag(", y="), i32_str),
            delimited(tag(", z="), i32_str, tag(">")),
        )),
        |pos| CelestialBody {
            position: pos.into(),
            velocity: Vec3::default(),
        },
    );
    separated_list1(line_ending, celestial_body)(s)
}

#[test]
fn day12() -> Result<()> {
    let mut bodies = parse(
        "\
<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>",
    )?
    .1;
    move_bodies(&mut bodies);
    #[rustfmt::skip]
    assert_eq!(&bodies, &vec![
        CelestialBody { position: ( 2, -1,  1).into(), velocity: ( 3, -1, -1).into() },
        CelestialBody { position: ( 3, -7, -4).into(), velocity: ( 1,  3,  3).into() },
        CelestialBody { position: ( 1, -7,  5).into(), velocity: (-3,  1, -3).into() },
        CelestialBody { position: ( 2,  2,  0).into(), velocity: (-1, -3,  1).into() },
    ]);

    #[rustfmt::skip]
    let bodies = vec![
        CelestialBody { position: (  8, -12, -9).into(), velocity: (-7,   3,  0).into() },
        CelestialBody { position: ( 13,  16, -3).into(), velocity: ( 3, -11, -5).into() },
        CelestialBody { position: (-29, -11, -1).into(), velocity: (-3,   7,  4).into() },
        CelestialBody { position: ( 16, -13, 23).into(), velocity: ( 7,   1,  1).into() },
    ];
    assert_eq!(1940, calculate_energy(&bodies));

    Ok(())
}
