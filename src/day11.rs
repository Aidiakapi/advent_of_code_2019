use crate::intcode::{sparse_memory, util::parse_intcode, Error, IoOperation, Value, VM};
use crate::vec2::{AabbIteratorEx, Vec2i};
use crate::HashMap;
module!(pt1: parse_intcode, pt2: parse_intcode);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Black,
    White,
}

fn paint(memory: Vec<Value>, starting_color: Color) -> Result<HashMap<Vec2i, Color>> {
    let mut panels = HashMap::new();
    let mut pos = Vec2i::default();
    let mut vm = VM::new(sparse_memory(memory));
    let mut is_painting = true;
    let mut direction = Vec2i::new(0, -1);
    panels.insert(pos, starting_color);
    vm.run_all_async(|op| {
        match op {
            IoOperation::Read(target) => {
                *target = Some(match panels.get(&pos).cloned().unwrap_or(Color::Black) {
                    Color::Black => 0,
                    Color::White => 1,
                })
            }
            IoOperation::Write(value) => {
                if is_painting {
                    let color = match value {
                        0 => Color::Black,
                        1 => Color::White,
                        _ => {
                            return Err(Error::Custom(format!(
                                "expected 0 or 1 during painting, got {}",
                                value
                            )))
                        }
                    };
                    panels.insert(pos, color);
                } else {
                    match value {
                        0 => direction = Vec2i::new(direction.y, -direction.x),
                        1 => direction = Vec2i::new(-direction.y, direction.x),
                        _ => {
                            return Err(Error::Custom(format!(
                                "expected 0 or 1 during rotation, got {}",
                                value
                            )))
                        }
                    }
                    pos += direction;
                }
                is_painting = !is_painting;
            }
        }
        Ok(())
    })?;
    Ok(panels)
}

fn pt1(memory: Vec<Value>) -> Result<usize> {
    let panels = paint(memory, Color::Black)?;
    Ok(panels.len())
}

fn pt2(memory: Vec<Value>) -> Result<String> {
    let panels = paint(memory, Color::White)?;
    let (min, max) = panels.keys().cloned().aabb().unwrap();

    let mut output = String::new();
    for y in min.y..=max.y {
        for x in min.x..=max.x {
            output.push(
                match panels
                    .get(&Vec2i::new(x, y))
                    .cloned()
                    .unwrap_or(Color::Black)
                {
                    Color::Black => ' ',
                    Color::White => '█',
                },
            );
        }
        output.push('\n');
    }
    Ok(output)
}
