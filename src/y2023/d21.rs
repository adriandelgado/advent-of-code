use std::collections::{HashSet, VecDeque};

use ndarray::ArrayView2;

pub fn part1(input: &str) -> usize {
    let num_rows = input.lines().count();
    let num_cols = input.find('\n').unwrap();
    let grid = ArrayView2::from_shape((num_rows, num_cols + 1), input.as_bytes()).unwrap();

    let raw_start = input.find('S').unwrap();
    let (row_start, col_start) = (raw_start / (num_cols + 1), raw_start % (num_cols + 1));

    let mut steps_64 = 0;

    let mut queue = VecDeque::from([(row_start, col_start, 0)]);
    let mut visited = HashSet::new();

    while let Some((row_c, col_c, steps_c)) = queue.pop_front() {
        if steps_c == 64 {
            steps_64 += 1;
            continue;
        }
        // Up
        if row_c > 0 {
            let next = (row_c - 1, col_c, steps_c + 1);
            if matches!(grid[(next.0, next.1)], b'.' | b'S') && visited.insert(next) {
                queue.push_back(next);
            }
        }
        // Down
        if row_c + 1 < num_rows {
            let next = (row_c + 1, col_c, steps_c + 1);
            if matches!(grid[(next.0, next.1)], b'.' | b'S') && visited.insert(next) {
                queue.push_back(next);
            }
        }
        // Left
        if col_c > 0 {
            let next = (row_c, col_c - 1, steps_c + 1);
            if matches!(grid[(next.0, next.1)], b'.' | b'S') && visited.insert(next) {
                queue.push_back(next);
            }
        }
        // Right
        if col_c + 1 < num_cols {
            let next = (row_c, col_c + 1, steps_c + 1);
            if matches!(grid[(next.0, next.1)], b'.' | b'S') && visited.insert(next) {
                queue.push_back(next);
            }
        }
    }

    steps_64
}

pub fn part2(input: &str) -> usize {
    let num_rows = input.lines().count();
    let num_cols = input.find('\n').unwrap();
    let grid = ArrayView2::from_shape((num_rows, num_cols + 1), input.as_bytes()).unwrap();

    let raw_start = input.find('S').unwrap();
    let (row_start, col_start) = (raw_start / (num_cols + 1), raw_start % (num_cols + 1));

    let mut steps_64 = HashSet::new();

    let mut queue = VecDeque::from([(row_start, col_start, 0, 0, 0)]);
    let mut visited = HashSet::new();

    while let Some((row_c, col_c, steps_c, x_world, y_world)) = queue.pop_front() {
        // 14775, 58711, 131809, 234069 -> 1 + 193(x/131) + 14581 (x/131)^2
        // 26501365 % 131 == 65
        // 26501365 // 131 == 202300
        // 596731692533901 + 2931735309
        // 596734624269210
        // WOLFRAM ALPHA MAGIC
        if steps_c == num_rows * 3 + 65 {
            steps_64.insert((row_c, col_c, x_world, y_world));
            continue;
        }

        // Up
        let (next_row, next_y_world) = if row_c > 0 {
            (row_c - 1, y_world)
        } else {
            (num_rows - 1, y_world + 1)
        };
        let next = (next_row, col_c, steps_c + 1, x_world, next_y_world);
        if matches!(grid[(next.0, next.1)], b'.' | b'S') && visited.insert(next) {
            queue.push_back(next);
        }

        // Down
        let (next_row, next_y_world) = if row_c + 1 < num_rows {
            (row_c + 1, y_world)
        } else {
            (0, y_world - 1)
        };
        let next = (next_row, col_c, steps_c + 1, x_world, next_y_world);
        if matches!(grid[(next.0, next.1)], b'.' | b'S') && visited.insert(next) {
            queue.push_back(next);
        }

        // Left
        let (next_col, next_x_world) = if col_c > 0 {
            (col_c - 1, x_world)
        } else {
            (num_cols - 1, x_world - 1)
        };
        let next = (row_c, next_col, steps_c + 1, next_x_world, y_world);
        if matches!(grid[(next.0, next.1)], b'.' | b'S') && visited.insert(next) {
            queue.push_back(next);
        }

        // Right
        let (next_col, next_x_world) = if col_c + 1 < num_cols {
            (col_c + 1, x_world)
        } else {
            (0, x_world + 1)
        };
        let next = (row_c, next_col, steps_c + 1, next_x_world, y_world);
        if matches!(grid[(next.0, next.1)], b'.' | b'S') && visited.insert(next) {
            queue.push_back(next);
        }
    }

    steps_64.len()
}
