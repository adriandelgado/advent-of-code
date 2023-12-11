#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]

use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let mut new_rows_idx = vec![];
    for (row_idx, row) in grid.iter().enumerate() {
        if row.iter().all(|&ch| ch == b'.') {
            new_rows_idx.push(row_idx);
        }
    }
    let mut new_cols_idx = vec![];
    for col in 0..grid[0].len() {
        if grid.iter().all(|row| row[col] == b'.') {
            new_cols_idx.push(col);
        }
    }

    let mut galaxies = vec![];
    for (row_idx, row) in grid.into_iter().enumerate() {
        for (col_idx, &ch) in row.into_iter().enumerate() {
            if ch == b'#' {
                galaxies.push((row_idx, col_idx));
            }
        }
    }

    let mut sum = 0;
    for pair in galaxies.iter().combinations(2) {
        let [&(x_0, y_0), &(x_1, y_1)] = &pair[..] else {
            unreachable!()
        };
        let extra_rows = new_rows_idx
            .iter()
            .filter(|&&row_idx| {
                (x_0 < row_idx && row_idx < x_1) || (x_1 < row_idx && row_idx < x_0)
            })
            .count()
            * 1;
        let extra_cols = new_cols_idx
            .iter()
            .filter(|&&col_idx| {
                (y_0 < col_idx && col_idx < y_1) || (y_1 < col_idx && col_idx < y_0)
            })
            .count()
            * 1;
        sum += x_0.abs_diff(x_1) + y_0.abs_diff(y_1) + extra_rows + extra_cols;
    }
    sum
}

pub fn part2(input: &str) -> usize {
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let mut new_rows_idx = vec![];
    for (row_idx, row) in grid.iter().enumerate() {
        if row.iter().all(|&ch| ch == b'.') {
            new_rows_idx.push(row_idx);
        }
    }
    let mut new_cols_idx = vec![];
    for col in 0..grid[0].len() {
        if grid.iter().all(|row| row[col] == b'.') {
            new_cols_idx.push(col);
        }
    }

    let mut galaxies = vec![];
    for (row_idx, row) in grid.into_iter().enumerate() {
        for (col_idx, &ch) in row.into_iter().enumerate() {
            if ch == b'#' {
                galaxies.push((row_idx, col_idx));
            }
        }
    }

    let mut sum = 0;
    for pair in galaxies.iter().combinations(2) {
        let [&(x_0, y_0), &(x_1, y_1)] = &pair[..] else {
            unreachable!()
        };
        let extra_rows = new_rows_idx
            .iter()
            .filter(|&&row_idx| {
                (x_0 < row_idx && row_idx < x_1) || (x_1 < row_idx && row_idx < x_0)
            })
            .count()
            * 999_999;
        let extra_cols = new_cols_idx
            .iter()
            .filter(|&&col_idx| {
                (y_0 < col_idx && col_idx < y_1) || (y_1 < col_idx && col_idx < y_0)
            })
            .count()
            * 999_999;
        sum += x_0.abs_diff(x_1) + y_0.abs_diff(y_1) + extra_rows + extra_cols;
    }
    sum
}
