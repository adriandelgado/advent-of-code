use std::collections::HashMap;

use ndarray::{Array2, ArrayView2, ArrayViewMut2};

pub fn part1(input: &str) -> usize {
    let num_rows = input.lines().count();
    let num_cols = input.find('\n').unwrap();
    let grid = ArrayView2::from_shape((num_rows, num_cols + 1), input.as_bytes()).unwrap();

    let mut total_load = 0;
    for column in grid.columns().into_iter().take(num_cols) {
        let mut next_load_amount = num_rows;

        for (rock, load) in column.iter().zip((1..=num_rows).rev()) {
            match rock {
                b'#' => {
                    next_load_amount = load - 1;
                }
                b'O' => {
                    total_load += next_load_amount;
                    next_load_amount -= 1;
                }
                _ => {}
            }
        }
    }

    total_load
}

const SIMULS: usize = 1_000_000_000;

pub fn part2(input: &str) -> usize {
    let num_rows = input.lines().count();
    let num_cols = input.find('\n').unwrap();
    let mut grid = Array2::from_shape_fn((num_rows, num_cols), |(r, c)| {
        input.as_bytes()[r * (num_cols + 1) + c]
    });

    let mut seen = HashMap::new();
    let mut previous_loads: Vec<Array2<u8>> = Vec::new();

    for s in 0..SIMULS {
        if let Some(last_s) = seen.insert(grid.clone(), s) {
            let target_simul = (SIMULS - last_s) % (s - last_s) + last_s;
            return get_north_load(previous_loads[target_simul].view());
        }
        run_simulation(grid.view_mut());
        previous_loads.push(grid.clone());
    }
    get_north_load(grid.view())
}

fn get_north_load(grid: ArrayView2<u8>) -> usize {
    grid.rows()
        .into_iter()
        .zip((1..=grid.dim().0).rev())
        .map(|(row, load)| row.iter().filter(|&&ch| ch == b'O').count() * load)
        .sum()
}

fn run_simulation(mut grid: ArrayViewMut2<u8>) {
    let (num_rows, num_cols) = grid.dim();

    // north
    for mut column in grid.columns_mut() {
        let mut next_row_idx = 0;

        for row_idx in 0..num_rows {
            let rock = column[row_idx];
            match rock {
                b'#' => {
                    next_row_idx = row_idx + 1;
                }
                b'O' => {
                    column[row_idx] = b'.';
                    column[next_row_idx] = b'O';
                    next_row_idx += 1;
                }
                _ => {}
            }
        }
    }

    // west
    for mut row in grid.rows_mut() {
        let mut next_col_idx = 0;

        for col_idx in 0..num_cols {
            let rock = row[col_idx];
            match rock {
                b'#' => {
                    next_col_idx = col_idx + 1;
                }
                b'O' => {
                    row[col_idx] = b'.';
                    row[next_col_idx] = b'O';
                    next_col_idx += 1;
                }
                _ => {}
            }
        }
    }

    // south
    for mut column in grid.columns_mut() {
        let mut next_row_idx = num_rows - 1;

        for row_idx in (0..num_rows).rev() {
            let rock = column[row_idx];
            match rock {
                b'#' => {
                    next_row_idx = row_idx.saturating_sub(1);
                }
                b'O' => {
                    column[row_idx] = b'.';
                    column[next_row_idx] = b'O';
                    next_row_idx = next_row_idx.saturating_sub(1);
                }
                _ => {}
            }
        }
    }

    // east
    for mut row in grid.rows_mut() {
        let mut next_col_idx = num_cols - 1;

        for col_idx in (0..num_cols).rev() {
            let rock = row[col_idx];
            match rock {
                b'#' => {
                    next_col_idx = col_idx.saturating_sub(1);
                }
                b'O' => {
                    row[col_idx] = b'.';
                    row[next_col_idx] = b'O';
                    next_col_idx = next_col_idx.saturating_sub(1);
                }
                _ => {}
            }
        }
    }
}
