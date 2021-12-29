use crate::graph::dfs;
use crate::HashMap;
use std::cmp::Ordering;
use std::collections::hash_map::Entry;

module!(pt1: parse, pt2: parse);

fn calculate_ore_cost<'s>(
    transformations: &Transformations<'s>,
    fuel_goal: u64,
    spare: &mut HashMap<&'s str, u64>,
) -> u64 {
    spare.clear();
    let mut ore_demand = 0;
    dfs(Molecule(fuel_goal, "FUEL"), |mut molecule| {
        match spare.entry(molecule.1) {
            Entry::Vacant(_) => {}
            Entry::Occupied(mut slot) => {
                if *slot.get() < molecule.0 {
                    molecule.0 -= slot.remove();
                } else {
                    *slot.get_mut() -= molecule.0;
                    molecule.0 = 0;
                }
            }
        }
        if molecule.1 == "ORE" {
            ore_demand += molecule.0;
        }
        let transformation = transformations.get(molecule.1).unwrap();
        let produced_per_transformation = transformation.into.0;
        let required_transformations =
            (molecule.0 + produced_per_transformation - 1) / produced_per_transformation;
        let spare_count = (required_transformations * produced_per_transformation) - molecule.0;
        if spare_count > 0 {
            *spare.entry(molecule.1).or_insert(0) += spare_count;
        }
        transformation
            .from
            .iter()
            .map(move |molecule| Molecule(required_transformations * molecule.0, molecule.1))
            .filter(|molecule| molecule.0 > 0)
    });
    ore_demand
}

fn pt1(transformations: Transformations) -> u64 {
    let mut spare = HashMap::new();
    calculate_ore_cost(&transformations, 1, &mut spare)
}

/// Calculates the range of fuel produced in which pt2's output will reside
/// returns (low, high) where low is an inclusive lower bound, and high is
/// and exclusive upper bound.
/// Runs in O(log n).
fn find_fuel_produced_range<'s>(
    transformations: &Transformations<'s>,
    spare: &mut HashMap<&'s str, u64>,
) -> (u64, u64) {
    let mut low = 0;
    let mut high = 1;

    loop {
        let ore_cost = calculate_ore_cost(transformations, high, spare);
        match ore_cost.cmp(&1_000_000_000_000) {
            Ordering::Less => {
                low = high;
                high *= 2;
            }
            Ordering::Equal => break (high, high + 1),
            Ordering::Greater => break (low, high),
        }
    }
}

fn pt2(transformations: Transformations) -> u64 {
    let mut spare = HashMap::new();
    let (mut low, mut high) = find_fuel_produced_range(&transformations, &mut spare);

    // O(log n) binary search within the range
    loop {
        match high - low {
            0 | 1 => break low,
            _ => {}
        }
        let midpoint = (high - low) / 2 + low;
        let ore_cost = calculate_ore_cost(&transformations, midpoint, &mut spare);
        match ore_cost.cmp(&1_000_000_000_000) {
            Ordering::Less => {
                low = midpoint;
            }
            Ordering::Equal => break midpoint,
            Ordering::Greater => {
                high = midpoint;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Molecule<'s>(u64, &'s str);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Transformation<'s> {
    from: Vec<Molecule<'s>>,
    into: Molecule<'s>,
}

type Transformations<'s> = HashMap<&'s str, Transformation<'s>>;

fn parse(s: &str) -> IResult<&str, Transformations> {
    use parsers::*;
    fn molecule(s: &str) -> IResult<&str, Molecule> {
        map(
            pair(terminated(u64_str, char(' ')), alpha1),
            |(count, name)| Molecule(count, name),
        )(s)
    }
    let from = separated_list1(tag(", "), molecule);
    let transformation = map(
        pair(terminated(from, tag(" => ")), molecule),
        |(from, into)| Transformation { from, into },
    );
    fn create_transformations(list: Vec<Transformation>) -> Result<Transformations> {
        let mut transformations = HashMap::with_capacity(list.len());
        for transformation in list {
            match transformations.entry(transformation.into.1) {
                Entry::Occupied(_) => {
                    return Err(AoCError::IncorrectInput(
                        "duplicate ways to create molecule",
                    ));
                }
                Entry::Vacant(slot) => {
                    slot.insert(transformation);
                }
            }
        }
        if transformations.get("FUEL").is_none() {
            return Err(AoCError::IncorrectInput(""));
        }
        transformations.insert(
            "ORE",
            Transformation {
                from: Vec::new(),
                into: Molecule(1, "ORE"),
            },
        );
        Ok(transformations)
    }
    map_res(
        separated_list1(line_ending, transformation),
        create_transformations,
    )(s)
}

#[test]
fn day14() -> Result<()> {
    fn test(input: &str, expected: u64) -> Result<()> {
        assert_eq!(pt1(parse(input)?.1), expected);
        Ok(())
    }

    test(
        "\
10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL",
        31,
    )?;
    test(
        "\
9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL",
        165,
    )?;
    test(
        "\
157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
        13312,
    )?;

    Ok(())
}
