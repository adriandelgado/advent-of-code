use std::collections::BTreeSet;

use winnow::{
    ascii::{dec_uint, digit1, space1},
    combinator::{preceded, separated, separated_pair},
    PResult, Parser,
};

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| extract_info.parse(line).unwrap())
        .map(|(have, winning)| {
            let have = BTreeSet::from_iter(have);
            let winning = BTreeSet::from_iter(winning);
            let matching = have.intersection(&winning).count();
            if matching == 0 {
                0
            } else {
                1 << (matching - 1)
            }
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut cards: Vec<_> = input
        .lines()
        .map(|line| extract_info.parse(line).unwrap())
        .map(|(have, winning)| {
            let have = BTreeSet::from_iter(have);
            let winning = BTreeSet::from_iter(winning);
            let matching = have.intersection(&winning).count();
            (1, matching)
        })
        .collect();

    for idx in 0..cards.len() {
        let (amount, matching) = cards[idx];
        for next_idx in (idx + 1)..(idx + 1 + matching) {
            cards[next_idx].0 += amount;
        }
    }

    cards.into_iter().map(|(amount, _)| amount).sum()
}

fn extract_info(input: &mut &str) -> PResult<(Vec<u8>, Vec<u8>)> {
    preceded(
        ("Card", space1, digit1, ":", space1),
        separated_pair(
            separated(1.., dec_uint::<_, u8, _>, space1),
            (space1, "|", space1),
            separated(1.., dec_uint::<_, u8, _>, space1),
        ),
    )
    .parse_next(input)
}
