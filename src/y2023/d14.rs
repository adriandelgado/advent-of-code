use std::collections::HashMap;

use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut total_load = 0;
    for col_idx in 0..num_cols {
        let mut next_load_amount = num_rows;

        for (rock, load) in grid
            .iter()
            .map(|row| row[col_idx])
            .zip((1..=num_rows).rev())
        {
            match rock {
                b'#' => {
                    next_load_amount = load - 1;
                }
                b'O' => {
                    total_load += next_load_amount;
                    next_load_amount -= 1;
                }
                b'.' => {}
                _ => unreachable!(),
            }
        }
    }

    total_load
}

const SIMULS: u32 = 1_000_000_000;

pub fn part2(input: &str) -> usize {
    let mut grid: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();

    let mut seen = HashMap::new();

    for i in 0..SIMULS {
        let unique_id = get_id(&grid);
        if let Some(last_i) = seen.insert(unique_id, i) {
            let mut grid: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
            for _ in 0..(SIMULS - last_i) % (i - last_i) + last_i {
                run_simulation(&mut grid);
            }
            return get_north_load(&grid);
        }
        run_simulation(&mut grid);
    }
    get_north_load(&grid)
}

fn get_north_load(grid: &[Vec<u8>]) -> usize {
    let mut total_load = 0;

    for (row, load) in grid.iter().zip((1..=grid.len()).rev()) {
        total_load += row.iter().filter(|&&ch| ch == b'O').count() * load;
    }

    total_load
}

fn get_id(grid: &[Vec<u8>]) -> Vec<u64> {
    grid.iter()
        .flatten()
        .chunks(39)
        .into_iter()
        .map(|chunk| {
            chunk
                .zip(0..)
                .map(|(&ch, pos)| {
                    let digit = match ch {
                        b'#' => 2,
                        b'O' => 1,
                        _ => 0,
                    };

                    digit * 3_u64.pow(pos)
                })
                .sum()
        })
        .collect()
}

fn run_simulation(grid: &mut [Vec<u8>]) {
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    // north
    for col_idx in 0..num_cols {
        let mut next_row_idx = 0;

        for row_idx in 0..num_rows {
            let rock = grid[row_idx][col_idx];
            match rock {
                b'#' => {
                    next_row_idx = row_idx + 1;
                }
                b'O' => {
                    grid[row_idx][col_idx] = b'.';
                    grid[next_row_idx][col_idx] = b'O';
                    next_row_idx += 1;
                }
                b'.' => {}
                _ => unreachable!(),
            }
        }
    }

    // west
    for row_idx in 0..num_rows {
        let mut next_col_idx = 0;

        for col_idx in 0..num_cols {
            let rock = grid[row_idx][col_idx];
            match rock {
                b'#' => {
                    next_col_idx = col_idx + 1;
                }
                b'O' => {
                    grid[row_idx][col_idx] = b'.';
                    grid[row_idx][next_col_idx] = b'O';
                    next_col_idx += 1;
                }
                b'.' => {}
                _ => unreachable!(),
            }
        }
    }

    // south
    for col_idx in 0..num_cols {
        let mut next_row_idx = num_rows - 1;

        for row_idx in (0..num_rows).rev() {
            let rock = grid[row_idx][col_idx];
            match rock {
                b'#' => {
                    next_row_idx = row_idx.saturating_sub(1);
                }
                b'O' => {
                    grid[row_idx][col_idx] = b'.';
                    grid[next_row_idx][col_idx] = b'O';
                    next_row_idx = next_row_idx.saturating_sub(1);
                }
                b'.' => {}
                _ => unreachable!(),
            }
        }
    }

    // east
    for row_idx in 0..num_rows {
        let mut next_col_idx = num_cols - 1;

        for col_idx in (0..num_cols).rev() {
            let rock = grid[row_idx][col_idx];
            match rock {
                b'#' => {
                    next_col_idx = col_idx.saturating_sub(1);
                }
                b'O' => {
                    grid[row_idx][col_idx] = b'.';
                    grid[row_idx][next_col_idx] = b'O';
                    next_col_idx = next_col_idx.saturating_sub(1);
                }
                b'.' => {}
                _ => unreachable!(),
            }
        }
    }
}
