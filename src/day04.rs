module!(pt1: parse, pt2: parse);

use std::ops::RangeInclusive;

#[derive(Clone, Copy, PartialEq, Eq)]
struct DigitIter(Option<u32>);

impl Iterator for DigitIter {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let curr = self.0?;
        let remainder = curr / 10;
        self.0 = if remainder > 0 { Some(remainder) } else { None };
        Some(curr % 10)
    }
}

fn digit_iter(nr: u32) -> DigitIter {
    DigitIter(Some(nr))
}

fn increasing_6_digits(range: RangeInclusive<u32>) -> impl Iterator<Item = u32> {
    range.filter(|&nr| {
        // 6 digit
        if nr >= 1_000_000 || nr < 100_000 {
            return false;
        }
        // increasing
        digit_iter(nr)
            .skip(1)
            .zip(digit_iter(nr))
            .clone()
            .all(|(left, right)| left <= right)
    })
}

fn pt1(range: RangeInclusive<u32>) -> usize {
    increasing_6_digits(range)
        .filter(|&nr| {
            // two adjacent digits
            digit_iter(nr)
                .skip(1)
                .zip(digit_iter(nr))
                .any(|(left, right)| left == right)
        })
        .count()
}

fn pt2(range: RangeInclusive<u32>) -> usize {
    increasing_6_digits(range)
        .filter(|&nr| {
            // two (but not more) adjacent digits
            let mut rep_count = 1;
            let mut last_digit = nr % 10;
            for digit in digit_iter(nr).skip(1) {
                if last_digit == digit {
                    rep_count += 1;
                } else {
                    if rep_count == 2 {
                        return true;
                    }
                    rep_count = 1;
                    last_digit = digit;
                }
            }
            rep_count == 2
        })
        .count()
}

fn parse(s: &str) -> IResult<&str, RangeInclusive<u32>> {
    use parsers::*;
    map(tuple((u32_str, char('-'), u32_str)), |(from, _, to)| {
        from..=to
    })(s)
}
