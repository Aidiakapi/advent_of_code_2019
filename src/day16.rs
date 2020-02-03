#![allow(clippy::unreadable_literal, clippy::identity_op)]
use std::iter::repeat;

module!(pt1: parse, pt2: parse);

/// Calculates the binomial coefficients modulo 10.
/// Each entry is nCr(n, m) % 10 where:
///     n = i + depth - 1
///     m = depth - 1
///
/// i iterates from 0 to width, at 0 this results in:
/// nCr(x, x) = 1
///
/// It relies on the following derivation to calculate all following identities:
/// nCr(a + 1, b) = nCr(a, b) * (a + 1) / (a - b + 1)
///
/// Panics if width or depth are 0
fn calculate_binomial_coefficients_mod_10(width: usize, depth: usize) -> Vec<u8> {
    use num::{BigUint, One, ToPrimitive};
    assert!(width > 0 && depth > 0);
    let mut coefficients = Vec::with_capacity(width);
    let m = depth - 1;
    let mut current = BigUint::one();
    coefficients.push(1);
    for i in 1..width {
        current *= i + m;
        current /= i;
        coefficients.push((&current % 10u64).to_u8().unwrap());
    }

    coefficients
}

#[inline]
fn fft_pattern(idx: usize) -> impl Iterator<Item = i32> + Clone {
    repeat(0)
        .take(idx + 1)
        .chain(repeat(1).take(idx + 1))
        .chain(repeat(0).take(idx + 1))
        .chain(repeat(-1).take(idx + 1))
        .cycle()
        .skip(1)
}

#[inline]
#[allow(clippy::needless_range_loop)]
fn fft(digits: &[u8], output: &mut [u8]) {
    debug_assert_eq!(digits.len(), output.len());
    for idx in 0..digits.len() {
        output[idx] = (digits
            .iter()
            .zip(fft_pattern(idx))
            .map(|(digit, pattern)| (*digit as i32) * pattern)
            .sum::<i32>()
            .abs()
            % 10) as u8;
    }
}

fn fft100(digits: &mut Vec<u8>) {
    let mut buffer = vec![0; digits.len()];
    for _ in 0..100 {
        fft(digits, &mut buffer);
        std::mem::swap(digits, &mut buffer);
    }
}

fn fft_multi(
    digits: &[u8],
    iterations: usize,
    skip: usize,
    count: Option<usize>,
) -> Result<Vec<u8>> {
    if skip >= digits.len() {
        return Err(AoCError::Logic(
            "skip may not be larger than the input digits",
        ));
    }
    if skip < digits.len() / 2 {
        return Err(AoCError::Logic(
            "cannot use fft_multi for the first half of the output",
        ));
    }
    let coefficients = calculate_binomial_coefficients_mod_10(digits.len() - skip, iterations);
    let mut output = Vec::with_capacity(digits.len() - skip);

    for idx in skip..if let Some(count) = count {
        digits.len().min(skip + count)
    } else {
        digits.len()
    } {
        output.push(
            digits[idx..]
                .iter()
                .enumerate()
                .map(|(j, value)| *value * coefficients[j])
                .fold(0, |acc, value| (acc + value) % 10),
        );
    }

    Ok(output)
}

fn pt1(mut digits: Vec<u8>) -> String {
    fft100(&mut digits);
    format!(
        "{:0>8}",
        000 + digits[0] as u32 * 10000000
            + digits[1] as u32 * 1000000
            + digits[2] as u32 * 100000
            + digits[3] as u32 * 10000
            + digits[4] as u32 * 1000
            + digits[5] as u32 * 100
            + digits[6] as u32 * 10
            + digits[7] as u32 * 1
    )
}

fn pt2(digits: Vec<u8>) -> Result<String> {
    let digits = digits.repeat(10_000);
    let offset = 0
        + digits[0] as usize * 1000000
        + digits[1] as usize * 100000
        + digits[2] as usize * 10000
        + digits[3] as usize * 1000
        + digits[4] as usize * 100
        + digits[5] as usize * 10
        + digits[6] as usize * 1;

    let output = fft_multi(&digits, 100, offset, Some(8))?;

    Ok(format!(
        "{:0>8}",
        000 + output[0] as u32 * 10000000
            + output[1] as u32 * 1000000
            + output[2] as u32 * 100000
            + output[3] as u32 * 10000
            + output[4] as u32 * 1000
            + output[5] as u32 * 100
            + output[6] as u32 * 10
            + output[7] as u32 * 1
    ))
}

fn parse(s: &str) -> IResult<&str, Vec<u8>> {
    use parsers::*;
    many1(map(one_of("0123456789"), |c| c as u8 - b'0'))(s)
}

#[test]
fn day16() -> Result<()> {
    let mut digits = vec![1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(fft_multi(&digits, 1, 4, None)?, vec![6, 1, 5, 8]);
    assert_eq!(fft_multi(&digits, 2, 4, None)?, vec![0, 4, 3, 8]);
    assert_eq!(fft_multi(&digits, 3, 4, None)?, vec![5, 5, 1, 8]);
    assert_eq!(fft_multi(&digits, 4, 4, None)?, vec![9, 4, 9, 8]);

    let mut output = digits.clone();
    fft(&digits, &mut output);
    std::mem::swap(&mut digits, &mut output);
    assert_eq!(digits, vec![4, 8, 2, 2, 6, 1, 5, 8]);
    fft(&digits, &mut output);
    std::mem::swap(&mut digits, &mut output);
    assert_eq!(digits, vec![3, 4, 0, 4, 0, 4, 3, 8]);
    fft(&digits, &mut output);
    std::mem::swap(&mut digits, &mut output);
    assert_eq!(digits, vec![0, 3, 4, 1, 5, 5, 1, 8]);
    fft(&digits, &mut output);
    std::mem::swap(&mut digits, &mut output);
    assert_eq!(digits, vec![0, 1, 0, 2, 9, 4, 9, 8]);

    let digits = vec![
        8, 0, 8, 7, 1, 2, 2, 4, 5, 8, 5, 9, 1, 4, 5, 4, 6, 6, 1, 9, 0, 8, 3, 2, 1, 8, 6, 4, 5, 5,
        9, 5,
    ];
    let mut digits100 = digits.clone();
    fft100(&mut digits100);
    assert_eq!(fft_multi(&digits, 100, 16, None)?[..], digits100[16..]);

    Ok(())
}
