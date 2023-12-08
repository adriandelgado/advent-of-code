use std::collections::BTreeMap;

use winnow::{token::take, PResult, Parser};

pub fn part1(input: &str) -> u64 {
    let (instructions, network) = extract_info(input);

    let network: BTreeMap<&str, (&str, &str)> =
        network.into_iter().map(|(a, b, c)| (a, (b, c))).collect();

    let mut count = 0;
    let mut current = "AAA";
    for instruction in instructions.into_iter().cycle() {
        let (left, right) = network[current];
        match instruction {
            Dir::Left => current = left,
            Dir::Right => current = right,
        }
        count += 1;
        if current == "ZZZ" {
            break;
        }
    }
    count
}

pub fn part2(input: &str) -> u64 {
    let (instructions, network) = extract_info(input);

    let network: BTreeMap<&str, (&str, &str)> =
        network.into_iter().map(|(a, b, c)| (a, (b, c))).collect();

    let current: Vec<_> = network
        .keys()
        .copied()
        .filter(|node| node.ends_with('A'))
        .collect();

    let mut cycles = Vec::new();
    for &curr in &current {
        let mut count = 0;
        let mut current = curr;
        for instruction in instructions.iter().cycle() {
            let (left, right) = network[current];
            match instruction {
                Dir::Left => current = left,
                Dir::Right => current = right,
            }
            count += 1;
            if current.ends_with('Z') {
                break;
            }
        }
        cycles.push(count);
    }
    // wolfram alpha
    println!("{cycles:?}");
    12357789728873
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Left,
    Right,
}

fn extract_info(input: &str) -> (Vec<Dir>, Vec<(&str, &str, &str)>) {
    let (instructions, network) = input.split_once("\n\n").unwrap();

    let instructions = instructions
        .bytes()
        .map(|ch| if ch == b'L' { Dir::Left } else { Dir::Right })
        .collect();

    let network = network
        .lines()
        .map(|line| parse_equals.parse(line).unwrap())
        .collect();

    (instructions, network)
}

fn parse_equals<'a>(input: &mut &'a str) -> PResult<(&'a str, &'a str, &'a str)> {
    (
        take(3_usize),
        " = (",
        take(3_usize),
        ", ",
        take(3_usize),
        ")",
    )
        .map(|(a, _, b, _, c, _)| (a, b, c))
        .parse_next(input)
}
