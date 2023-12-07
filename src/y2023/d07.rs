use winnow::{ascii::dec_uint, combinator::separated_pair, token::take, PResult, Parser};

pub fn solve<const PART_1: bool>(input: &str) -> u32 {
    let mut bets: Vec<_> = input
        .lines()
        .map(|line| parse_hand::<PART_1>.parse(line).unwrap())
        .map(|(cards, bid)| {
            let ty = HandType::from_cards::<PART_1>(cards);
            (ty, cards, bid)
        })
        .collect();

    bets.sort_unstable();

    bets.into_iter()
        .zip(1..)
        .map(|((_, _, bid), rank)| rank * bid)
        .sum()
}

pub fn part1(input: &str) -> u32 {
    solve::<true>(input)
}

pub fn part2(input: &str) -> u32 {
    solve::<false>(input)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OneAir,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_cards<const PART_1: bool>(cards: [u8; 5]) -> Self {
        let mut counter = [0; 14];

        for card in cards {
            counter[card as usize] += 1;
        }

        let [joker_count, mut counter @ ..] = counter;

        if !PART_1 {
            *counter.iter_mut().max().unwrap() += joker_count;
        }

        let mut max_count = 1;
        let mut different_cards = 0;
        for count in counter {
            different_cards += u8::from(count != 0);
            max_count = max_count.max(count);
        }

        match 4 + max_count - different_cards {
            0 => Self::HighCard,
            2 => Self::OneAir,
            3 => Self::TwoPair,
            4 => Self::ThreeOfAKind,
            5 => Self::FullHouse,
            6 => Self::FourOfAKind,
            _ => Self::FiveOfAKind,
        }
    }
}

fn parse_hand<const PART_1: bool>(input: &mut &str) -> PResult<([u8; 5], u32)> {
    separated_pair(
        take(5_usize).map(|cards: &str| {
            assert_eq!(cards.len(), 5);
            std::array::from_fn(|idx| parse_card::<PART_1>(cards.as_bytes()[idx]))
        }),
        ' ',
        dec_uint,
    )
    .parse_next(input)
}

fn parse_card<const PART_1: bool>(ch: u8) -> u8 {
    (match ch {
        b'A' => 14,
        b'K' => 13,
        b'Q' => 12,
        b'J' => {
            if PART_1 {
                11
            } else {
                1
            }
        }
        b'T' => 10,
        digit => digit - b'0',
    } - 1) // To start at 0, we shift down one unit
}
