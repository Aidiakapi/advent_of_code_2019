use crate::intcode::{
    ascii::{Ascii, AsciiOp},
    growing_memory,
    util::{parse_intcode, write_once},
    Value, VM,
};

module!(pt1: parse_intcode, pt2: parse_intcode);

// Will jump over a gap if there's a spot to land on
// (!1 || !2 || !3) && 4
const PT1_INSTRUCTIONS: &'static str = "\
OR A J
AND B J
AND C J
NOT J J
AND D J
WALK
";

// Will jump over a gap if there's a spot to land on
// and after landing, it can either immediately jump (8)
// or walk forward once and jump (5 && 9)
// or walk forward twice (5 && 6).
//
// The AND A T, NOT T T, OR A T, resets T to true.
//
// (8 || (5 && (6 || 9))) && 4 && (!3 || !2 || !1)
const PT2_INSTRUCTIONS: &'static str = "\
NOT J J
AND E J
OR F T
OR I T
AND T J
OR H J
AND D J
AND A T
NOT T T
OR A T
AND C T
AND B T
AND A T
NOT T T
AND T J
RUN
";

fn solve(memory: Vec<Value>, instructions: &'static str) -> Result<Value> {
    let mut vm = VM::new(growing_memory(memory));
    let mut output = None;
    vm.run_ascii(|op| match op {
        AsciiOp::Read(out) => {
            out.push_str(instructions);
            Ok(())
        }
        AsciiOp::WriteAscii(_) => Ok(()),
        AsciiOp::Write(value) => write_once(&mut output)(value),
    })?;

    output.ok_or(AoCError::NoSolution)
}

fn pt1(memory: Vec<Value>) -> Result<Value> {
    solve(memory, PT1_INSTRUCTIONS)
}
fn pt2(memory: Vec<Value>) -> Result<Value> {
    solve(memory, PT2_INSTRUCTIONS)
}
