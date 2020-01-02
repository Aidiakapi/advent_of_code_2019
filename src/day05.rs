use crate::intcode::{
    util::{parse_intcode, reading_not_supported, write_once},
    Error::WritingNotSupported,
    Value, VM,
};
module!(pt1: parse_intcode, pt2: parse_intcode);

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
    vm.run_all(reading_not_supported, write_once(&mut output))?;
    output.ok_or(AoCError::Logic("intcode program wrote no output"))
}
