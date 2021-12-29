module!(pt1: parse, pt2: parse);

fn parse(s: &str) -> IResult<&str, Vec<i32>> {
    use parsers::*;
    separated_list1(line_ending, i32_str)(s)
}

fn pt1(modules: Vec<i32>) -> i32 {
    modules.into_iter().map(|mass| mass / 3 - 2).sum()
}

fn calc_fuel(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel <= 0 {
        0
    } else {
        fuel + calc_fuel(fuel)
    }
}

fn pt2(modules: Vec<i32>) -> i32 {
    modules.into_iter().map(calc_fuel).sum()
}
