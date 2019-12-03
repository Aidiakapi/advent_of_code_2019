use std::fmt::Debug;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum AoCError {
    #[error("parse int error")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("parse float error")]
    ParseFloat(#[from] std::num::ParseFloatError),
    #[error("nom parse error")]
    NomParse(String),
    #[error("incomplete parse (remainder {remainder:?})")]
    IncompleteParse { remainder: String },
    #[error("intcode error {0}")]
    Intcode(#[from] crate::intcode::Error),
    #[error("no possible solution found")]
    NoSolution,
    #[error("logic error ({0})")]
    Logic(&'static str),
    #[error("incorrect input ({0})")]
    IncorrectInput(&'static str),
}

impl<E: Debug> From<nom::Err<E>> for AoCError {
    fn from(err: nom::Err<E>) -> AoCError {
        AoCError::NomParse(format!("{:?}", err))
    }
}
