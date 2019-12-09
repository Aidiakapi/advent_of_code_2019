module!(pt1: parse, pt2: parse);

use crate::intcode::{
    util::{reading_not_supported, write_single_value},
    Error::WritingNotSupported,
    Value, VM,
};

fn pt1(memory: Vec<Value>) -> Result<Value> {
    let mut vm = VM::new(memory);
    vm.registers.pending_in = Some(1);

    let mut output = None;
    vm.run_all(reading_not_supported, |value| {
        if output.is_some() {
            Err(WritingNotSupported)
        } else if value == 0 {
            Ok(())
        } else {
            output = Some(value);
            Ok(())
        }
    })?;
    output.ok_or(AoCError::Logic("intcode program wrote no output"))
}

fn pt2(memory: Vec<Value>) -> Result<Value> {
    let mut vm = VM::new(memory);
    vm.registers.pending_in = Some(5);

    let mut output = None;
    vm.run_all(reading_not_supported, write_single_value(&mut output))?;
    output.ok_or(AoCError::Logic("intcode program wrote no output"))
}

fn parse(s: &str) -> IResult<&str, Vec<Value>> {
    use parsers::*;
    separated_list(char(','), i64_str)(s)
}
