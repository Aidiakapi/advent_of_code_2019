#![allow(dead_code)]

use num::ToPrimitive;
use thiserror::Error;
use std::ops::Range;

pub type Value = i64;
pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unknown opcode ({0})")]
    UnknownOpcode(Value),
    #[error("index out of range ({0})")]
    IndexOutOfRange(Value),
    #[error("range out of range ({0:?})")]
    RangeOutOfRange(Range<Value>),
}

pub fn run_intcode<I: InstructionSet, M: Memory>(m: &mut M, mut ip: Value) -> Result<Value> {
    while I::execute(m, &mut ip)? {}
    Ok(ip)
}

pub trait InstructionSet {
    fn execute<M: Memory>(m: &mut M, ip: &mut Value) -> Result<bool>;
}

pub trait Instruction {
    const OPCODE: Value;
    fn execute<M: Memory>(m: &mut M, ip: &mut Value) -> Result<bool>;
}

macro_rules! instruction_set_for_tuple {
    ($($t:ident),+) => {
        #[allow(unused_parens)]
        impl<$($t: Instruction),+> InstructionSet for ($($t),+) {
            fn execute<M: Memory>(m: &mut M, ip: &mut Value) -> Result<bool> {
                let opcode = *m.access(*ip)?;
                $(if opcode == $t::OPCODE {
                    return $t::execute(m, ip);
                })+
                Err(Error::UnknownOpcode(opcode))
            }
        }
    };
}
// instruction_set_for_tuple!(A);
// instruction_set_for_tuple!(A, B);
instruction_set_for_tuple!(A, B, C);
// instruction_set_for_tuple!(A, B, C, D);
// instruction_set_for_tuple!(A, B, C, D, E);

pub trait Memory {
    fn access(&self, idx: Value) -> Result<&Value>;
    fn access_mut(&mut self, idx: Value) -> Result<&mut Value>;
    fn range(&self, range: Range<Value>) -> Result<&[Value]>;
    fn range_mut(&mut self, range: Range<Value>) -> Result<&mut [Value]>;
}

macro_rules! access_and_range {
    ($access_name:ident, $range_name:ident, $($borrow:tt)+) => {
        fn $access_name($($borrow)+self, idx: Value) -> Result<$($borrow)+Value> {
            if let Some(idx) = idx.to_usize() {
                if idx < self.len() {
                    return Ok($($borrow)+self[idx])
                }
            }
            Err(Error::IndexOutOfRange(idx))
        }
        fn $range_name($($borrow)+self, range: Range<Value>) -> Result<$($borrow)+[Value]> {
            if let (Some(start), Some(end)) = (range.start.to_usize(), range.end.to_usize()) {
                if end <= self.len() {
                    return Ok($($borrow)+self[start..end])
                }
            }
            Err(Error::RangeOutOfRange(range))
        }
    };
}
impl Memory for Vec<Value> {
    access_and_range!(access, range, &);
    access_and_range!(access_mut, range_mut, &mut);
}
