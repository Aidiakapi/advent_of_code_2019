module!(pt1: parse, pt2: parse);

use crate::day02;
use crate::intcode::{
    run_intcode, Error, Instruction, InstructionSet, Memory, ModeIter, Result as icResult, Value,
};

#[derive(Debug)]
pub struct Input<'io>(pub &'io mut Vec<Value>);
#[derive(Debug)]
pub struct Output<'io>(pub &'io mut Vec<Value>);
#[derive(Debug, Clone, Copy, Default)]
pub struct JumpIfTrue;
#[derive(Debug, Clone, Copy, Default)]
pub struct JumpIfFalse;
#[derive(Debug, Clone, Copy, Default)]
pub struct LessThan;
#[derive(Debug, Clone, Copy, Default)]
pub struct Equals;

impl Instruction for Input<'_> {
    const OPCODE: Value = 3;
    fn execute<M: Memory>(
        &mut self,
        m: &mut M,
        ip: &mut Value,
        mut modes: ModeIter,
    ) -> icResult<bool> {
        let value = self.0.pop().ok_or(Error::NoInputAvailable)?;
        m.write(*ip + 1, modes.next().unwrap()?, value)?;
        *ip += 2;
        Ok(true)
    }
}

impl Instruction for Output<'_> {
    const OPCODE: Value = 4;
    fn execute<M: Memory>(
        &mut self,
        m: &mut M,
        ip: &mut Value,
        mut modes: ModeIter,
    ) -> icResult<bool> {
        let value = m.read(*ip + 1, modes.next().unwrap()?)?;
        self.0.push(value);
        *ip += 2;
        Ok(true)
    }
}

impl Instruction for JumpIfTrue {
    const OPCODE: Value = 5;
    fn execute<M: Memory>(
        &mut self,
        m: &mut M,
        ip: &mut Value,
        mut modes: ModeIter,
    ) -> icResult<bool> {
        let condition = m.read(*ip + 1, modes.next().unwrap()?)?;
        if condition == 0 {
            *ip += 3;
        } else {
            *ip = m.read(*ip + 2, modes.next().unwrap()?)?;
        }
        Ok(true)
    }
}

impl Instruction for JumpIfFalse {
    const OPCODE: Value = 6;
    fn execute<M: Memory>(
        &mut self,
        m: &mut M,
        ip: &mut Value,
        mut modes: ModeIter,
    ) -> icResult<bool> {
        let condition = m.read(*ip + 1, modes.next().unwrap()?)?;
        if condition != 0 {
            *ip += 3;
        } else {
            *ip = m.read(*ip + 2, modes.next().unwrap()?)?;
        }
        Ok(true)
    }
}

impl Instruction for LessThan {
    const OPCODE: Value = 7;
    fn execute<M: Memory>(
        &mut self,
        m: &mut M,
        ip: &mut Value,
        mut modes: ModeIter,
    ) -> icResult<bool> {
        let a = m.read(*ip + 1, modes.next().unwrap()?)?;
        let b = m.read(*ip + 2, modes.next().unwrap()?)?;
        m.write(*ip + 3, modes.next().unwrap()?, if a < b { 1 } else { 0 })?;
        *ip += 4;
        Ok(true)
    }
}

impl Instruction for Equals {
    const OPCODE: Value = 8;
    fn execute<M: Memory>(
        &mut self,
        m: &mut M,
        ip: &mut Value,
        mut modes: ModeIter,
    ) -> icResult<bool> {
        let a = m.read(*ip + 1, modes.next().unwrap()?)?;
        let b = m.read(*ip + 2, modes.next().unwrap()?)?;
        m.write(*ip + 3, modes.next().unwrap()?, if a == b { 1 } else { 0 })?;
        *ip += 4;
        Ok(true)
    }
}

pub fn instruction_set<'io>(
    inputs: &'io mut Vec<Value>,
    outputs: &'io mut Vec<Value>,
) -> impl InstructionSet + 'io {
    (
        day02::Add {},
        day02::Mul {},
        day02::Halt {},
        Input(inputs),
        Output(outputs),
        JumpIfTrue {},
        JumpIfFalse {},
        LessThan {},
        Equals {},
    )
}

fn pt1(mut memory: Vec<Value>) -> Result<Value> {
    let mut inputs = vec![1];
    let mut outputs = Vec::new();

    run_intcode(
        &mut memory,
        &mut 0,
        (
            day02::Add {},
            day02::Mul {},
            day02::Halt {},
            Input(&mut inputs),
            Output(&mut outputs),
        ),
    )?;

    if outputs.len() == 0 {
        return Err(AoCError::NoSolution);
    }
    if !outputs[0..outputs.len() - 1].iter().all(|&x| x == 0) {
        return Err(AoCError::Logic("expected all zeroes until final output"));
    }
    Ok(*outputs.last().unwrap())
}

fn pt2(mut memory: Vec<Value>) -> Result<Value> {
    let mut inputs = vec![5];
    let mut outputs = Vec::new();

    run_intcode(
        &mut memory,
        &mut 0,
        instruction_set(&mut inputs, &mut outputs),
    )?;

    if outputs.len() != 1 {
        return Err(AoCError::Logic("expected program to have 1 output"));
    }
    Ok(outputs[0])
}

fn parse(s: &str) -> IResult<&str, Vec<Value>> {
    use parsers::*;
    separated_list(char(','), i64_str)(s)
}
