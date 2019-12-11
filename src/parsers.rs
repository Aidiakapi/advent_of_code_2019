#![allow(dead_code)]
#[allow(unused_imports)]
pub use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, anychar, char, digit1, line_ending, one_of},
    combinator::{map, map_res, opt},
    error::ErrorKind,
    multi::{fold_many1, many0, many1, separated_list},
    sequence::{delimited, pair, terminated, tuple},
    Err, IResult,
};

macro_rules! unsigned_nr_str_parser {
    ($fn_name: ident, $t:ident) => {
        pub fn $fn_name(s: &str) -> IResult<&str, $t> {
            map_res(digit1, |s: &str| {
                s.parse::<$t>()
                    .map_err(|_err| Err::Error((s, ErrorKind::Digit)))
            })(s)
        }
    };
}

macro_rules! signed_nr_str_parser {
    ($fn_name: ident, $t:ident) => {
        pub fn $fn_name(s: &str) -> IResult<&str, $t> {
            map_res(
                pair(opt(one_of("+-")), digit1),
                |(sign, s): (Option<char>, &str)| {
                    s.parse::<$t>()
                        .map_err(|_err| Err::Error((s, ErrorKind::Digit)))
                        .map(|v| if let Some('-') = sign { -v } else { v })
                },
            )(s)
        }
    };
}

unsigned_nr_str_parser!(usize_str, usize);
unsigned_nr_str_parser!(u8_str, u8);
unsigned_nr_str_parser!(u16_str, u16);
unsigned_nr_str_parser!(u32_str, u32);
unsigned_nr_str_parser!(u64_str, u64);

signed_nr_str_parser!(isize_str, isize);
signed_nr_str_parser!(i8_str, i8);
signed_nr_str_parser!(i16_str, i16);
signed_nr_str_parser!(i32_str, i32);
signed_nr_str_parser!(i64_str, i64);

use nom::{
    error::{make_error, ParseError},
    Compare, InputIter, InputLength, Slice,
};
use std::ops::{Range, RangeFrom, RangeTo};
pub fn line_ending_or_eof<T, E: ParseError<T>>(input: T) -> IResult<T, (), E>
where
    T: Clone,
    T: Slice<Range<usize>> + Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
    T: InputIter + InputLength,
    T: Compare<&'static str>,
{
    alt((map(line_ending, |_| ()), eof))(input)
}

/// Matches the end of the file
pub fn eof<T: nom::InputLength, E: ParseError<T>>(input: T) -> IResult<T, (), E> {
    if input.input_len() == 0 {
        Ok((input, ()))
    } else {
        Err(nom::Err::Error(make_error(input, ErrorKind::Eof)))
    }
}
