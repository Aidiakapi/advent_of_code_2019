module!(pt1: parse, pt2: parse);
use crate::mat2::Mat2;
use crate::vec2::Vec2us;
use itertools::Itertools;

fn parse_image_layers(size: Vec2us, data: &[u8]) -> Result<Vec<Mat2<u8>>> {
    if data.len() % (size.x * size.y) != 0 {
        return Err(AoCError::Logic(
            "image data length not a multiple of width * size",
        ));
    }
    Ok(data
        .iter()
        .cloned()
        .chunks(size.x * size.y)
        .into_iter()
        .map(|chunk| {
            let mut image = Mat2::new(0, size);
            for (y, row) in chunk.chunks(size.x).into_iter().enumerate() {
                for (x, value) in row.enumerate() {
                    image[x][y] = value;
                }
            }
            image
        })
        .collect())
}

fn pt1((size, layers): (Vec2us, Vec<u8>)) -> Result<usize> {
    let layers = parse_image_layers(size, &layers)?;
    let (ones, twos) = layers
        .iter()
        .min_by_key(|layer| layer.data.iter().filter(|&&value| value == 0).count())
        .unwrap()
        .data
        .iter()
        .fold((0, 0), |(ones, twos), &value| match value {
            1 => (ones + 1, twos),
            2 => (ones, twos + 1),
            _ => (ones, twos),
        });
    Ok(ones * twos)
}

fn pt2((size, layers): (Vec2us, Vec<u8>)) -> Result<String> {
    let mut layers = parse_image_layers(size, &layers)?;
    let mut output = layers.remove(0);

    for layer in layers {
        for (pos, value) in output.iter_mut() {
            if *value == 2 {
                *value = layer[pos];
            }
        }
    }

    let mut res = String::with_capacity((size.x + size.x / 5 - 1) * size.y + size.y - 1);
    for y in 0..size.y {
        for x in 0..size.x {
            if x != 0 && x % 5 == 0 {
                res.push(' ');
            }
            match output[Vec2us::new(x, y)] {
                0 => res.push(' '),
                1 => res.push('█'),
                _ => return Err(AoCError::NoSolution),
            }
        }
        if y != size.y - 1 {
            res.push('\n');
        }
    }
    Ok(res)
}

fn parse(s: &str) -> IResult<&str, (Vec2us, Vec<u8>)> {
    use parsers::*;
    map(
        many1(map(one_of("012"), |c: char| c as u8 - b'0')),
        |layers| (Vec2us::new(25, 6), layers),
    )(s)
}

#[test]
fn day08() -> Result<()> {
    assert_eq!(pt2((Vec2us::new(2, 2), (parse("0222112222120000")?.1).1))?, " █\n█ ");
    Ok(())
}
