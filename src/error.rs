use std::fmt::Debug;
use thiserror::Error;

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
    #[error("no possible solution found")]
    NoSolution,
}

impl<E: Debug> From<nom::Err<E>> for AoCError {
    fn from(err: nom::Err<E>) -> AoCError {
        AoCError::NomParse(format!("{:?}", err))
    }
}
