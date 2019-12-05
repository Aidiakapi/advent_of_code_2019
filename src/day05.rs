module!(pt1: parse, pt2: parse);

use crate::day02;
use crate::intcode::{
    run_intcode, Error, Instruction, Memory, ModeIter, Result as icResult, Value,
};

struct Input<'io>(&'io mut Vec<Value>);
struct Output<'io>(&'io mut Vec<Value>);
struct JumpIfTrue;
struct JumpIfFalse;
struct LessThan;
struct Equals;

impl Instruction for Input<'_> {
    const OPCODE: Value = 3;
    fn execute<M: Memory>(&mut self, m: &mut M, ip: &mut Value, _: ModeIter) -> icResult<bool> {
        let value = self.0.pop().ok_or(Error::NoInputAvailable)?;
        m.write(*ip + 1, value)?;
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
        mut mode: ModeIter,
    ) -> icResult<bool> {
        let value = m.read(*ip + 1, mode.next().unwrap()?)?;
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
        m.write(*ip + 3, if a < b { 1 } else { 0 })?;
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
        m.write(*ip + 3, if a == b { 1 } else { 0 })?;
        *ip += 4;
        Ok(true)
    }
}

fn pt1(mut memory: Vec<Value>) -> Result<Value> {
    let mut inputs = vec![1];
    let mut outputs = Vec::new();

    run_intcode(
        &mut memory,
        0,
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
        0,
        (
            day02::Add {},
            day02::Mul {},
            day02::Halt {},
            Input(&mut inputs),
            Output(&mut outputs),
            JumpIfTrue {},
            JumpIfFalse {},
            LessThan {},
            Equals {},
        ),
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
