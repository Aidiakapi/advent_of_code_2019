//! Intcode interpreter spec
//!
//! ```
//! Registers:
//!     IP          Instruction pointer, holds the address of the next instruction to be executed
//!     RelBase     Relative base, used as an offset to an address for parameter mode 2.
//!     PendingIn   Input that's waiting to be consumed by an instruction.
//!     PendingOut  Output that's waiting to be consumed by the host.
//!
//! Opcodes:
//!     1  Add          Reads 2 params, sums, writes 1 param, jumps 4 ahead
//!     2  Mul          Reads 2 params, muls, writes 1 param, jumps 4 ahead
//!     3  Input        Loads input, writes 1 param, jumps 2 ahead
//!     4  Output       Reads 1 param, stores output, jumps 2 ahead
//!     5  JumpIfTrue   Reads 2 params, if first != 0 jump to second, otherwise jumps 3 ahead
//!     6  JumpIfFalse  Reads 2 params, if first == 0 jump to second, otherwise jumps 3 ahead
//!     7  LessThan     Reads 2 params, if first < second { 1 } else { 0 }, writes 1 param, jumps 4 ahead
//!     8  Equals       Reads 2 params, if first == second { 1 } else { 0 }, writes 1 param, jumps 4 ahead
//!     9  AdjRelBase   Reads 1 params, sums with relative base address, jumps 2 ahead
//!    99  Halt         Terminates program
//!
//! Param mode:
//!     0  Position     Parameter is an address of a value.
//!     1  Immediate    Parameter is the value (only for reads).
//!     2  Relative     Parameter is an offset to relative base address, which produces the value.
//!
//! Memory:
//!     Infinite in size, initialized to 0.
//!     Addressed by units of 64-bit signed integers.
//!
//! Operational state:
//!     Idle        Program is ready to start or continue execution.
//!     Halted      Program has completed its execution
//!     Reading     Program is waiting for an input to be provided.
//!     Writing     Program is waiting for an output to be handled.
//!
//! Error handling:
//!     Errors can only occur during decoding or execution of an instruction. This process is done as
//!     an atomic operation, and any error will result in no modifications to any state.
//! ```
#![allow(dead_code)]

use crate::HashMap;
use num::ToPrimitive;
use std::convert::TryFrom;
use std::iter::FromIterator;
use thiserror::Error;

pub type Value = i64;
#[derive(Clone, Error, Debug)]
pub enum Error {
    #[error("invalid opcode ({0})")]
    InvalidOpcode(Value),
    #[error("invalid mode ({0})")]
    InvalidMode(Value),
    #[error("index out of range ({0})")]
    IndexOutOfRange(Value),
    #[error("invalid write mode (immediate mode cannot be used for writing)")]
    InvalidWriteMode,
    #[error("invalid state ({0:?})")]
    InvalidState(State),
    #[error("reading is not supported")]
    ReadingNotSupported,
    #[error("writing is not supported")]
    WritingNotSupported,
    #[error("{0}")]
    Custom(String),
}
pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct VM<M: Memory> {
    pub memory: M,
    pub registers: Registers,
    pub state: State,
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Registers {
    pub ip: Value,
    pub relative_base: Value,
    pub pending_in: Option<Value>,
    pub pending_out: Option<Value>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Idle,
    Halted,
    Reading,
    Writing,
}
impl Default for State {
    fn default() -> Self {
        State::Idle
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    Add = 1,
    Multiply = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    AdjRelBase = 9,
    Halt = 99,
}
impl TryFrom<Value> for Opcode {
    type Error = Error;
    fn try_from(value: Value) -> Result<Opcode> {
        use Opcode::*;
        Ok(match value {
            1 => Add,
            2 => Multiply,
            3 => Input,
            4 => Output,
            5 => JumpIfTrue,
            6 => JumpIfFalse,
            7 => LessThan,
            8 => Equals,
            9 => AdjRelBase,
            99 => Halt,
            _ => return Err(Error::InvalidOpcode(value)),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}
impl TryFrom<Value> for Mode {
    type Error = Error;
    fn try_from(value: Value) -> Result<Mode> {
        use Mode::*;
        Ok(match value {
            0 => Position,
            1 => Immediate,
            2 => Relative,
            _ => return Err(Error::InvalidMode(value)),
        })
    }
}

pub trait Memory: Sized + Clone {
    fn read(&self, idx: Value) -> Result<Value>;
    fn write(&mut self, idx: Value, value: Value) -> Result<()>;
    fn largest_index(&self) -> Value;
}

pub trait MemoryIntoData<T>: Memory + Sized + Clone + Default {
    fn into_data(self) -> T;
}

pub fn fixed_memory<I>(initial: I) -> impl Memory + MemoryIntoData<Vec<Value>>
where
    I: IntoIterator<Item = Value>,
{
    Vec::from_iter(initial)
}

pub fn sparse_memory<I>(initial: I) -> impl Memory + MemoryIntoData<HashMap<Value, Value>>
where
    I: IntoIterator<Item = Value>,
{
    HashMap::from_iter(
        initial
            .into_iter()
            .enumerate()
            .filter(|(_, value)| *value != 0)
            .map(|(idx, value)| (idx as Value, value)),
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GrowingMemory(pub Vec<Value>);

pub fn growing_memory(memory: Vec<Value>) -> GrowingMemory {
    GrowingMemory(memory)
}

impl Memory for Vec<Value> {
    fn read(&self, idx: Value) -> Result<Value> {
        idx.to_usize()
            .and_then(|idx| self.get(idx))
            .cloned()
            .ok_or(Error::IndexOutOfRange(idx))
    }
    fn write(&mut self, idx: Value, value: Value) -> Result<()> {
        idx.to_usize()
            .and_then(|idx| self.get_mut(idx))
            .map(|target| {
                *target = value;
            })
            .ok_or(Error::IndexOutOfRange(idx))
    }
    fn largest_index(&self) -> Value {
        (self.len() as Value) - 1
    }
}
impl MemoryIntoData<Vec<Value>> for Vec<Value> {
    fn into_data(self) -> Self {
        self
    }
}
impl Memory for HashMap<Value, Value> {
    fn read(&self, idx: Value) -> Result<Value> {
        if idx >= 0 { Some(idx) } else { None }
            .map(|idx| self.get(&idx).cloned().unwrap_or(0))
            .ok_or(Error::IndexOutOfRange(idx))
    }
    fn write(&mut self, idx: Value, value: Value) -> Result<()> {
        if idx >= 0 { Some(idx) } else { None }
            .map(|idx| {
                if value == 0 {
                    self.remove(&idx);
                } else {
                    self.insert(idx, value);
                }
            })
            .ok_or(Error::IndexOutOfRange(idx))
    }
    fn largest_index(&self) -> Value {
        self.keys().cloned().max().unwrap_or(-1)
    }
}
impl MemoryIntoData<HashMap<Value, Value>> for HashMap<Value, Value> {
    fn into_data(self) -> Self {
        self
    }
}

impl Memory for GrowingMemory {
    fn read(&self, idx: Value) -> Result<Value> {
        match idx.to_usize() {
            Some(idx) => Ok(self.0.get(idx).cloned().unwrap_or(0)),
            None => Err(Error::IndexOutOfRange(idx)),
        }
    }
    fn write(&mut self, idx: Value, value: Value) -> Result<()> {
        match idx.to_usize() {
            Some(idx) => {
                if idx >= self.0.len() {
                    if value == 0 {
                        return Ok(());
                    } else {
                        self.0.reserve(idx - self.0.len() + 1);
                        self.0.resize(idx + 1, 0);
                    }
                }
                self.0[idx] = value;
                Ok(())
            }
            None => Err(Error::IndexOutOfRange(idx)),
        }
    }
    fn largest_index(&self) -> Value {
        let mut slice = self.0.as_slice();
        while let Some(nr) = slice.last().cloned() {
            if nr != 0 {
                break;
            }
            slice = &slice[0..slice.len() - 1];
        }
        (slice.len() as Value) - 1
    }
}
impl MemoryIntoData<Vec<Value>> for GrowingMemory {
    fn into_data(self) -> Vec<Value> {
        self.0
    }
}

pub enum IoOperation<'v> {
    Read(&'v mut Option<Value>),
    Write(Value),
}

impl<M: Memory> VM<M> {
    pub fn new(memory: M) -> Self {
        Self {
            memory,
            registers: Registers::default(),
            state: State::default(),
        }
    }

    pub fn run_all<FI, FO>(&mut self, mut read: FI, mut write: FO) -> Result<()>
    where
        FI: FnMut() -> Result<Value>,
        FO: FnMut(Value) -> Result<()>,
    {
        self.run_all_async(|io_op| {
            match io_op {
                IoOperation::Read(out) => *out = Some(read()?),
                IoOperation::Write(value) => write(value)?,
            }
            Ok(())
        })
        .map(|_| ())
    }

    pub fn run_all_async<F>(&mut self, mut io: F) -> Result<bool>
    where
        F: FnMut(IoOperation) -> Result<()>,
    {
        loop {
            match self.state {
                State::Idle => self.run_one()?,
                State::Halted => break Ok(true),
                State::Reading => {
                    let mut input = None;
                    io(IoOperation::Read(&mut input))?;
                    if let Some(input) = input {
                        self.registers.pending_in = Some(input);
                        self.state = State::Idle;
                    } else {
                        break Ok(false);
                    }
                }
                State::Writing => {
                    io(IoOperation::Write(self.registers.pending_out.unwrap()))?;
                    self.registers.pending_out = None;
                    self.state = State::Idle;
                }
            }
        }
    }

    pub fn run_all_no_io(&mut self) -> Result<()> {
        self.run_all(util::reading_not_supported, util::writing_not_supported)
    }

    pub fn run_one(&mut self) -> Result<()> {
        if self.state != State::Idle {
            return Err(Error::InvalidState(self.state));
        }

        let ip = self.registers.ip;
        let instruction = self.memory.read(ip)?;
        let opcode = Opcode::try_from(instruction % 100)?;
        let mut modes = instruction / 100;
        let mut pop_mode = || {
            let mode = modes % 10;
            modes /= 10;
            Mode::try_from(mode)
        };

        let new_ip = match opcode {
            Opcode::Add => {
                let a = self.read(ip + 1, pop_mode()?)?;
                let b = self.read(ip + 2, pop_mode()?)?;
                self.write(ip + 3, pop_mode()?, a + b)?;
                ip + 4
            }
            Opcode::Multiply => {
                let a = self.read(ip + 1, pop_mode()?)?;
                let b = self.read(ip + 2, pop_mode()?)?;
                self.write(ip + 3, pop_mode()?, a * b)?;
                ip + 4
            }
            Opcode::Input => {
                if let Some(input) = self.registers.pending_in {
                    self.write(ip + 1, pop_mode()?, input)?;
                    self.registers.pending_in = None;
                    ip + 2
                } else {
                    self.state = State::Reading;
                    ip
                }
            }
            Opcode::Output => {
                self.registers.pending_out = Some(self.read(ip + 1, pop_mode()?)?);
                self.state = State::Writing;
                ip + 2
            }
            Opcode::JumpIfTrue => {
                if self.read(ip + 1, pop_mode()?)? != 0 {
                    self.read(ip + 2, pop_mode()?)?
                } else {
                    ip + 3
                }
            }
            Opcode::JumpIfFalse => {
                if self.read(ip + 1, pop_mode()?)? == 0 {
                    self.read(ip + 2, pop_mode()?)?
                } else {
                    ip + 3
                }
            }
            Opcode::LessThan => {
                let a = self.read(ip + 1, pop_mode()?)?;
                let b = self.read(ip + 2, pop_mode()?)?;
                self.write(ip + 3, pop_mode()?, if a < b { 1 } else { 0 })?;
                ip + 4
            }
            Opcode::Equals => {
                let a = self.read(ip + 1, pop_mode()?)?;
                let b = self.read(ip + 2, pop_mode()?)?;
                self.write(ip + 3, pop_mode()?, if a == b { 1 } else { 0 })?;
                ip + 4
            }
            Opcode::AdjRelBase => {
                self.registers.relative_base += self.read(ip + 1, pop_mode()?)?;
                ip + 2
            }
            Opcode::Halt => {
                self.state = State::Halted;
                ip
            }
        };
        self.registers.ip = new_ip;

        Ok(())
    }

    pub fn read(&self, idx: Value, mode: Mode) -> Result<Value> {
        match mode {
            Mode::Immediate => self.memory.read(idx),
            Mode::Position => self.memory.read(self.memory.read(idx)?),
            Mode::Relative => self
                .memory
                .read(self.memory.read(idx)? + self.registers.relative_base),
        }
    }

    pub fn write(&mut self, idx: Value, mode: Mode, value: Value) -> Result<()> {
        match mode {
            Mode::Immediate => Err(Error::InvalidWriteMode),
            Mode::Position => self.memory.write(self.memory.read(idx)?, value),
            Mode::Relative => self
                .memory
                .write(self.memory.read(idx)? + self.registers.relative_base, value),
        }
    }
}

pub mod util {
    use super::*;

    pub fn reading_not_supported() -> Result<Value> {
        Err(Error::ReadingNotSupported)
    }
    pub fn writing_not_supported(_: Value) -> Result<()> {
        Err(Error::WritingNotSupported)
    }

    pub fn write_single_value<'t>(
        target: &'t mut Option<Value>,
    ) -> impl FnMut(Value) -> Result<()> + 't {
        move |value| {
            if target.is_some() {
                Err(Error::WritingNotSupported)
            } else {
                *target = Some(value);
                Ok(())
            }
        }
    }

    pub trait Batch: Default {
        const N: usize;
        fn store(&mut self, n: usize, value: Value) -> Result<()>;
    }
    pub fn write_batching<B, F>(mut callback: F) -> impl FnMut(Value) -> Result<()>
    where
        B: Batch,
        F: FnMut(B) -> Result<()>,
    {
        let mut n = 0;
        let mut storage = B::default();
        move |value| {
            storage.store(n, value)?;
            if n == B::N - 1 {
                n = 0;
                let mut temp = B::default();
                std::mem::swap(&mut temp, &mut storage);
                callback(temp)
            } else {
                n += 1;
                Ok(())
            }
        }
    }

    macro_rules! replace_tt {
        ($expr:tt, $sub:tt) => {
            $sub
        };
    }
    macro_rules! impl_batch_trait {
        ($count:tt, $($n:tt),+) => {
            impl Batch for ($(replace_tt!($n, Value)),+) {
                const N: usize = $count;

                #[inline]
                fn store(&mut self, n: usize, value: Value) -> Result<()> {
                    match n {
                        $($n => self.$n = value,)+
                        _ => unreachable!(),
                    }
                    Ok(())
                }
            }
        };
    }
    impl_batch_trait!(2, 0, 1);
    impl_batch_trait!(3, 0, 1, 2);
    impl_batch_trait!(4, 0, 1, 2, 3);
    impl_batch_trait!(5, 0, 1, 2, 3, 4);
    impl_batch_trait!(6, 0, 1, 2, 3, 4, 5);

    pub fn read_from_iter<I>(iter: I) -> impl FnMut() -> Result<Value>
    where
        I: IntoIterator<Item = Value>,
    {
        let mut iter = iter.into_iter();
        move || iter.next().ok_or(Error::ReadingNotSupported)
    }

    pub fn parse_intcode(s: &str) -> nom::IResult<&str, Vec<Value>> {
        use crate::parsers::*;
        separated_list(char(','), i64_str)(s)
    }
}

pub mod debugger {
    use super::*;

    pub fn disassemble<M: Memory>(vm: &VM<M>) -> String {
        use arrayvec::ArrayVec;
        use std::fmt::Write;
        macro_rules! out {
            ($($tks:tt)*) => {
                write!($($tks)*).expect("failed to write to string");
            };
        }
        let mut d = String::new();
        out!(d, "Instructions:\nFLG        ADDR        VALUE   DESC\n");

        let mut ptr = 0;
        let max_idx = vm.memory.largest_index();
        'outer: while ptr <= max_idx {
            if ptr == vm.registers.ip {
                out!(d, ">");
            } else {
                out!(d, " ");
            }
            if ptr == vm.registers.relative_base {
                out!(d, "@");
            } else {
                out!(d, " ");
            }
            out!(d, "   {: >10}   ", ptr);
            let instruction = match vm.memory.read(ptr) {
                Ok(instruction) => instruction,
                Err(err) => {
                    out!(d, "             cannot read memory at address ({})\n", err);
                    ptr += 1;
                    continue;
                }
            };
            out!(d, "{: >10}   ", instruction);
            let opcode = match Opcode::try_from(instruction % 100) {
                Ok(opcode) => opcode,
                Err(err) => {
                    out!(d, "cannot parse opcode ({})\n", err);
                    ptr += 1;
                    continue;
                }
            };

            let (in_params, out_params): (usize, usize) = match opcode {
                Opcode::Add => (2, 1),
                Opcode::Multiply => (2, 1),
                Opcode::Input => (0, 1),
                Opcode::Output => (1, 0),
                Opcode::JumpIfTrue => (2, 0),
                Opcode::JumpIfFalse => (2, 0),
                Opcode::LessThan => (2, 1),
                Opcode::Equals => (2, 1),
                Opcode::AdjRelBase => (1, 0),
                Opcode::Halt => (0, 0),
            };

            let modes = {
                let mut mode_list = ArrayVec::<[Mode; 3]>::new();
                let mut modes = instruction / 100;
                for idx in 0..in_params + out_params {
                    mode_list.push(match Mode::try_from(modes % 10) {
                        Ok(mode) => mode,
                        Err(err) => {
                            out!(d, "cannot decode mode for param {} ({})\n", idx, err);
                            ptr += (in_params + out_params + 1) as Value;
                            continue 'outer;
                        }
                    });
                    modes /= 10;
                }
                mode_list
            };

            let values = {
                let mut values = ArrayVec::<[Value; 4]>::new();
                for idx in 0..in_params + out_params {
                    values.push(match vm.memory.read(ptr + 1 + idx as Value) {
                        Ok(value) => value,
                        Err(err) => {
                            out!(d, "cannot read value for param {} ({})", idx, err);
                            ptr += (in_params + out_params + 1) as Value;
                            continue 'outer;
                        }
                    });
                }
                values
            };

            let str_st_len = d.len();
            out!(
                d,
                "{} ",
                match opcode {
                    Opcode::Add => "add",
                    Opcode::Multiply => "mul",
                    Opcode::Input => "inp",
                    Opcode::Output => "out",
                    Opcode::JumpIfTrue => "jit",
                    Opcode::JumpIfFalse => "jif",
                    Opcode::LessThan => "clt",
                    Opcode::Equals => "ceq",
                    Opcode::AdjRelBase => "rel",
                    Opcode::Halt => "hcf",
                }
            );
            for i in 0..in_params + out_params {
                match modes[i] {
                    Mode::Immediate => out!(d, "{}", values[i]),
                    Mode::Position => out!(d, "[{}]", values[i]),
                    Mode::Relative => out!(d, "@[{}]", values[i]),
                }
                if in_params > 0 && i == in_params - 1 {
                    if out_params != 0 {
                        out!(d, " => ");
                    }
                } else if i != in_params + out_params - 1 {
                    out!(d, ", ");
                }
            }

            for _ in d.len() - str_st_len..50 {
                out!(d, " ");
            }
            out!(d, "\n");

            ptr += (in_params + out_params + 1) as Value;
        }

        d
    }
}
