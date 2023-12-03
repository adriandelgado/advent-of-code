use std::collections::{BTreeMap, BTreeSet};

use atoi::FromRadix10;
use winnow::{
    ascii::digit1,
    error::{ContextError, ErrMode},
    Parser,
};

pub fn part1(input: &str) -> u32 {
    let modulo = input.find('\n').unwrap() + 1;

    input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            let mut sum = 0;
            let mut col = 0;
            while col < line.len() {
                let (number, len) = u32::from_radix_10(line[col..].as_bytes());
                if len > 0 && has_adjacent_sym(len, (row, col), input.as_bytes(), modulo) {
                    sum += number;
                    col += len;
                } else {
                    col += 1;
                }
            }
            sum
        })
        .sum()
}

fn has_adjacent_sym(len: usize, (row, col): (usize, usize), input: &[u8], modulo: usize) -> bool {
    for surr_col in col.saturating_sub(1)..=(col + len).min(modulo - 2) {
        if !matches!(
            input[row.saturating_sub(1) * modulo + surr_col],
            b'0'..=b'9' | b'.'
        ) || !matches!(
            input[((row + 1) * modulo).min(input.len() - modulo) + surr_col],
            b'0'..=b'9' | b'.'
        ) {
            return true;
        }
    }

    !matches!(
        input[row * modulo + col.saturating_sub(1)],
        b'0'..=b'9' | b'.'
    ) || !matches!(
        input[row * modulo + (col + len).min(modulo - 2)],
        b'0'..=b'9' | b'.'
    )
}

pub fn part2(input: &str) -> u32 {
    let schematic: Vec<&str> = input.lines().collect();
    let mut all_numbers = BTreeMap::new();
    let mut maybe_gears = Vec::new();

    for (row, line) in schematic.iter().enumerate() {
        let mut col = 0;
        while col < line.len() {
            let slice = &line[col..];
            if let Ok::<(&str, &str), ErrMode<ContextError>>((_, num)) = digit1.parse_peek(slice) {
                for moving_col in col..(col + num.len()) {
                    all_numbers.insert((row, moving_col), (num, (row, col)));
                }
                col += num.len();
            } else {
                col += 1;
            }
        }

        for (col, ch) in line.chars().enumerate() {
            match ch {
                '*' => {
                    maybe_gears.push((row, col));
                }
                _ => {}
            }
        }
    }
    maybe_gears
        .into_iter()
        .filter_map(|coords| has_2_adjacent_nums(coords, &all_numbers))
        .sum()
}

fn has_2_adjacent_nums(
    (x, y): (usize, usize),
    all_numbers: &BTreeMap<(usize, usize), (&str, (usize, usize))>,
) -> Option<u32> {
    let mut nums = BTreeSet::new();
    for y_coord in y.saturating_sub(1)..=(y + 1) {
        for x_coord in x.saturating_sub(1)..=(x + 1) {
            if let Some(&num_data) = all_numbers.get(&(x_coord, y_coord)) {
                nums.insert(num_data);
            }
        }
    }
    (nums.len() == 2).then(|| {
        nums.into_iter()
            .map(|(num, _)| num.parse::<u32>().unwrap())
            .product()
    })
}
