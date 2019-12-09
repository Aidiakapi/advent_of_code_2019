#![allow(dead_code)]

use crate::HashMap;
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
    #[error("mode not supported ({0:?})")]
    ModeNotSupported(Mode),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Position,
    Immediate,
    Relative,
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
            2 => Ok(Mode::Relative),
            x => Err(Error::InvalidParameterMode(x)),
        })
    }
}

pub fn run_intcode<I: InstructionSet, M: Memory>(
    m: &mut M,
    ip: &mut Value,
    mut instruction_set: I,
) -> Result<()> {
    while instruction_set.execute(m, ip)? {}
    Ok(())
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
instruction_set_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J);

pub trait Memory {
    fn read(&self, idx: Value, mode: Mode) -> Result<Value>;
    fn write(&mut self, idx: Value, mode: Mode, value: Value) -> Result<()>;

    fn get_relative_base(&self) -> Result<Value>;
    fn set_relative_base(&mut self, value: Value) -> Result<()>;
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
            Mode::Relative => Err(Error::ModeNotSupported(Mode::Relative)),
        }
    }
    fn write(&mut self, idx: Value, mode: Mode, value: Value) -> Result<()> {
        if mode != Mode::Position {
            return Err(Error::ModeNotSupported(mode));
        }
        let position = self[as_usize(self, idx)?];
        let position = as_usize(self, position)?;
        self[position] = value;
        Ok(())
    }

    fn get_relative_base(&self) -> Result<Value> {
        Err(Error::ModeNotSupported(Mode::Relative))
    }
    fn set_relative_base(&mut self, _value: Value) -> Result<()> {
        Err(Error::ModeNotSupported(Mode::Relative))
    }
}

impl Memory for (Vec<Value>, Value) {
    fn read(&self, idx: Value, mode: Mode) -> Result<Value> {
        let value = self.0[as_usize(&self.0, idx)?];
        match mode {
            Mode::Immediate => Ok(value),
            Mode::Position => self.read(value, Mode::Immediate),
            Mode::Relative => self.read(value + self.1, Mode::Immediate),
        }
    }
    fn write(&mut self, idx: Value, mode: Mode, value: Value) -> Result<()> {
        match mode {
            Mode::Immediate => Err(Error::ModeNotSupported(mode)),
            Mode::Position => {
                let position = self.0[as_usize(&self.0, idx)?];
                let position = as_usize(&self.0, position)?;
                self.0[position] = value;
                Ok(())
            }
            Mode::Relative => {
                let position = self.0[as_usize(&self.0, idx)?];
                let position = as_usize(&self.0, position + self.1)?;
                self.0[position] = value;
                Ok(())
            }
        }
    }

    fn get_relative_base(&self) -> Result<Value> {
        Ok(self.1)
    }
    fn set_relative_base(&mut self, value: Value) -> Result<()> {
        self.1 = value;
        Ok(())
    }
}

impl Memory for (HashMap<Value, Value>, Value) {
    fn read(&self, idx: Value, mode: Mode) -> Result<Value> {
        if idx < 0 {
            return Err(Error::IndexOutOfRange(idx));
        }
        let value = self.0.get(&idx).cloned().unwrap_or(0);
        match mode {
            Mode::Immediate => Ok(value),
            Mode::Position => self.read(value, Mode::Immediate),
            Mode::Relative => self.read(value + self.1, Mode::Immediate),
        }
    }
    
    fn write(&mut self, idx: Value, mode: Mode, value: Value) -> Result<()> {
        if idx < 0 {
            return Err(Error::IndexOutOfRange(idx));
        }
        let idx = self.0.get(&idx).cloned().unwrap_or(0);
        let idx = match mode {
            Mode::Immediate => return Err(Error::ModeNotSupported(mode)),
            Mode::Position => idx,
            Mode::Relative => idx + self.1,
        };
        if value == 0 {
            self.0.remove(&idx);
        } else {
            self.0.insert(idx, value);
        }
        Ok(())
    }
    
    fn get_relative_base(&self) -> Result<Value> {
        Ok(self.1)
    }
    fn set_relative_base(&mut self, value: Value) -> Result<()> {
        self.1 = value;
        Ok(())
    }
}
