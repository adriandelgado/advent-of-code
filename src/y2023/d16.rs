use std::collections::HashSet;

use ndarray::ArrayView2;

pub fn part1(input: &str) -> usize {
    let num_rows = input.lines().count();
    let num_cols = input.find('\n').unwrap();
    let grid = ArrayView2::from_shape((num_rows, num_cols + 1), input.as_bytes()).unwrap();

    let mut energized = HashSet::new();
    energized.insert((0, 0));
    let mut visited = HashSet::new();

    let mut stack = vec![((0_usize, 0_usize), Dir::Right)];

    while let Some(((c_row, c_col), c_dir)) = stack.pop() {
        for n_dir in c_dir.next(grid[(c_row, c_col)]) {
            match n_dir {
                Dir::Up => {
                    if c_row > 0 && visited.insert(((c_row - 1, c_col), n_dir)) {
                        energized.insert((c_row - 1, c_col));
                        stack.push(((c_row - 1, c_col), n_dir));
                    }
                }
                Dir::Down => {
                    if c_row + 1 < num_rows && visited.insert(((c_row + 1, c_col), n_dir)) {
                        energized.insert((c_row + 1, c_col));
                        stack.push(((c_row + 1, c_col), n_dir));
                    }
                }
                Dir::Left => {
                    if c_col > 0 && visited.insert(((c_row, c_col - 1), n_dir)) {
                        energized.insert((c_row, c_col - 1));
                        stack.push(((c_row, c_col - 1), n_dir));
                    }
                }
                Dir::Right => {
                    if c_col + 1 < num_cols && visited.insert(((c_row, c_col + 1), n_dir)) {
                        energized.insert((c_row, c_col + 1));
                        stack.push(((c_row, c_col + 1), n_dir));
                    }
                }
            }
        }
    }

    energized.len()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl Dir {
    fn next(self, current: u8) -> Vec<Dir> {
        match (self, current) {
            (Dir::Up, b'\\') => vec![Dir::Left],
            (Dir::Down, b'/') => vec![Dir::Left],
            (Dir::Left, b'-') => vec![Dir::Left],
            (Dir::Left, b'.') => vec![Dir::Left],
            (Dir::Up, b'/') => vec![Dir::Right],
            (Dir::Down, b'\\') => vec![Dir::Right],
            (Dir::Right, b'-') => vec![Dir::Right],
            (Dir::Right, b'.') => vec![Dir::Right],
            (Dir::Up, b'|') => vec![Dir::Up],
            (Dir::Up, b'.') => vec![Dir::Up],
            (Dir::Left, b'\\') => vec![Dir::Up],
            (Dir::Right, b'/') => vec![Dir::Up],
            (Dir::Down, b'|') => vec![Dir::Down],
            (Dir::Down, b'.') => vec![Dir::Down],
            (Dir::Left, b'/') => vec![Dir::Down],
            (Dir::Right, b'\\') => vec![Dir::Down],
            (Dir::Up, b'-') => vec![Dir::Right, Dir::Left],
            (Dir::Down, b'-') => vec![Dir::Right, Dir::Left],
            (Dir::Left, b'|') => vec![Dir::Down, Dir::Up],
            (Dir::Right, b'|') => vec![Dir::Down, Dir::Up],
            _ => unreachable!(),
        }
    }
}

pub fn part2(input: &str) -> usize {
    let num_rows = input.lines().count();
    let num_cols = input.find('\n').unwrap();
    let grid = ArrayView2::from_shape((num_rows, num_cols + 1), input.as_bytes()).unwrap();

    let mut initial_states = Vec::new();
    for col in 0..num_cols {
        initial_states.push(((0, col), Dir::Down));
    }
    for col in 0..num_cols {
        initial_states.push(((num_rows - 1, col), Dir::Up));
    }
    for row in 0..num_rows {
        initial_states.push(((row, 0), Dir::Right));
    }
    for row in 0..num_rows {
        initial_states.push(((row, num_rows - 1), Dir::Left));
    }

    let mut max_energized = 0;

    for init in initial_states {
        let mut energized = HashSet::new();
        energized.insert(init.0);
        let mut visited = HashSet::new();

        let mut stack = vec![init];

        while let Some(((c_row, c_col), c_dir)) = stack.pop() {
            for n_dir in c_dir.next(grid[(c_row, c_col)]) {
                match n_dir {
                    Dir::Up => {
                        if c_row > 0 && visited.insert(((c_row - 1, c_col), n_dir)) {
                            energized.insert((c_row - 1, c_col));
                            stack.push(((c_row - 1, c_col), n_dir));
                        }
                    }
                    Dir::Down => {
                        if c_row + 1 < num_rows && visited.insert(((c_row + 1, c_col), n_dir)) {
                            energized.insert((c_row + 1, c_col));
                            stack.push(((c_row + 1, c_col), n_dir));
                        }
                    }
                    Dir::Left => {
                        if c_col > 0 && visited.insert(((c_row, c_col - 1), n_dir)) {
                            energized.insert((c_row, c_col - 1));
                            stack.push(((c_row, c_col - 1), n_dir));
                        }
                    }
                    Dir::Right => {
                        if c_col + 1 < num_cols && visited.insert(((c_row, c_col + 1), n_dir)) {
                            energized.insert((c_row, c_col + 1));
                            stack.push(((c_row, c_col + 1), n_dir));
                        }
                    }
                }
            }
        }

        max_energized = max_energized.max(energized.len());
    }

    max_energized
}
