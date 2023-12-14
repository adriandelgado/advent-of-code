use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    let grid: Vec<_> = input.lines().map(str::as_bytes).collect();
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
                _ => {}
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
        if let Some(last_i) = seen.insert(grid.clone(), i) {
            // grid nuevo
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

#[allow(clippy::naive_bytecount)]
fn get_north_load(grid: &[Vec<u8>]) -> usize {
    grid.iter()
        .zip((1..=grid.len()).rev())
        .map(|(row, load)| row.iter().filter(|&&ch| ch == b'O').count() * load)
        .sum()
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
                _ => {}
            }
        }
    }

    // west
    for row in grid.iter_mut() {
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
                _ => {}
            }
        }
    }

    // east
    for row in grid.iter_mut() {
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
