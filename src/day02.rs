module!(pt1: parse, pt2: parse);

use crate::intcode::{run_intcode, Instruction, Memory, ModeIter, Result as icResult, Value};

#[derive(Debug, Clone, Copy, Default)]
pub struct Add;
#[derive(Debug, Clone, Copy, Default)]
pub struct Mul;
#[derive(Debug, Clone, Copy, Default)]
pub struct Halt;
impl Instruction for Add {
    const OPCODE: Value = 1;
    fn execute<M: Memory>(
        &mut self,
        m: &mut M,
        ip: &mut Value,
        mut modes: ModeIter,
    ) -> icResult<bool> {
        let a = m.read(*ip + 1, modes.next().unwrap()?)?;
        let b = m.read(*ip + 2, modes.next().unwrap()?)?;
        m.write(*ip + 3, a + b)?;
        *ip += 4;
        Ok(true)
    }
}
impl Instruction for Mul {
    const OPCODE: Value = 2;
    fn execute<M: Memory>(
        &mut self,
        m: &mut M,
        ip: &mut Value,
        mut modes: ModeIter,
    ) -> icResult<bool> {
        let a = m.read(*ip + 1, modes.next().unwrap()?)?;
        let b = m.read(*ip + 2, modes.next().unwrap()?)?;
        m.write(*ip + 3, a * b)?;
        *ip += 4;
        Ok(true)
    }
}
impl Instruction for Halt {
    const OPCODE: Value = 99;
    fn execute<M: Memory>(&mut self, _m: &mut M, _ip: &mut Value, _: ModeIter) -> icResult<bool> {
        Ok(false)
    }
}

type Instructions = (Add, Mul, Halt);

fn pt1(mut memory: Vec<i64>) -> Result<i64> {
    memory[1] = 12;
    memory[2] = 2;
    run_intcode(&mut memory, &mut 0, Instructions::default())?;
    Ok(memory[0])
}

fn pt2(mut memory: Vec<i64>) -> Result<i64> {
    for noun in 0..=99 {
        memory[1] = noun;
        for verb in 0..=99 {
            memory[2] = verb;
            let mut memory = memory.clone();
            run_intcode(&mut memory, &mut 0, Instructions::default())?;
            if memory[0] == 19690720 {
                return Ok(noun * 100 + verb);
            }
        }
    }
    Err(AoCError::NoSolution)
}

fn parse(s: &str) -> IResult<&str, Vec<i64>> {
    use parsers::*;
    separated_list(char(','), i64_str)(s)
}

#[test]
fn day02() -> Result<()> {
    let mut memory = vec![1, 0, 0, 0, 99];
    run_intcode(&mut memory, &mut 0, Instructions::default())?;
    assert_eq!(memory, vec![2, 0, 0, 0, 99]);

    memory = vec![2, 3, 0, 3, 99];
    run_intcode(&mut memory, &mut 0, Instructions::default())?;
    assert_eq!(memory, vec![2, 3, 0, 6, 99]);

    memory = vec![2, 4, 4, 5, 99, 0];
    run_intcode(&mut memory, &mut 0, Instructions::default())?;
    assert_eq!(memory, vec![2, 4, 4, 5, 99, 9801]);

    memory = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    run_intcode(&mut memory, &mut 0, Instructions::default())?;
    assert_eq!(memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);

    Ok(())
}
