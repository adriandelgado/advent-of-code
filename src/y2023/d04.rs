use winnow::{
    ascii::{dec_uint, space0, space1},
    combinator::{opt, preceded, separated, separated_pair},
    token::take,
    PResult, Parser,
};

pub fn part1(input: &str) -> u16 {
    input
        .lines()
        .map(|line| extract_info.parse(line).unwrap())
        .map(|(have, winning)| {
            let matching = have.into_iter().filter(|h| winning.contains(h)).count();

            1 << matching >> 1
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut cards: Vec<_> = input
        .lines()
        .map(|line| extract_info.parse(line).unwrap())
        .map(|(have, winning)| {
            let matching = have.into_iter().filter(|h| winning.contains(h)).count();
            (1, matching)
        })
        .collect();

    for idx in 0..cards.len() {
        let (amount, matching) = cards[idx];
        for next_card in cards.iter_mut().skip(idx + 1).take(matching) {
            next_card.0 += amount;
        }
    }

    cards.into_iter().map(|(amount, _)| amount).sum()
}

fn extract_info(input: &mut &str) -> PResult<(Vec<u8>, Vec<u8>)> {
    preceded(
        (take(10_usize), opt(" ")),
        separated_pair(
            separated(10, dec_uint::<_, u8, _>, space1),
            (" | ", space0),
            separated(25, dec_uint::<_, u8, _>, space1),
        ),
    )
    .parse_next(input)
}
