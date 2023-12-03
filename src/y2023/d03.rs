use std::collections::{BTreeMap, BTreeSet};

use winnow::{
    ascii::digit1,
    error::{ContextError, ErrMode},
    Parser,
};

pub fn part1(input: &str) -> u32 {
    let schematic: Vec<&str> = input.lines().collect();
    let mut all_numbers = Vec::new();
    let mut symbols = BTreeSet::new();

    for (row, line) in schematic.iter().enumerate() {
        let mut col = 0;
        while col < line.len() {
            let slice = &line[col..];
            if let Ok::<(&str, &str), ErrMode<ContextError>>((_, num)) = digit1.parse_peek(slice) {
                all_numbers.push((num, (row, col)));
                col += num.len();
            } else {
                col += 1;
            }
        }

        for (col, ch) in line.chars().enumerate() {
            match ch {
                '0'..='9' | '.' => {}
                _ => {
                    symbols.insert((row, col));
                }
            }
        }
    }
    all_numbers
        .into_iter()
        .filter(|&(num_str, coords)| has_adjacent_sym(num_str, coords, &symbols))
        .map(|(num_str, _)| num_str.parse::<u32>().unwrap())
        .sum()
}

fn has_adjacent_sym(
    num_str: &str,
    (x, y): (usize, usize),
    symbols: &BTreeSet<(usize, usize)>,
) -> bool {
    for y_coord in y.saturating_sub(1)..=(y + num_str.len()) {
        for x_coord in x.saturating_sub(1)..=(x + 1) {
            if symbols.contains(&(x_coord, y_coord)) {
                return true;
            }
        }
    }
    false
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
