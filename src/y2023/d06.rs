use atoi::FromRadix10;
use winnow::{
    combinator::{preceded, rest, separated_pair, terminated},
    token::{take, take_till},
    PResult, Parser,
};

pub fn part1(input: &str) -> u32 {
    let (time, distance) = parse_time_distance.parse(input).unwrap();

    std::iter::zip(num_iter(time), num_iter(distance))
        .map(|(t, d)| -> u32 { (1..=t).map(|hold| u32::from((t - hold) * hold > d)).sum() })
        .product()
}

pub fn part2(input: &str) -> u32 {
    let (t, d) = parse_time_distance.parse(input).unwrap();

    amount_ways_beating((parse_spaced_num(t), parse_spaced_num(d)))
}

fn num_iter(input: &str) -> impl Iterator<Item = u32> + '_ {
    input
        .split_ascii_whitespace()
        .map(|n| u32::from_radix_10(n.as_bytes()).0)
}

#[allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss
)]
fn amount_ways_beating((t, d): (u64, u64)) -> u32 {
    let (t, d) = (t as f64, d as f64);
    let discriminant = t.mul_add(t, -4.0 * d);
    let start = (t - f64::sqrt(discriminant)) / 2.0;
    let end = (t + f64::sqrt(discriminant)) / 2.0;

    ((end - 1.0).ceil() - (start + 1.0).floor() + 1.0) as u32
}

fn parse_time_distance<'a>(input: &mut &'a str) -> PResult<(&'a str, &'a str)> {
    terminated(
        separated_pair(
            preceded(take(13_usize), take_till(1.., '\n')),
            '\n',
            preceded(take(12_usize), take_till(1.., '\n')),
        ),
        rest,
    )
    .parse_next(input)
}

fn parse_spaced_num(input: &str) -> u64 {
    input
        .bytes()
        .rev()
        .filter(|&ch| ch != b' ')
        .map(|ch| u64::from(ch - b'0'))
        .zip(0..)
        .map(|(d, pos)| d * 10_u64.pow(pos))
        .sum()
}
