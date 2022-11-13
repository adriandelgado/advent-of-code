use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map, map_res},
    multi::fold_many1,
    sequence::tuple,
    IResult,
};

pub(super) fn part1(input: &str) -> String {
    fold_many1(
        map(parse_line, wrapping_paper),
        || 0,
        |acc, item| acc + item,
    )(input)
    .unwrap()
    .1
    .to_string()
}

pub(super) fn part2(input: &str) -> String {
    fold_many1(map(parse_line, ribbon), || 0, |acc, item| acc + item)(input)
        .unwrap()
        .1
        .to_string()
}

fn wrapping_paper((num1, num2, num3): (u32, u32, u32)) -> u32 {
    let side1 = num1 * num2;
    let side2 = num2 * num3;
    let side3 = num3 * num1;

    let slack = side1.min(side2).min(side3);

    2 * (side1 + side2 + side3) + slack
}

fn ribbon((num1, num2, num3): (u32, u32, u32)) -> u32 {
    let max_edge = num1.max(num2).max(num3);

    let ribbon_present = 2 * (num1 + num2 + num3 - max_edge);

    let volume = num1 * num2 * num3;

    ribbon_present + volume
}

fn parse_line(input: &str) -> IResult<&str, (u32, u32, u32)> {
    map(
        tuple((
            parse_number,
            tag("x"),
            parse_number,
            tag("x"),
            parse_number,
            newline,
        )),
        |(num1, _, num2, _, num3, _)| (num1, num2, num3),
    )(input)
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}
