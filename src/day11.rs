use crate::intcode::{self, sparse_memory, util::parse_intcode, IoOperation, Value, VM};
use crate::vec2::{AabbIteratorEx, Vec2i};
use crate::HashMap;
use std::convert::{Into, TryFrom, TryInto};
module!(pt1: parse_intcode, pt2: parse_intcode);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Black,
    White,
}
impl From<Color> for Value {
    fn from(color: Color) -> Value {
        match color {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}
impl TryFrom<Value> for Color {
    type Error = intcode::Error;
    fn try_from(value: Value) -> intcode::Result<Color> {
        match value {
            0 => Ok(Color::Black),
            1 => Ok(Color::White),
            _ => Err(intcode::Error::Custom(format!(
                "cannot convert {} into color",
                value
            ))),
        }
    }
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
                *target = Some(panels.get(&pos).cloned().unwrap_or(Color::Black).into())
            }
            IoOperation::Write(value) => {
                if is_painting {
                    panels.insert(pos, value.try_into()?);
                } else {
                    match value {
                        0 => direction = Vec2i::new(direction.y, -direction.x),
                        1 => direction = Vec2i::new(-direction.y, direction.x),
                        _ => {
                            return Err(intcode::Error::Custom(format!(
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
