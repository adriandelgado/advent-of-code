use std::collections::HashMap;

use winnow::{
    combinator::{delimited, rest, separated, separated_pair, terminated},
    token::{take, take_till},
    PResult, Parser,
};

const AAA: u16 = node_to_u16(b"AAA");
const ZZZ: u16 = node_to_u16(b"ZZZ");

pub fn part1(input: &str) -> u16 {
    let (instructions, network) = extract_info.parse(input.as_bytes()).unwrap();

    let mut count = 0;
    let mut current = AAA;
    for instruction in instructions.iter().cycle() {
        let (left, right) = network[&current];
        match instruction {
            b'L' => current = left,
            _ => current = right,
        }
        count += 1;
        if current == ZZZ {
            break;
        }
    }
    count
}

pub fn part2(input: &str) -> u64 {
    let (instructions, network) = extract_info.parse(input.as_bytes()).unwrap();

    let mut lcm_out = 1;
    for curr in network.keys().copied().filter(|node| node % 26 == 0) {
        let mut count = 0;
        let mut current = curr;
        for instruction in instructions.iter().cycle() {
            let (left, right) = network[&current];
            match instruction {
                b'L' => current = left,
                _ => current = right,
            }
            count += 1;
            if current % 26 == 25 {
                break;
            }
        }
        lcm_out = lcm(lcm_out, count);
    }
    lcm_out
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == b {
        return a;
    }
    if b > a {
        (a, b) = (b, a);
    }
    while b > 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

type Pair = (u16, u16);

fn extract_info<'a>(input: &mut &'a [u8]) -> PResult<(&'a [u8], HashMap<u16, Pair>)> {
    terminated(
        separated_pair(take_till(1.., b'\n'), b"\n\n", separated(1.., node, b'\n')),
        rest,
    )
    .parse_next(input)
}

fn node(input: &mut &[u8]) -> PResult<(u16, Pair)> {
    separated_pair(
        take(3_usize).map(node_to_u16),
        b" = ",
        delimited(
            b'(',
            separated_pair(
                take(3_usize).map(node_to_u16),
                b", ",
                take(3_usize).map(node_to_u16),
            ),
            b')',
        ),
    )
    .parse_next(input)
}

const fn node_to_u16(input: &[u8]) -> u16 {
    if let &[c, d, u] = input {
        let c = (c - b'A') as u16;
        let d = (d - b'A') as u16;
        let u = (u - b'A') as u16;
        u + d * 26 + c * 26 * 26
    } else {
        0
    }
}
