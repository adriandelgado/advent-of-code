use winnow::{
    ascii::{dec_uint, newline},
    combinator::{fold_repeat, terminated},
    PResult, Parser,
};

pub fn part1(input: &str) -> u32 {
    fold_repeat(
        1..,
        parse_line.map(wrapping_paper),
        || 0,
        |acc, item| acc + item,
    )
    .parse(input)
    .unwrap()
}

pub fn part2(input: &str) -> u32 {
    fold_repeat(1.., parse_line.map(ribbon), || 0, |acc, item| acc + item)
        .parse(input)
        .unwrap()
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

fn parse_line(input: &mut &str) -> PResult<(u32, u32, u32)> {
    terminated((dec_uint, 'x', dec_uint, 'x', dec_uint), newline)
        .map(|(num1, _, num2, _, num3)| (num1, num2, num3))
        .parse_next(input)
}
