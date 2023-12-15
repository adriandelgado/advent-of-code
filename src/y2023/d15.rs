use winnow::{
    ascii::alpha1,
    combinator::{dispatch, fail, success},
    token::any,
    PResult, Parser,
};

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

pub fn part2(input: &str) -> usize {
    let mut lenses: Vec<Vec<(&[u8], usize)>> = Vec::with_capacity(255);
    for _ in 0..255 {
        lenses.push(Vec::new());
    }

    for (label, op) in input
        .trim_end()
        .split(',')
        .map(str::as_bytes)
        .map(|op| operation.parse(op).unwrap())
    {
        let current_box = &mut lenses[hash(label)];
        match (op, current_box.iter().position(|&(b, _)| b == label)) {
            (Op::Insert(focal_lenght), Some(label_idx)) => {
                current_box[label_idx].1 = focal_lenght;
            }
            (Op::Insert(focal_lenght), None) => {
                current_box.push((label, focal_lenght));
            }
            (Op::Remove, Some(label_idx)) => {
                current_box.remove(label_idx);
            }
            (Op::Remove, None) => {}
        }
    }

    let mut focusing_power = 0;

    for (box_num, lens_box) in std::iter::zip(1.., lenses) {
        for (slot, (_, focal_len)) in std::iter::zip(1.., lens_box) {
            focusing_power += box_num * slot * focal_len;
        }
    }

    focusing_power
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Remove,
    Insert(usize),
}

fn operation<'a>(input: &mut &'a [u8]) -> PResult<(&'a [u8], Op)> {
    (
        alpha1,
        dispatch! {any;
            b'=' => any.map(|fl| Op::Insert((fl - b'0') as usize)),
            b'-' => success(Op::Remove),
            _ => fail
        },
    )
        .parse_next(input)
}
