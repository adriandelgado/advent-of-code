use std::iter;

use winnow::{
    ascii::alpha1,
    combinator::{alt, preceded},
    token::any,
    PResult, Parser,
};

// 511416
pub fn part1(input: &str) -> usize {
    input.trim().split(',').map(str::as_bytes).map(hash).sum()
}

fn hash(input: &[u8]) -> usize {
    let mut current_value: u8 = 0;
    for &ch in input {
        current_value = current_value.wrapping_add(ch);
        current_value = current_value.wrapping_mul(17);
    }
    current_value as usize
}

// 290779 202 - 215
pub fn part2(input: &str) -> usize {
    let mut lenses: Vec<Vec<(&[u8], usize)>> = Vec::with_capacity(255);
    for _ in 0..255 {
        lenses.push(Vec::new());
    }

    for ((box_idx, label), op) in input
        .trim_end()
        .split(',')
        .map(str::as_bytes)
        .map(|op| operation.parse(op).unwrap())
    {
        let maybe_idx = lenses[box_idx].iter().position(|&(b, _)| b == label);
        match (op, maybe_idx) {
            (Op::Dash, Some(label_idx)) => {
                lenses[box_idx].remove(label_idx);
            }
            (Op::Eq(focal_lenght), Some(label_idx)) => {
                lenses[box_idx][label_idx].1 = focal_lenght;
            }
            (Op::Eq(focal_lenght), None) => {
                lenses[box_idx].push((label, focal_lenght));
            }
            _ => {}
        }
    }

    let mut focusing_power = 0;

    for (lens_box, box_num) in iter::zip(lenses, 1..) {
        for ((_, focal_len), slot) in iter::zip(lens_box, 1..) {
            focusing_power += box_num * slot * focal_len;
        }
    }

    focusing_power
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Dash,
    Eq(usize),
}

fn operation<'a>(input: &mut &'a [u8]) -> PResult<((usize, &'a [u8]), Op)> {
    (
        alpha1.map(|label| (hash(label), label)),
        alt((
            preceded(b'=', any).map(|fl| Op::Eq((fl - b'0') as usize)),
            b'-'.value(Op::Dash),
        )),
    )
        .parse_next(input)
}
