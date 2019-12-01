#![feature(optin_builtin_traits)]

pub(crate) mod error;
pub(crate) mod parsers;
#[macro_use]
pub(crate) mod module;

macro_rules! gen_main {
    ($($mod_name:ident)*) => {
        $(
            mod $mod_name;
        )*

        fn main() {
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

gen_main!(
    
);

fn get_exclusive_module() -> Option<String> {
    std::env::args().skip(1).next()
}

fn read_module_input(module_name: &'static str) -> std::io::Result<String> {
    std::fs::read_to_string(format!("./data/{}.txt", module_name))
}

struct Closure<'a> {
    module_name: &'static str,
    stdout: &'a mut std::io::Stdout,
    last_part: &'a mut Option<&'static str>,
}
fn execute_module_callback(closure: &mut Closure, msg: module::Message<'static>) {
    use colored::Colorize;
    use module::Message;
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
                    println!("\n{} {}\n{}", "error:".bright_red().bold().underline(), err, format!("{:?}", err).red());
                }
            }
        }
    }
}
fn execute_module<F>(module_name: &'static str, executor: F)
where
    F: FnOnce(&str, Closure),
{
    use colored::Colorize;
    let mut stdout = std::io::stdout();
    match read_module_input(module_name) {
        Ok(input) => {
            let mut last_part = None;
            executor(
                &input,
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
