use bstr::B;
use winnow::{
    ascii::{escaped_transform, hex_uint},
    combinator::{alt, delimited, preceded},
    token::{take, take_till},
    PResult, Parser,
};

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let escaped = parse_line.parse(line.as_bytes()).unwrap().len();
            line.len() - escaped
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.escape_debug().count() + 2 - line.len())
        .sum()
}

fn parse_line(input: &mut &[u8]) -> PResult<Vec<u8>> {
    delimited(
        b'"',
        escaped_transform(
            // TODO: PR to winnow documentation: must match at least 1 char
            take_till(1.., b"\\\""),
            '\\',
            alt((
                b'\\'.value(B(b"\\")),
                b'"'.value(B(b"\"")),
                hex.value(B(b"\x7f")),
            )),
        ),
        b'"',
    )
    .parse_next(input)
}

fn hex(input: &mut &[u8]) -> PResult<u8> {
    preceded(b'x', take(2_usize))
        .and_then(hex_uint)
        .parse_next(input)
}
