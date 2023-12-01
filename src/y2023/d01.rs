use winnow::{
    ascii::dec_int,
    combinator::{alt, peek, rest, terminated},
    token::take,
    PResult, Parser,
};

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|mut line| {
            let (dec, units) = extract_digits_1(&mut line);
            dec * 10 + units
        })
        .sum()
}

pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|mut line| {
            let (dec, units) = extract_digits_2(&mut line);
            dec * 10 + units
        })
        .sum()
}

fn extract_digits_1(line: &mut &str) -> (i64, i64) {
    let mut dec = None;
    let mut units = None;

    while !line.is_empty() {
        if let Ok((_, num)) = extract_digit.parse_peek(line) {
            if dec.is_none() {
                dec = Some(num);
            } else {
                units = Some(num);
            }
        }
        *line = &line[1..];
    }

    (dec.unwrap(), units.or(dec).unwrap())
}

fn extract_digits_2(line: &mut &str) -> (i64, i64) {
    let mut dec = None;
    let mut units = None;

    while !line.is_empty() {
        if let Ok((_, num)) = extract_digit_complete.parse_peek(line) {
            if dec.is_none() {
                dec = Some(num);
            } else {
                units = Some(num);
            }
        }
        *line = &line[1..];
    }

    (dec.unwrap(), units.or(dec).unwrap())
}

fn extract_digit(line: &mut &str) -> PResult<i64> {
    terminated(take(1_usize).and_then(dec_int), peek(rest)).parse_next(line)
}

fn extract_digit_complete(line: &mut &str) -> PResult<i64> {
    terminated(
        alt((
            "one".value(1_i64),
            "two".value(2_i64),
            "three".value(3_i64),
            "four".value(4_i64),
            "five".value(5_i64),
            "six".value(6_i64),
            "seven".value(7_i64),
            "eight".value(8_i64),
            "nine".value(9_i64),
            take(1_usize).and_then(dec_int),
        )),
        peek(rest),
    )
    .parse_next(line)
}
