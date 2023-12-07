use std::collections::BTreeMap;

use winnow::{
    ascii::dec_uint,
    combinator::{alt, repeat, separated_pair},
    token::take,
    PResult, Parser,
};

pub fn part1(input: &str) -> u64 {
    let mut bets: Vec<_> = input
        .lines()
        .map(|line| parse_cards::<true>.parse(line).unwrap())
        .map(|(cards, bid)| {
            let t = get_type_of_hand(cards);
            (t, cards, bid)
        })
        .collect();

    bets.sort_unstable();

    bets.into_iter()
        .zip(1..)
        .map(|((_, _, bid), rank)| rank * bid)
        .sum()
}

fn get_type_of_hand(cards: [u8; 5]) -> CardType {
    let mut counter = BTreeMap::<u8, u8>::new();

    for card in cards {
        counter.entry(card).and_modify(|c| *c += 1).or_insert(1);
    }

    match counter.len() {
        1 => CardType::FiveOfAKind,
        4 => CardType::OneAir,
        5 => CardType::HighCard,
        count => {
            let max_count = *counter.values().max().unwrap();
            match (count, max_count) {
                (_, 4) => CardType::FourOfAKind,
                (_, 2) => CardType::TwoPair,
                (3, _) => CardType::ThreeOfAKind,
                _ => CardType::FullHouse,
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CardType {
    HighCard,
    OneAir,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub fn part2(input: &str) -> u64 {
    let mut bets: Vec<_> = input
        .lines()
        .map(|line| parse_cards::<false>.parse(line).unwrap())
        .map(|(cards, bid)| {
            let t = get_type_of_hand_jokers(cards);
            (t, cards, bid)
        })
        .collect();

    bets.sort_unstable();

    bets.into_iter()
        .zip(1..)
        .map(|((_, _, bid), rank)| rank * bid)
        .sum()
}

fn get_type_of_hand_jokers(cards: [u8; 5]) -> CardType {
    // check if not jokers
    if !cards.contains(&1) {
        return get_type_of_hand(cards);
    }
    let mut counter = BTreeMap::<u8, u8>::new();

    for card in cards {
        counter.entry(card).and_modify(|c| *c += 1).or_insert(1);
    }

    let joker_count = counter.remove(&1).unwrap();

    if joker_count >= 4 {
        return CardType::FiveOfAKind;
    }

    let (max_count, max_card) = counter
        .iter()
        .map(|(&card, &count)| (count, card))
        .max()
        .unwrap();

    if joker_count == 3 {
        if max_count == 2 {
            return CardType::FiveOfAKind;
        } else {
            return CardType::FourOfAKind;
        }
    }

    *counter.get_mut(&max_card).unwrap() += joker_count;

    match counter.len() {
        1 => CardType::FiveOfAKind,
        4 => CardType::OneAir,
        5 => unreachable!(),
        count => {
            let max_count = *counter.values().max().unwrap();
            match (count, max_count) {
                (_, 4) => CardType::FourOfAKind,
                (_, 2) => CardType::TwoPair,
                (3, _) => CardType::ThreeOfAKind,
                _ => CardType::FullHouse,
            }
        }
    }
}

fn parse_cards<const PART_1: bool>(input: &mut &str) -> PResult<([u8; 5], u64)> {
    separated_pair(
        repeat(
            5,
            take(1_usize).and_then(alt((
                dec_uint,
                'A'.value(14),
                'K'.value(13),
                'Q'.value(12),
                'J'.value(if PART_1 { 11 } else { 1 }),
                'T'.value(10),
            ))),
        )
        .map(|vec: Vec<u8>| vec.try_into().unwrap()),
        ' ',
        dec_uint,
    )
    .parse_next(input)
}
