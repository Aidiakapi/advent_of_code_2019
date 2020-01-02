use crate::intcode::{
    self,
    util::{parse_intcode, read_from_iter, write_once},
    GrowingMemory, Value, VM,
};
use crate::vec2::Vec2us;
use std::collections::VecDeque;

module!(pt1: parse_intcode, pt2: parse_intcode);

fn is_in_tractor_beam(
    original_memory: &Vec<Value>,
    vm: &mut VM<GrowingMemory>,
    pos: Vec2us,
) -> Result<bool> {
    let mut res = None;
    let vm_result = vm.run_all(
        read_from_iter(pos.convert::<Value>()?),
        write_once(&mut res),
    );

    // Reset VM memory to original
    vm.registers = intcode::Registers::default();
    vm.state = intcode::State::default();
    // If the offsets that change each iteration aren't universal
    // for all AoC inputs, this optimization should be disabled.
    // vm.memory.0 = original_memory.clone();
    vm.memory.0.truncate(original_memory.len());
    for &i in [132, 222, 221, 223, 224, 249].iter() {
        vm.memory.0[i] = original_memory[i];
    }

    vm_result?;
    let res = res.ok_or(AoCError::IncorrectInput(
        "program did not produce an output",
    ))?;
    Ok(res == 1)
}

fn pt1(memory: Vec<Value>) -> Result<usize> {
    let mut vm = VM::new(GrowingMemory(memory.clone()));
    (0..50)
        .flat_map(|x| (0..50).map(move |y| Vec2us::new(x, y)))
        .map(|pos| is_in_tractor_beam(&memory, &mut vm, pos))
        .fold(Ok(0), |acc, val| Ok(acc? + if val? { 1 } else { 0 }))
}

fn pt2(memory: Vec<Value>) -> Result<usize> {
    let mut vm = VM::new(GrowingMemory(memory.clone()));

    let mut is_in_tractor_beam = move |x: usize, y: usize| -> bool {
        is_in_tractor_beam(&memory, &mut vm, Vec2us::new(x, y)).unwrap()
    };

    let mut start_row = None;
    let mut rows = VecDeque::with_capacity(100);
    // Finds a valid starting point (where the beam becomes continuous)
    'outer: for i in 0..20 {
        for j in 0..20 {
            for &(mut x, mut y) in [(i, j), (j, i)].iter() {
                if is_in_tractor_beam(x, y)
                    && is_in_tractor_beam(x + 1, y)
                    && is_in_tractor_beam(x, y + 1)
                {
                    while is_in_tractor_beam(x, y - 1) {
                        y -= 1;
                    }
                    while is_in_tractor_beam(x - 1, y) {
                        x -= 1;
                    }
                    let start = x;
                    while is_in_tractor_beam(x + 1, y) {
                        x += 1;
                    }
                    let range = start..x + 1;
                    start_row = Some(y);
                    rows.push_back(range);
                    break 'outer;
                }
            }
        }
    }

    let start_row = start_row.ok_or(AoCError::NoSolution)?;
    for y in start_row + 1.. {
        let prev_range = rows.back().unwrap().clone();
        let mut x = prev_range.start;
        while !is_in_tractor_beam(x, y) {
            x += 1;
        }
        let start = x;
        x = prev_range.end - 1;
        while is_in_tractor_beam(x, y) {
            x += 1;
        }
        let end = x;

        rows.push_back(start..end);
        if rows.len() == 101 {
            rows.pop_front();
        } else {
            continue;
        }
        if end - start >= 100
            && rows
                .front()
                .unwrap()
                .end
                .checked_sub(start)
                .map(|v| v >= 100)
                .unwrap_or(false)
        {
            let x = start;
            let y = y - 99;
            return Ok(x * 10000 + y);
        }
    }
    unreachable!()
}
