module!(pt1: parse, pt2: parse);

use crate::intcode::{Memory, Value, VM};

fn pt1(mut memory: Vec<Value>) -> Result<Value> {
    memory[1] = 12;
    memory[2] = 2;
    let mut vm = VM::new(memory);
    vm.run_all_no_io()?;
    Ok(Memory::read(&vm.memory, 0)?)
}

fn pt2(mut memory: Vec<Value>) -> Result<Value> {
    for noun in 0..=99 {
        memory[1] = noun;
        for verb in 0..=99 {
            memory[2] = verb;
            let mut vm = VM::new(memory.clone());
            vm.run_all_no_io()?;
            if Memory::read(&vm.memory, 0)? == 19690720 {
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
    use crate::intcode;
    fn test_example(input: Vec<Value>, output: Vec<Value>) -> Result<()> {
        let mut vm = VM::new(input);
        vm.run_all_no_io()?;
        assert_eq!(
            output,
            (0..output.len())
                .map(|i| Memory::read(&vm.memory, i as Value))
                .collect::<intcode::Result<Vec<_>>>()?
        );
        Ok(())
    }
    test_example(vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99])?;
    test_example(vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99])?;
    test_example(vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801])?;
    test_example(
        vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
        vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
    )?;

    Ok(())
}
