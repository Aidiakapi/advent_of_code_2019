use std::cmp::Ordering;

module!(pt1: parse, pt2: parse);

type Card = u16;
fn factory_order() -> Vec<Card> {
    (0..10007u16).collect()
}

fn deal_into_new_stack(deck: &mut [Card]) {
    deck.reverse()
}
fn cut(deck: &mut [Card], amt: i32) {
    match amt.cmp(&0) {
        Ordering::Equal => {}
        Ordering::Greater => deck.rotate_left((amt as usize) % deck.len()),
        Ordering::Less => deck.rotate_right(((-amt) as usize) % deck.len()),
    }
}
fn deal_with_increment(deck: &mut Vec<Card>, backbuffer: &mut Vec<Card>, increment: usize) {
    assert_eq!(deck.len(), backbuffer.len());
    let mut idx = 0;
    for &card in deck.iter() {
        backbuffer[idx] = card;
        idx += increment;
        if idx >= deck.len() {
            idx -= deck.len();
        }
    }
    std::mem::swap(deck, backbuffer);
}

fn apply_technique(technique: Technique, deck: &mut Vec<Card>, backbuffer: &mut Vec<Card>) {
    match technique {
        Technique::DealIntoNewStack => deal_into_new_stack(deck),
        Technique::Cut(amt) => cut(deck, amt),
        Technique::DealWithIncrement(increment) => deal_with_increment(deck, backbuffer, increment),
    }
}

fn pt1(steps: Vec<Technique>) -> usize {
    let mut deck = factory_order();
    let mut backbuffer = deck.clone();

    for technique in steps {
        apply_technique(technique, &mut deck, &mut backbuffer);
    }
    deck.iter().position(|&card| card == 2019).unwrap()
}

/// Calculates the multiplicative inverse in a finite field.
/// Based on psuedocode in: https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Computing_multiplicative_inverses_in_modular_structures
fn multiplicative_inverse(a: i128, n: i128) -> i128 {
    let mut t = 0i128;
    let mut newt = 1i128;
    let mut r = n;
    let mut newr = a;

    while newr != 0 {
        let quotient = r / newr;
        t = t - quotient * newt;
        r = r - quotient * newr;
        std::mem::swap(&mut t, &mut newt);
        std::mem::swap(&mut r, &mut newr);
    }

    if r > 1 {
        panic!("invalid n");
    }
    if t < 0 {
        t += n;
    }

    t
}

/// Gets values such that f(p) = p * res.0 + res.1 is a function
/// where p is a position of a card in the deck, and f(p) is the
/// position of that card before shuffling the deck once.
fn get_mul_add_to_reverse_shuffle(steps: &[Technique], deck_size: i128) -> (i128, i128) {
    let mut mul = 1i128;
    let mut add = 0i128;
    for &step in steps.iter().rev() {
        match step {
            Technique::DealIntoNewStack => {
                add += 1;
                let x = deck_size - 1;
                mul = (mul * x) % deck_size;
                add = (add * x) % deck_size;
            }
            Technique::Cut(amt) => {
                add =
                    (add + if amt < 0 {
                        deck_size + amt as i128
                    } else {
                        amt as i128
                    }) % deck_size;
            }
            Technique::DealWithIncrement(increment) => {
                let x = multiplicative_inverse(increment as i128, deck_size as i128);
                mul = (mul * x) % deck_size;
                add = (add * x) % deck_size;
            }
        }
    }

    (mul, add)
}

fn modular_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    assert!(modulus > 0 && (modulus - 1) < std::u64::MAX as u128);
    if modulus == 1 {
        return 0;
    }

    let mut res = 1;
    base %= modulus;
    while exp > 0 {
        if (exp % 2) == 1 {
            res = (res * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }

    res
}

#[inline]
fn nr_in_position_after(
    techniques: &[Technique],
    position: i128,
    deck_size: i128,
    rep_count: u64,
) -> i128 {
    let (mul, add) = get_mul_add_to_reverse_shuffle(techniques, deck_size);

    // Explanation:
    // m = multiplier
    // a = addition
    // f(0) = p + 0
    // f(1) = (p) * m + a = pm + a
    // f(2) = (pm + a) * m + a = pm^2 + am + a
    // f(3) = (pm^2 + am + a) * m + a = pm^3 + am^2 + am + a
    // f(4) = (pm^3 + am^2 + am + a) * m + a = pm^4 + am^3 + am^2 + am + a
    //
    // It can also be rewritten as:
    // f(x) = pm^x + g(x)
    // g(0) = 0
    // g(x) = mg(x - 1) + a
    // Where g is a linear non-homogenous recurrence, which can be rewritten as:
    // g(x) = (am^x - a) / (m - 1)
    //
    // Consequently, calculating all repetitions can be done using:
    // f(x) = pm^x + (am^x - a) / (m - 1)

    let mx = modular_pow(mul as u128, rep_count as u128, deck_size as u128) as i128;
    let pmx = (position * mx) % deck_size;
    let amx = (add * mx) % deck_size;
    let inv = multiplicative_inverse(mul - 1, deck_size);
    let res = (pmx + (amx - add) * inv) % deck_size;
    if res < 0 {
        res + deck_size
    } else {
        res
    }
}

fn pt2(steps: Vec<Technique>) -> i128 {
    const DECK_SIZE: i128 = 119315717514047;
    const REPETITION_COUNT: u64 = 101741582076661;

    nr_in_position_after(&steps, 2020, DECK_SIZE, REPETITION_COUNT)
}

fn parse(s: &str) -> IResult<&str, Vec<Technique>> {
    use parsers::*;

    let deal_into_new_stack = map(tag("deal into new stack"), |_| Technique::DealIntoNewStack);
    let cut = map(preceded(tag("cut "), i32_str), Technique::Cut);
    let deal_with_increment = map(
        preceded(tag("deal with increment "), usize_str),
        Technique::DealWithIncrement,
    );

    let technique = alt((deal_into_new_stack, cut, deal_with_increment));
    many1(terminated(technique, line_ending_or_eof))(s)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Technique {
    DealIntoNewStack,
    Cut(i32),
    DealWithIncrement(usize),
}

#[test]
fn day22() {
    let steps = vec![
        Technique::DealIntoNewStack,
        Technique::Cut(2),
        Technique::DealWithIncrement(3),
        Technique::DealIntoNewStack,
        Technique::Cut(-49),
        Technique::DealWithIncrement(45),
    ];
    let steps = steps.as_slice();

    let deck = &mut factory_order();
    deck.truncate(101);
    let backbuffer = &mut deck.clone();

    let mut iter = move |deck: &mut Vec<Card>| {
        for &technique in steps {
            apply_technique(technique, deck, backbuffer);
        }
    };

    for i in 1..=10 {
        iter(deck);
        for j in 0..deck.len() {
            assert_eq!(
                deck[j] as i128,
                nr_in_position_after(steps, j as i128, 101, i)
            );
        }
    }
}
