module!(pt1: parse, pt2: parse);
use crate::day02::{Add, Halt, Mul};
use crate::day05::{Equals, Input, JumpIfFalse, JumpIfTrue, LessThan, Output};
use crate::intcode::{
    run_intcode, Instruction, InstructionSet, Memory, ModeIter, Result as icResult, Value,
};
use crate::HashMap;

#[derive(Debug, Clone, Copy, Default)]
struct AdjustRelativeBase;

impl Instruction for AdjustRelativeBase {
    const OPCODE: Value = 9;
    fn execute<M: Memory>(
        &mut self,
        m: &mut M,
        ip: &mut Value,
        mut modes: ModeIter,
    ) -> icResult<bool> {
        let offset = m.read(*ip + 1, modes.next().unwrap()?)?;
        m.set_relative_base(m.get_relative_base()? + offset)?;
        *ip += 2;
        Ok(true)
    }
}

pub fn instruction_set<'io>(
    inputs: &'io mut Vec<Value>,
    outputs: &'io mut Vec<Value>,
) -> impl InstructionSet + 'io {
    (
        Add {},
        Mul {},
        Halt {},
        Input(inputs),
        Output(outputs),
        JumpIfTrue {},
        JumpIfFalse {},
        LessThan {},
        Equals {},
        AdjustRelativeBase {},
    )
}

pub fn to_hash_memory<I>(memory: I) -> HashMap<Value, Value>
where
    I: IntoIterator<Item = Value>,
{
    memory
        .into_iter()
        .enumerate()
        .filter(|&(_, value)| value != 0)
        .map(|(idx, value)| (idx as Value, value))
        .collect()
}

fn pt1(memory: Vec<Value>) -> Result<Value> {
    let memory = to_hash_memory(memory.into_iter());
    let mut inputs = vec![1];
    let mut outputs = Vec::new();
    run_intcode(
        &mut (memory, 0),
        &mut 0,
        instruction_set(&mut inputs, &mut outputs),
    )?;
    if outputs.len() != 1 {
        return Err(AoCError::Logic(
            "program did not return only a single output",
        ));
    }
    Ok(outputs[0])
}

fn pt2(memory: Vec<Value>) -> Result<Value> {
    let memory = to_hash_memory(memory.into_iter());
    let mut inputs = vec![2];
    let mut outputs = Vec::new();
    run_intcode(
        &mut (memory, 0),
        &mut 0,
        instruction_set(&mut inputs, &mut outputs),
    )?;
    if outputs.len() != 1 {
        return Err(AoCError::Logic(
            "program did not return only a single output",
        ));
    }
    Ok(outputs[0])
}

fn parse(s: &str) -> IResult<&str, Vec<Value>> {
    use parsers::*;
    separated_list(char(','), i64_str)(s)
}

#[test]
fn day09() -> Result<()> {
    let memory = to_hash_memory(
        [
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ]
        .iter()
        .cloned(),
    );
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    run_intcode(
        &mut (memory, 0),
        &mut 0,
        instruction_set(&mut inputs, &mut outputs),
    )?;
    assert_eq!(
        outputs,
        vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
    );

    Ok(())
}
