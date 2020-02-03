module!(pt1: parse, pt2: parse);

use crate::graph::dfs;
use crate::HashMap;
use std::collections::hash_map::Entry;

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

struct ParentIter<'h, 's: 'h>(&'h HashMap<&'s str, Body<'s>>, Option<&'s str>);
impl<'h, 's: 'h> Iterator for ParentIter<'h, 's> {
    type Item = &'h Body<'s>;
    fn next(&mut self) -> Option<&'h Body<'s>> {
        if let Some(body) = self.0.get(&self.1?) {
            self.1 = body.parent;
            Some(body)
        } else {
            None
        }
    }
}
fn parent_iter<'h, 's: 'h>(
    map: &'h HashMap<&'s str, Body<'s>>,
    start: &'s str,
) -> ParentIter<'h, 's> {
    ParentIter(map, Some(start))
}

fn create_bodies<'s>(orbits: &[Orbit<'s>]) -> Result<HashMap<&'s str, Body<'s>>> {
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
                    name: orbit.object,
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
    dfs(("COM", 0), |(name, current_depth)| {
        total += current_depth;
        let body = &bodies[name];
        body.children
            .iter()
            .map(move |&child| (child, current_depth + 1))
    });
    Ok(total)
}

fn pt2(orbits: Vec<Orbit>) -> Result<usize> {
    let bodies = create_bodies(&orbits)?;
    let visited = parent_iter(&bodies, "YOU")
        .enumerate()
        .map(|(dist, body)| (body.name, dist))
        .collect::<HashMap<_, _>>();

    parent_iter(&bodies, "SAN")
        .enumerate()
        .filter_map(|(dist2, body)| visited.get(&body.name).map(|dist1| dist1 + dist2))
        .next()
        // The distance should not include YOU and SAN themselves
        .map(|x| x - 2)
        .ok_or(AoCError::NoSolution)
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
