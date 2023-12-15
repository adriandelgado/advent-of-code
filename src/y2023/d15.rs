use winnow::{
    ascii::{alpha1, dec_uint},
    combinator::{alt, preceded},
    PResult, Parser,
};

pub fn part1(input: &str) -> usize {
    input.trim_end().split(',').map(hash).sum()
}

fn hash(input: &str) -> usize {
    let mut current_value = 0;
    for ch in input.bytes() {
        current_value += ch as usize;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

pub fn part2(input: &str) -> usize {
    let mut lenses: Vec<Vec<(&str, usize)>> = Vec::with_capacity(255);
    for _ in 0..255 {
        lenses.push(Vec::new());
    }

    for ((box_idx, label), op) in input
        .trim_end()
        .split(',')
        .map(|op| operation.parse(op).unwrap())
    {
        match op {
            Op::Dash => {
                if let Some(idx) = lenses[box_idx].iter().position(|&(b, _)| b == label) {
                    lenses[box_idx].remove(idx);
                }
            }
            Op::Eq(focal_lenght) => {
                if let Some(idx) = lenses[box_idx].iter().position(|&(b, _)| b == label) {
                    lenses[box_idx][idx] = (label, focal_lenght);
                } else {
                    lenses[box_idx].push((label, focal_lenght))
                }
            }
        }
    }

    let mut focusing_power = 0;

    for (lens_box, box_num) in lenses.into_iter().zip(1..) {
        for ((_, focal_len), slot) in lens_box.into_iter().zip(1..) {
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

fn operation<'a>(input: &mut &'a str) -> PResult<((usize, &'a str), Op)> {
    (
        alpha1.map(|l| (hash(l), l)),
        alt((
            preceded('=', dec_uint).map(|fl: u64| Op::Eq(fl as usize)),
            '-'.value(Op::Dash),
        )),
    )
        .parse_next(input)
}
