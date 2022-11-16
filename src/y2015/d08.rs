use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, is_not, tag, take},
    combinator::value,
    number::complete::hex_u32,
    sequence::{delimited, preceded},
    IResult,
};

pub(super) fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let escaped = parse_line(line.as_bytes()).unwrap().1.len();
            line.len() - escaped
        })
        .sum::<usize>()
        .to_string()
}

pub(super) fn part2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let escaped = line.escape_debug().to_string();
            escaped.len() + 2 - line.len()
        })
        .sum::<usize>()
        .to_string()
}

fn parse_line(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    delimited(
        tag("\""),
        escaped_transform(
            is_not("\\\""),
            '\\',
            alt((
                value(&b"\\"[..], tag("\\")),
                value(&b"\""[..], tag("\"")),
                value(&b"\x7f"[..], hex),
            )),
        ),
        tag("\""),
    )(input)
}

// TODO: PR nom to include u8 (same as char)
fn hex(input: &[u8]) -> IResult<&[u8], u8> {
    let (rest, maybe_num) = preceded(tag("x"), take(2usize))(input)?;
    let (_, num) = hex_u32(maybe_num)?;
    Ok((rest, num.try_into().unwrap()))
}
