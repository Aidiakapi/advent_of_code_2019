module!(pt1: parse, pt2: parse);
use crate::intcode::{
    sparse_memory,
    util::{read_from_iter, write_single_value},
    Value, VM,
};
use std::iter::once;

fn run_program(memory: Vec<Value>, input: Value) -> Result<Value> {
    let mut vm = VM::new(sparse_memory(memory));
    let mut output = None;
    vm.run_all(read_from_iter(once(input)), write_single_value(&mut output))?;

    output.ok_or(AoCError::Logic("intcode program didn't return a value"))
}

fn pt1(memory: Vec<Value>) -> Result<Value> {
    run_program(memory, 1)
}
fn pt2(memory: Vec<Value>) -> Result<Value> {
    run_program(memory, 2)
}

fn parse(s: &str) -> IResult<&str, Vec<Value>> {
    use parsers::*;
    separated_list(char(','), i64_str)(s)
}

#[test]
fn day09() -> Result<()> {
    use crate::intcode::util::reading_not_supported;
    fn run(input: &[Value]) -> Result<Vec<Value>> {
        let mut vm = VM::new(sparse_memory(input.iter().cloned()));
        let mut output = Vec::new();
        vm.run_all(reading_not_supported, |value| {
            output.push(value);
            Ok(())
        })?;
        Ok(output)
    }

    assert_eq!(
        &[109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,],
        run(&[109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,])?
            .as_slice()
    );

    let value = run(&[1102, 34915192, 34915192, 7, 4, 7, 99, 0])?;
    assert!(value.len() == 1 && value[0] >= 1000_0000_0000_0000 && value[0] <= 9999_9999_9999_9999);

    assert_eq!(
        &[1125899906842624],
        run(&[104, 1125899906842624, 99])?.as_slice()
    );

    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;
    use ::test::{black_box, Bencher};

    #[bench]
    fn day09_pt1(b: &mut Bencher) {
        let input = std::fs::read_to_string("./data/day09.txt").unwrap();
        let input = input.trim();
        b.iter(|| pt1(parse(black_box(input)).unwrap().1));
    }

    #[bench]
    fn day09_pt2(b: &mut Bencher) {
        let input = std::fs::read_to_string("./data/day09.txt").unwrap();
        let input = input.trim();
        b.iter(|| pt2(parse(black_box(input)).unwrap().1));
    }
}
