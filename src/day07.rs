use crate::intcode::{
    util::{parse_intcode, read_from_iter, write_once},
    IoOperation, VM,
};
use itertools::Itertools;
use std::collections::VecDeque;
module!(pt1: parse_intcode, pt2: parse_intcode);

fn pt1(memory: Vec<i64>) -> Result<String> {
    let mut max = std::i64::MIN;
    let mut max_phases = Vec::new();
    for phases in (0..=4).permutations(5) {
        let mut last_output = 0;
        for &phase in &phases {
            let mut output = None;
            let mut vm = VM::new(memory.clone());
            vm.run_all(
                read_from_iter([phase, last_output].iter().cloned()),
                write_once(&mut output),
            )?;
            last_output =
                output.ok_or(AoCError::Logic("intcode program did not produce an output"))?;
        }

        if last_output > max {
            max = last_output;
            max_phases = phases;
        }
    }
    Ok(format!(
        "{} (from {})",
        max,
        max_phases.into_iter().join(",")
    ))
}

fn pt2(memory: Vec<i64>) -> Result<String> {
    let mut max = std::i64::MIN;
    let mut max_phases = Vec::new();

    for phases in (5..=9).permutations(5) {
        let output = compute_output_pt2(&memory, &phases)?;
        if output > max {
            max = output;
            max_phases = phases;
        }
    }
    Ok(format!(
        "{} (from {})",
        max,
        max_phases.into_iter().join(",")
    ))
}

fn compute_output_pt2(memory: &Vec<i64>, phases: &[i64]) -> Result<i64> {
    let mut inputs = vec![VecDeque::new(); 5];
    let mut amplifiers = vec![VM::new(memory.clone()); 5];

    for (i, &phase) in phases.iter().enumerate() {
        inputs[i].push_back(phase);
    }
    inputs[0].push_back(0);

    loop {
        let mut still_running = false;
        for (idx, amplifier) in amplifiers.iter_mut().enumerate() {
            still_running |= !amplifier.run_all_async(|io_op| {
                match io_op {
                    IoOperation::Read(out) => *out = inputs[idx].pop_front(),
                    IoOperation::Write(value) => inputs[(idx + 1) % 5].push_back(value),
                }
                Ok(())
            })?;
        }
        if !still_running {
            break;
        }
    }

    if inputs[0].len() != 1 || inputs[1..].iter().any(|v| v.len() != 0) {
        return Err(AoCError::Logic(
            "intcode program did not properly halt with only one output remaining",
        ));
    }
    Ok(inputs[0][0])
}

#[test]
fn day07() -> Result<()> {
    assert_eq!(
        pt1(vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0
        ])?,
        "43210 (from 4,3,2,1,0)"
    );
    assert_eq!(
        pt1(vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0
        ])?,
        "54321 (from 0,1,2,3,4)"
    );
    assert_eq!(
        pt1(vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
        ])?,
        "65210 (from 1,0,4,3,2)"
    );

    assert_eq!(
        pt2(vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5
        ])?,
        "139629729 (from 9,8,7,6,5)"
    );
    assert_eq!(
        pt2(vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
        ])?,
        "18216 (from 9,7,8,5,6)"
    );
    Ok(())
}
