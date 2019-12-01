macro_rules! module_part {
    ($input:expr, $result:expr, $part_name:ident, $parser:ident) => {
        use crate::module::ToModuleResult;
        let parsed = $parser($input);
        *$result = Some(
            parsed
                .to_module_result()
                .and_then(|parsed| $part_name(parsed).to_module_result()),
        );
    };
    ($input:expr, $result:expr, $part_name:ident) => {
        *$result = Some($part_name($input).to_module_result());
    };
}

#[macro_export]
macro_rules! module {
    ($($part_name:ident$(: $parser:ident)?),*) => {
        #[allow(unused_imports)]
        use {crate::{parsers, error::AoCError, module::Result}, nom::IResult};
        #[allow(dead_code)]
        pub(crate) fn module<Out>(input: &str, mut out: Out)
        where
            Out: FnMut(crate::module::Message<'static>) -> ()
        {
            $({
                out(crate::module::Message::Start(stringify!($part_name)));
                let mut result = None;
                module_part!(input, &mut result, $part_name$(, $parser)?);
                let result = result.unwrap();
                use std::string::ToString;
                out(crate::module::Message::Finish(stringify!($part_name), result.map(|result| result.to_string())));
            })*
        }
    };
}

use crate::error::AoCError;
use nom::IResult;
use std::fmt::Debug;

pub type Result<T> = ::std::result::Result<T, crate::error::AoCError>;

pub auto trait IsNotResult {}
impl<T, E> !IsNotResult for std::result::Result<T, E> {}
pub trait ToModuleResult: Sized {
    type Output;
    fn to_module_result(self) -> Self::Output;
}
impl<T> ToModuleResult for T
where
    T: IsNotResult,
{
    type Output = Result<Self>;
    fn to_module_result(self) -> Self::Output {
        Ok(self)
    }
}
impl<T> ToModuleResult for Result<T> {
    type Output = Self;
    fn to_module_result(self) -> Self::Output {
        self
    }
}
impl<O, E: Debug> ToModuleResult for IResult<&'_ str, O, E> {
    type Output = Result<O>;
    fn to_module_result(self) -> Self::Output {
        self.map_err(Into::into).and_then(|(remainder, result)| {
            if remainder.len() != 0 {
                Err(AoCError::IncompleteParse {
                    remainder: remainder.to_owned(),
                })
            } else {
                Ok(result)
            }
        })
    }
}
impl<O, E: Debug> ToModuleResult for IResult<(), O, E> {
    type Output = Result<O>;
    fn to_module_result(self) -> Self::Output {
        self.map(|(_, result)| result).map_err(Into::into)
    }
}

pub enum Message<'s> {
    Start(&'s str),
    Finish(&'s str, Result<String>),
}
