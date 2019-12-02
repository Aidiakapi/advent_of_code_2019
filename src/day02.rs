module!(pt1: parse, pt2: parse);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Program {
    memory: Vec<usize>,
}

impl Program {
    fn run_till_halt(&mut self) -> Result<()> {
        const OUT_OF_RANGE: AoCError = AoCError::Logic("index out of range");

        let mut ip = 0;
        loop {
            match *self.memory.get(ip).ok_or(OUT_OF_RANGE)? {
                x @ 1 | x @ 2 => {
                    let a = *self
                        .memory
                        .get(*self.memory.get(ip + 1).ok_or(OUT_OF_RANGE)?)
                        .ok_or(OUT_OF_RANGE)?;
                    let b = *self
                        .memory
                        .get(*self.memory.get(ip + 2).ok_or(OUT_OF_RANGE)?)
                        .ok_or(OUT_OF_RANGE)?;
                    let c = if x == 1 { a + b } else { a * b };
                    let idx = *self.memory.get(ip + 3).ok_or(OUT_OF_RANGE)?;
                    *self.memory.get_mut(idx).ok_or(OUT_OF_RANGE)? = c;
                    ip += 4;
                }
                99 => {
                    return Ok(());
                }
                _ => {
                    return Err(AoCError::Logic("invalid opcode"));
                }
            }
        }
    }
}

fn pt1(mut memory: Vec<usize>) -> Result<usize> {
    memory[1] = 12;
    memory[2] = 2;
    let mut program = Program { memory };
    program.run_till_halt()?;
    Ok(program.memory[0])
}

fn pt2(mut memory: Vec<usize>) -> Result<usize> {
    for noun in 0..=99 {
        memory[1] = noun;
        for verb in 0..=99 {
            memory[2] = verb;
            let mut program = Program {
                memory: memory.clone(),
            };
            program.run_till_halt()?;
            if program.memory[0] == 19690720 {
                return Ok(noun * 100 + verb);
            }
        }
    }
    Err(AoCError::NoSolution)
}

fn parse(s: &str) -> IResult<&str, Vec<usize>> {
    use parsers::*;
    separated_list(char(','), usize_str)(s)
}
