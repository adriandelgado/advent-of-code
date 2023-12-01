use winnow::{ascii::dec_uint, combinator::alt, token::take, PResult, Parser};

pub fn part1(input: &str) -> u16 {
    input
        .lines()
        .map(extract_digits::<true>)
        .map(|(dec, units)| dec * 10 + units)
        .sum()
}

pub fn part2(input: &str) -> u16 {
    input
        .lines()
        .map(extract_digits::<false>)
        .map(|(dec, units)| dec * 10 + units)
        .sum()
}

fn extract_digits<const PART_1: bool>(mut line: &str) -> (u16, u16) {
    let mut dec = None;
    let mut units = None;

    while !line.is_empty() {
        let result = if PART_1 {
            extract_digit.parse_peek(line)
        } else {
            extract_digit_spelled.parse_peek(line)
        };

        if let Ok((_, digit)) = result {
            if dec.is_none() {
                dec = Some(digit);
            } else {
                units = Some(digit);
            }
        }

        line = &line[1..];
    }

    (dec.unwrap(), units.or(dec).unwrap())
}

fn extract_digit(line: &mut &str) -> PResult<u16> {
    take(1_usize).and_then(dec_uint).parse_next(line)
}

fn extract_digit_spelled(line: &mut &str) -> PResult<u16> {
    alt((
        "one".value(1),
        "two".value(2),
        "three".value(3),
        "four".value(4),
        "five".value(5),
        "six".value(6),
        "seven".value(7),
        "eight".value(8),
        "nine".value(9),
        take(1_usize).and_then(dec_uint),
    ))
    .parse_next(line)
}
