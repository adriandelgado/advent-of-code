use std::collections::BTreeMap;

use winnow::{
    ascii::dec_uint,
    combinator::{alt, separated, separated_pair},
    PResult, Parser,
};

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| extract_info.parse(line).unwrap())
        .filter_map(|(game_id, games)| {
            games
                .into_iter()
                .all(|(num, cube)| match cube {
                    Color::Red => num <= 12,
                    Color::Green => num <= 13,
                    Color::Blue => num <= 14,
                })
                .then_some(game_id)
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| extract_info.parse(line).unwrap())
        .map(|(_, games)| -> u32 {
            let mut max_values = BTreeMap::new();
            for (num, cube) in games {
                max_values
                    .entry(cube)
                    .and_modify(|n| *n = num.max(*n))
                    .or_insert(num);
            }
            max_values.values().product()
        })
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Color {
    Red,
    Green,
    Blue,
}

fn extract_info(input: &mut &str) -> PResult<(u32, Vec<(u32, Color)>)> {
    ("Game ", dec_uint, ": ", games)
        .map(|(_, game_id, _, games)| (game_id, games))
        .parse_next(input)
}

fn games(input: &mut &str) -> PResult<Vec<(u32, Color)>> {
    separated(
        1..,
        separated_pair(
            dec_uint,
            " ",
            alt((
                "green".value(Color::Green),
                "red".value(Color::Red),
                "blue".value(Color::Blue),
            )),
        ),
        alt((", ", "; ")),
    )
    .parse_next(input)
}
