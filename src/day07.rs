module!(pt1: parse, pt2: parse);
use crate::day02::{Add, Halt, Mul};
use crate::day05::{Equals, Input, JumpIfFalse, JumpIfTrue, LessThan, Output};
use crate::intcode::{self, run_intcode};
use itertools::Itertools;

fn pt1(memory: Vec<i64>) -> Result<String> {
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    let mut max = std::i64::MIN;
    let mut max_permutation = 0;
    for permutation in [0, 1, 2, 3, 4].iter().cloned().permutations(5) {
        debug_assert!(inputs.is_empty());
        debug_assert!(outputs.is_empty());
        outputs.push(0);

        for &phase in &permutation {
            debug_assert!(inputs.is_empty());
            inputs.push(
                outputs
                    .pop()
                    .ok_or(AoCError::Logic("intcode program didn't produce an output"))?,
            );
            inputs.push(phase);
            debug_assert!(outputs.is_empty());

            let mut memory = memory.clone();
            run_intcode(
                &mut memory,
                &mut 0,
                (
                    Add {},
                    Mul {},
                    Halt {},
                    Input(&mut inputs),
                    Output(&mut outputs),
                    JumpIfTrue {},
                    JumpIfFalse {},
                    LessThan {},
                    Equals {},
                ),
            )?;
        }

        let output = outputs
            .pop()
            .ok_or(AoCError::Logic("intcode program didn't produce an output"))?;

        if output >= max {
            max = output;
            max_permutation = permutation[0] * 10000
                + permutation[1] * 1000
                + permutation[2] * 100
                + permutation[3] * 10
                + permutation[4];
        }
    }
    Ok(format!("{} (from {:0>5})", max, max_permutation))
}

fn pt2(memory: Vec<i64>) -> Result<String> {
    let mut max = std::i64::MIN;
    let mut max_permutation = 0;

    for permutation in [5, 6, 7, 8, 9].iter().cloned().permutations(5) {
        let output = compute_output_pt2(&memory, &permutation)?;
        if output > max {
            max = output;
            max_permutation = permutation[0] * 10000
                + permutation[1] * 1000
                + permutation[2] * 100
                + permutation[3] * 10
                + permutation[4];
        }
    }
    Ok(format!("{} (from {:0>5})", max, max_permutation))
}

fn compute_output_pt2(memory: &Vec<i64>, permutation: &[i64]) -> Result<i64> {
    #[derive(Debug, Clone)]
    struct Amplifier {
        ip: i64,
        memory: Vec<i64>,
        inputs: Vec<i64>,
        outputs: Vec<i64>,
    }
    let mut amplifiers = vec![
        Amplifier {
            ip: 0,
            memory: memory.clone(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        5
    ];

    fn pass_input(amplifier: &mut Amplifier, value: i64) {
        amplifier.inputs.insert(0, value);
    }

    for (idx, &phase) in permutation.iter().enumerate() {
        pass_input(&mut amplifiers[idx], phase);
    }

    pass_input(&mut amplifiers[0], 0);

    loop {
        let mut had_progress = false;
        for idx in 0..5 {
            let amplifier = &mut amplifiers[idx];
            match run_intcode(
                &mut amplifier.memory,
                &mut amplifier.ip,
                (
                    Add {},
                    Mul {},
                    Halt {},
                    Input(&mut amplifier.inputs),
                    Output(&mut amplifier.outputs),
                    JumpIfTrue {},
                    JumpIfFalse {},
                    LessThan {},
                    Equals {},
                ),
            ) {
                Ok(_) => {},
                Err(intcode::Error::NoInputAvailable) => {},
                x => x?,
            };

            if amplifier.outputs.is_empty() {
                continue;
            }
            
            had_progress = true;
            let next_idx = (idx + 1) % 5;
            for out_idx in 0..amplifier.outputs.len() {
                let value = amplifiers[idx].outputs[out_idx];
                pass_input(&mut amplifiers[next_idx], value);
            }
            amplifiers[idx].outputs.clear();
        }

        if !had_progress {
            if amplifiers[0].inputs.len() != 1 || amplifiers.iter().skip(1).any(|amplifier| amplifier.inputs.len() != 0) {
                break Err(AoCError::Logic("incorrect passing of arguments between amplifiers"));
            }
            break Ok(amplifiers[0].inputs[0]);
        }
    }
}

fn parse(s: &str) -> IResult<&str, Vec<i64>> {
    use parsers::*;
    separated_list(char(','), i64_str)(s)
}

#[test]
fn day07() -> Result<()> {
    assert_eq!(
        pt1(vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0
        ])?,
        "43210 (from 43210)"
    );
    assert_eq!(
        pt1(vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0
        ])?,
        "54321 (from 01234)"
    );
    assert_eq!(
        pt1(vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
        ])?,
        "65210 (from 10432)"
    );

    assert_eq!(
        pt2(vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5
        ])?,
        "139629729 (from 98765)"
    );
    assert_eq!(
        pt2(vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
        ])?,
        "18216 (from 97856)"
    );
    Ok(())
}
