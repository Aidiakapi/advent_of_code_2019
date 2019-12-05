#![allow(dead_code)]

use num::ToPrimitive;
use std::ops::Range;
use thiserror::Error;

pub type Value = i64;
pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unknown instruction ({0})")]
    UnknownInstruction(Value),
    #[error("invalid parameter mode ({0})")]
    InvalidParameterMode(Value),
    #[error("index out of range ({0})")]
    IndexOutOfRange(Value),
    #[error("range out of range ({0:?})")]
    RangeOutOfRange(Range<Value>),
    #[error("no input available")]
    NoInputAvailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Position,
    Immediate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModeIter(Value);
impl Iterator for ModeIter {
    type Item = Result<Mode>;
    fn next(&mut self) -> Option<Result<Mode>> {
        let mode = self.0 % 10;
        self.0 /= 10;
        Some(match mode {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            x => Err(Error::InvalidParameterMode(x)),
        })
    }
}

pub fn run_intcode<I: InstructionSet, M: Memory>(
    m: &mut M,
    mut ip: Value,
    mut instruction_set: I,
) -> Result<Value> {
    while instruction_set.execute(m, &mut ip)? {}
    Ok(ip)
}

pub trait InstructionSet {
    fn execute<M: Memory>(&mut self, m: &mut M, ip: &mut Value) -> Result<bool>;
}

pub trait Instruction {
    const OPCODE: Value;
    fn execute<M: Memory>(
        &mut self,
        m: &mut M,
        ip: &mut Value,
        param_modes: ModeIter,
    ) -> Result<bool>;
}

macro_rules! instruction_set_for_tuple {
    ($($nr:tt:$t:ident),+) => {
        #[allow(unused_parens)]
        impl<$($t: Instruction),+> InstructionSet for ($($t),+) {
            fn execute<M: Memory>(&mut self, m: &mut M, ip: &mut Value) -> Result<bool> {
                let opcode = m.read(*ip, Mode::Immediate)?;
                let instruction = opcode % 100;
                let param_modes = ModeIter(opcode / 100);
                $(if instruction == $t::OPCODE {
                    return self.$nr.execute(m, ip, param_modes);
                })+
                Err(Error::UnknownInstruction(instruction))
            }
        }
    };
}
// instruction_set_for_tuple!(0:A);
// instruction_set_for_tuple!(0:A, 1:B);
instruction_set_for_tuple!(0: A, 1: B, 2: C);
// instruction_set_for_tuple!(0:A, 1:B, 2:C, 3:D);
instruction_set_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E);
instruction_set_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I);

pub trait Memory {
    fn read(&self, idx: Value, mode: Mode) -> Result<Value>;
    fn write(&mut self, idx: Value, value: Value) -> Result<()>;
}

fn as_usize(vec: &Vec<Value>, idx: Value) -> Result<usize> {
    match idx.to_usize() {
        Some(idx) if idx < vec.len() => Ok(idx),
        _ => Err(Error::IndexOutOfRange(idx)),
    }
}

impl Memory for Vec<Value> {
    fn read(&self, idx: Value, mode: Mode) -> Result<Value> {
        let value = self[as_usize(self, idx)?];
        match mode {
            Mode::Immediate => Ok(value),
            Mode::Position => self.read(value, Mode::Immediate),
        }
    }
    fn write(&mut self, idx: Value, value: Value) -> Result<()> {
        let position = self[as_usize(self, idx)?];
        let position = as_usize(self, position)?;
        self[position] = value;
        Ok(())
    }
}
