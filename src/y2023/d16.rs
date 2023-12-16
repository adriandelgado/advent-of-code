use std::collections::HashSet;

use ndarray::{Array2, ArrayView2};

fn count_energized(init: ((usize, usize), Dir), grid: ArrayView2<u8>) -> u16 {
    let (num_rows, modulo) = grid.dim();
    let num_cols = modulo - 1;

    let mut energized = Array2::<bool>::default((num_rows, num_cols));
    energized[init.0] = true;

    let mut visited = HashSet::new();
    visited.insert(init);

    let mut stack = vec![init];
    while let Some(((c_row, c_col), c_dir)) = stack.pop() {
        for n_dir in c_dir.next(grid[(c_row, c_col)]) {
            match n_dir {
                Dir::Up => {
                    if c_row > 0 && visited.insert(((c_row - 1, c_col), n_dir)) {
                        energized[(c_row - 1, c_col)] = true;
                        stack.push(((c_row - 1, c_col), n_dir));
                    }
                }
                Dir::Down => {
                    if c_row + 1 < num_rows && visited.insert(((c_row + 1, c_col), n_dir)) {
                        energized[(c_row + 1, c_col)] = true;
                        stack.push(((c_row + 1, c_col), n_dir));
                    }
                }
                Dir::Left => {
                    if c_col > 0 && visited.insert(((c_row, c_col - 1), n_dir)) {
                        energized[(c_row, c_col - 1)] = true;
                        stack.push(((c_row, c_col - 1), n_dir));
                    }
                }
                Dir::Right => {
                    if c_col + 1 < num_cols && visited.insert(((c_row, c_col + 1), n_dir)) {
                        energized[(c_row, c_col + 1)] = true;
                        stack.push(((c_row, c_col + 1), n_dir));
                    }
                }
            }
        }
    }

    energized
        .as_slice_memory_order()
        .unwrap()
        .iter()
        .fold(0, |acc, &b| acc + u16::from(b))
}

pub fn part1(input: &str) -> u16 {
    let num_rows = input.lines().count();
    let num_cols = input.find('\n').unwrap();
    let grid = ArrayView2::from_shape((num_rows, num_cols + 1), input.as_bytes()).unwrap();

    count_energized(((0_usize, 0_usize), Dir::Right), grid)
}

pub fn part2(input: &str) -> u16 {
    let num_rows = input.lines().count();
    let num_cols = input.find('\n').unwrap();
    let grid = ArrayView2::from_shape((num_rows, num_cols + 1), input.as_bytes()).unwrap();

    let mut max_energized = 0;

    for col in 0..num_cols {
        let init = ((0, col), Dir::Down);
        max_energized = max_energized.max(count_energized(init, grid));
        let init = ((num_rows - 1, col), Dir::Up);
        max_energized = max_energized.max(count_energized(init, grid));
    }
    for row in 0..num_rows {
        let init = ((row, 0), Dir::Right);
        max_energized = max_energized.max(count_energized(init, grid));
        let init = ((row, num_rows - 1), Dir::Left);
        max_energized = max_energized.max(count_energized(init, grid));
    }

    max_energized
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn next(self, current: u8) -> impl Iterator<Item = Self> {
        let arr = match (self, current) {
            (Dir::Up, b'\\') | (Dir::Down, b'/') => [Some(Dir::Left), None],
            (Dir::Up, b'/') | (Dir::Down, b'\\') => [Some(Dir::Right), None],
            (Dir::Left, b'\\') | (Dir::Right, b'/') => [Some(Dir::Up), None],
            (Dir::Left, b'/') | (Dir::Right, b'\\') => [Some(Dir::Down), None],
            (Dir::Up | Dir::Down, b'-') => [Some(Dir::Right), Some(Dir::Left)],
            (Dir::Left | Dir::Right, b'|') => [Some(Dir::Down), Some(Dir::Up)],
            (dir, _) => [Some(dir), None],
        };

        arr.into_iter().flatten()
    }
}
