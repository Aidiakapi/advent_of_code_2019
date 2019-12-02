module!(pt1: parse, pt2: parse);

use crate::intcode::{run_intcode, Instruction, Memory, Value, Result as icResult};

struct Add;
struct Mul;
struct Halt;
impl Instruction for Add {
    const OPCODE: Value = 1;
    fn execute<M: Memory>(m: &mut M, ip: &mut Value) -> icResult<bool> {
        let operands = m.range(*ip + 1..*ip + 4)?;
        let (a, b, c) = (operands[0], operands[1], operands[2]);
        *m.access_mut(c)? = *m.access(a)? + *m.access(b)?;
        *ip += 4;
        Ok(true)
    }
}
impl Instruction for Mul {
    const OPCODE: Value = 2;
    fn execute<M: Memory>(m: &mut M, ip: &mut Value) -> icResult<bool> {
        let operands = m.range(*ip + 1..*ip + 4)?;
        let (a, b, c) = (operands[0], operands[1], operands[2]);
        *m.access_mut(c)? = *m.access(a)? * *m.access(b)?;
        *ip += 4;
        Ok(true)
    }
}
impl Instruction for Halt {
    const OPCODE: Value = 99;
    fn execute<M: Memory>(_m: &mut M, _ip: &mut Value) -> icResult<bool> {
        Ok(false)
    }
}

type Instructions = (Add, Mul, Halt);

fn pt1(mut memory: Vec<i64>) -> Result<i64> {
    memory[1] = 12;
    memory[2] = 2;
    run_intcode::<Instructions, _>(&mut memory, 0)?;
    Ok(memory[0])
}

fn pt2(mut memory: Vec<i64>) -> Result<i64> {
    for noun in 0..=99 {
        memory[1] = noun;
        for verb in 0..=99 {
            memory[2] = verb;
            let mut memory = memory.clone();
            run_intcode::<Instructions, _>(&mut memory, 0)?;
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
