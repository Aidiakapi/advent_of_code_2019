use crate::intcode::{
    sparse_memory,
    util::{parse_intcode, reading_not_supported, write_batching},
    Error as icError, IoOperation, Value, VM,
};
use crate::HashMap;
use std::cmp::Ordering;
use std::convert::{TryFrom, TryInto};

type Vec2 = crate::vec2::Vec2<Value>;

module!(pt1: parse_intcode, pt2: parse_intcode);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl TryFrom<Value> for Tile {
    type Error = icError;
    fn try_from(value: Value) -> ::std::result::Result<Tile, icError> {
        Ok(match value {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HorizontalPaddle,
            4 => Tile::Ball,
            _ => return Err(icError::Custom(format!("invalid tile ({})", value))),
        })
    }
}

fn pt1(memory: Vec<Value>) -> Result<usize> {
    let mut board: HashMap<Vec2, Tile> = HashMap::new();
    let mut vm = VM::new(sparse_memory(memory));
    vm.run_all(
        reading_not_supported,
        write_batching(|(x, y, tile)| {
            board.insert(Vec2::new(x, y), tile.try_into()?);
            Ok(())
        }),
    )?;

    Ok(board.values().filter(|&&tile| tile == Tile::Block).count())
}

fn pt2(mut memory: Vec<Value>) -> Result<i64> {
    memory[0] = 2;
    let mut board: HashMap<Vec2, Tile> = HashMap::new();
    let mut score = 0;

    let mut ball_pos = Vec2::default();
    let mut paddle_pos = Vec2::default();
    let mut pos = Vec2::default();
    let mut output_n = 0;
    let mut vm = VM::new(sparse_memory(memory));
    vm.run_all_async(|io_op| {
        match io_op {
            IoOperation::Read(value) => {
                *value = Some(match paddle_pos.x.cmp(&ball_pos.x) {
                    Ordering::Less => 1,
                    Ordering::Equal => 0,
                    Ordering::Greater => -1,
                });
            }
            IoOperation::Write(value) => match output_n {
                0 => {
                    pos.x = value;
                    output_n = 1;
                }
                1 => {
                    pos.y = value;
                    output_n = 2;
                }
                2 => {
                    if pos == (-1, 0).into() {
                        score = value;
                    } else {
                        let tile = value.try_into()?;
                        board.insert(pos, tile);
                        match tile {
                            Tile::Ball => ball_pos = pos,
                            Tile::HorizontalPaddle => paddle_pos = pos,
                            _ => {}
                        }
                    }
                    output_n = 0;
                }
                _ => unreachable!(),
            },
        }
        Ok(())
    })?;

    let blocks_left = board.values().filter(|&&tile| tile == Tile::Block).count();
    if blocks_left == 0 {
        Ok(score)
    } else {
        Err(AoCError::Logic(
            "some blocks could not be destroyed with the trivial AI",
        ))
    }
}
