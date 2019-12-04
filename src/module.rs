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

// Main function
#[macro_export]
macro_rules! generate_main {
    ($($mod_name:ident)*) => {
        $(
            mod $mod_name;
        )*

        fn main() {
            use module::*;
            use colored::Colorize;
            println!("{} {} {} {}", "Advent".bright_red().bold(),
                "of".bright_white(), "Code".bright_green().bold(), "2019".bright_blue());

            let exclusive_module = get_exclusive_module();
            let exclusive_module = exclusive_module.as_ref().map(String::as_str);

            $(
            if exclusive_module.is_none() || exclusive_module == Some(stringify!($mod_name)) {
                execute_module(stringify!($mod_name), |input, mut closure| $mod_name::module(input, |msg| {
                    execute_module_callback(&mut closure, msg)
                }));
            }
            )*;
        }
    };
}

pub fn get_exclusive_module() -> Option<String> {
    std::env::args().skip(1).next()
}

pub fn read_module_input(module_name: &'static str) -> std::io::Result<String> {
    std::fs::read_to_string(format!("./data/{}.txt", module_name))
}

pub struct Closure<'a> {
    module_name: &'static str,
    stdout: &'a mut std::io::Stdout,
    last_part: &'a mut Option<&'static str>,
}
pub fn execute_module_callback(closure: &mut Closure, msg: Message<'static>) {
    use colored::Colorize;
    use std::io::Write;
    match msg {
        Message::Start(part) => {
            *closure.last_part = Some(part);
            let _ = write!(
                closure.stdout,
                "{} {}",
                closure.module_name.bright_green(),
                part.bright_blue().bold()
            );
            let _ = closure.stdout.flush();
        }
        Message::Finish(part, result) => {
            assert_eq!(Some(part), *closure.last_part);
            match result {
                Ok(s) => {
                    let s = s.trim();
                    if s.contains('\n') || s.contains('\r') {
                        println!("\n{}", s.bright_white());
                    } else {
                        println!(" {}", s.bright_white());
                    }
                }
                Err(err) => {
                    println!(
                        "\n{} {}\n{}",
                        "error:".bright_red().bold().underline(),
                        err,
                        format!("{:?}", err).red()
                    );
                }
            }
        }
    }
}
pub fn execute_module<F>(module_name: &'static str, executor: F)
where
    F: FnOnce(&str, Closure),
{
    use colored::Colorize;
    let mut stdout = std::io::stdout();
    match read_module_input(module_name) {
        Ok(input) => {
            let mut last_part = None;
            executor(
                input.trim(),
                Closure {
                    module_name,
                    stdout: &mut stdout,
                    last_part: &mut last_part,
                },
            );
        }
        Err(err) => {
            eprintln!(
                "{}\n{}",
                "error reading input:".bright_red().bold().underline(),
                format!("{:?}", err).red()
            );
        }
    }
}
