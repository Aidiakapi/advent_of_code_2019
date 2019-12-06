module!(pt1: parse, pt2: parse);

use std::collections::{hash_map::Entry, HashMap};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Orbit<'s> {
    target: &'s str,
    object: &'s str,
}

#[derive(Debug)]
struct Body<'s> {
    name: &'s str,
    parent: Option<&'s str>,
    children: Vec<&'s str>,
}

fn create_bodies<'s>(orbits: &Vec<Orbit<'s>>) -> Result<HashMap<&'s str, Body<'s>>> {
    let mut bodies: HashMap<&str, Body> = HashMap::with_capacity(orbits.len());
    for orbit in orbits {
        match bodies.entry(orbit.target) {
            Entry::Occupied(existing_parent) => {
                existing_parent.into_mut().children.push(orbit.object);
            }
            Entry::Vacant(slot) => {
                slot.insert(Body {
                    name: orbit.target,
                    parent: None,
                    children: vec![orbit.object],
                });
            }
        }

        match bodies.entry(orbit.object) {
            Entry::Occupied(existing_body) => {
                let existing_body = existing_body.into_mut();
                if existing_body.parent.is_some() {
                    return Err(AoCError::IncorrectInput(
                        "multiple parents specified for the same body",
                    ));
                }
                existing_body.parent = Some(orbit.target);
            }
            Entry::Vacant(slot) => {
                slot.insert(Body {
                    name: orbit.target,
                    parent: Some(orbit.target),
                    children: Vec::new(),
                });
            }
        }
    }
    Ok(bodies)
}

fn pt1(orbits: Vec<Orbit>) -> Result<usize> {
    let bodies = create_bodies(&orbits)?;
    let mut total = 0;
    fn dfs<'s>(
        bodies: &HashMap<&'s str, Body<'s>>,
        total: &mut usize,
        current_depth: usize,
        current_body: &'s str,
    ) {
        if let Some(body) = bodies.get(current_body) {
            *total += current_depth;
            for &child in &body.children {
                dfs(bodies, total, current_depth + 1, child);
            }
        }
    }
    dfs(&bodies, &mut total, 0, "COM");
    Ok(total)
}

fn pt2(orbits: Vec<Orbit>) -> Result<usize> {
    let bodies = create_bodies(&orbits)?;
    fn dfs<'s>(
        bodies: &HashMap<&'s str, Body<'s>>,
        current_depth: usize,
        current_body: &'s str,
        has_santa: &mut Option<usize>,
        has_you: &mut Option<usize>,
        result: &mut Option<usize>,
    ) {
        if let Some(body) = bodies.get(current_body) {
            if current_body == "YOU" {
                *has_you = Some(current_depth);
            }
            if current_body == "SAN" {
                *has_santa = Some(current_depth);
            }
            let mut current_has_santa = None;
            let mut current_has_you = None;

            for &child in &body.children {
                dfs(
                    bodies,
                    current_depth + 1,
                    child,
                    &mut current_has_santa,
                    &mut current_has_you,
                    result,
                );
            }

            if result.is_some() {
                return;
            }
            if let Some(current_has_santa) = current_has_santa {
                *has_santa = Some(current_has_santa);
            }
            if let Some(current_has_you) = current_has_you {
                *has_you = Some(current_has_you);
            }

            match (*has_santa, *has_you) {
                (Some(santa), Some(you)) => {
                    *result = Some(santa + you - current_depth * 2);
                }
                _ => {}
            }
        }
    }
    let mut result = None;
    dfs(&bodies, 0, "COM", &mut None, &mut None, &mut result);
    result.ok_or(AoCError::NoSolution)
}

fn parse(s: &str) -> IResult<&str, Vec<Orbit>> {
    use parsers::*;
    let orbit = map(
        pair(terminated(alphanumeric1, char(')')), alphanumeric1),
        |(target, object)| Orbit { target, object },
    );
    separated_list(line_ending, orbit)(s)
}

#[test]
fn day06() -> Result<()> {
    use crate::module::ToModuleResult;
    let input = parse(
        "\
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L",
    )
    .to_module_result()?;
    assert_eq!(pt1(input)?, 42);

    let input = parse(
        "\
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN",
    )
    .to_module_result()?;
    assert_eq!(pt2(input)?, 4);

    Ok(())
}
