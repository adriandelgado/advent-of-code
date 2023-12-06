use winnow::{
    ascii::{alpha1, dec_uint, space1},
    combinator::{preceded, rest, separated, separated_pair, terminated},
    token::take_till1,
    PResult, Parser,
};

pub fn part1(input: &str) -> usize {
    let (time, distance) = extract_info_1.parse(input).unwrap();

    std::iter::zip(time, distance)
        .map(|(t, d)| (1..=t).filter(|hold| (t - hold) * hold > d).count())
        .product()
}

pub fn part2(input: &str) -> u64 {
    let (t, d) = extract_info_2.parse(input).unwrap();

    amount_ways_beating((t, d))
}

#[allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss
)]
fn amount_ways_beating((t, d): (u64, u64)) -> u64 {
    let (t, d) = (t as f64, d as f64);
    let discriminant = t * t - 4.0 * d;
    let start = (t - f64::sqrt(discriminant)) / 2.0;
    let end = (t + f64::sqrt(discriminant)) / 2.0;

    ((end - 1.0).ceil() - (start + 1.0).floor() + 1.0) as u64
}

fn extract_info_1(input: &mut &str) -> PResult<(Vec<u64>, Vec<u64>)> {
    terminated(
        separated_pair(get_several_nums, '\n', get_several_nums),
        rest,
    )
    .parse_next(input)
}

fn get_several_nums(input: &mut &str) -> PResult<Vec<u64>> {
    preceded(
        (alpha1, ":", space1),
        separated(4, dec_uint::<_, u64, _>, space1),
    )
    .parse_next(input)
}

fn extract_info_2(input: &mut &str) -> PResult<(u64, u64)> {
    terminated(separated_pair(get_one_num, '\n', get_one_num), rest).parse_next(input)
}

fn get_one_num(input: &mut &str) -> PResult<u64> {
    preceded(
        (alpha1, ":", space1),
        take_till1('\n').map(parse_spaced_num),
    )
    .parse_next(input)
}

fn parse_spaced_num(input: &str) -> u64 {
    input
        .as_bytes()
        .iter()
        .rev()
        .filter(|&&ch| ch != b' ')
        .map(|ch| u64::from(ch - b'0'))
        .zip(0..)
        .map(|(d, pos)| d * 10_u64.pow(pos))
        .sum()
}
