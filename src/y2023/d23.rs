use std::{collections::HashSet, rc::Rc};

use ndarray::ArrayView2;

pub fn part1(input: &str) -> u64 {
    let num_rows = input.lines().count();
    let num_cols = input.find('\n').unwrap();
    let grid = ArrayView2::from_shape((num_rows, num_cols + 1), input.as_bytes()).unwrap();
    let start_cell = (0, 1);
    let end_cell = (num_rows - 1, num_cols - 2);
    let mut stack = vec![((1, 1), Dir::Down, 1)];

    let mut max_steps = 0;

    while let Some(((curr_x, curr_y), dir, steps)) = stack.pop() {
        if (curr_x, curr_y) == end_cell {
            max_steps = steps.max(max_steps);
            continue;
        } else if (curr_x, curr_y) == start_cell {
            continue;
        }

        // Up
        if !matches!(dir, Dir::Down) && matches!(grid[(curr_x, curr_y)], b'.' | b'^') {
            let next = ((curr_x - 1, curr_y), Dir::Up, steps + 1);
            if !matches!(grid[next.0], b'#') {
                stack.push(next);
            }
        }

        // Down
        if !matches!(dir, Dir::Up) && matches!(grid[(curr_x, curr_y)], b'.' | b'v') {
            let next = ((curr_x + 1, curr_y), Dir::Down, steps + 1);
            if !matches!(grid[next.0], b'#') {
                stack.push(next);
            }
        }

        // Left
        if !matches!(dir, Dir::Right) && matches!(grid[(curr_x, curr_y)], b'.' | b'<') {
            let next = ((curr_x, curr_y - 1), Dir::Left, steps + 1);
            if !matches!(grid[next.0], b'#') {
                stack.push(next);
            }
        }

        // Right
        if !matches!(dir, Dir::Left) && matches!(grid[(curr_x, curr_y)], b'.' | b'>') {
            let next = ((curr_x, curr_y + 1), Dir::Right, steps + 1);
            if !matches!(grid[next.0], b'#') {
                stack.push(next);
            }
        }
    }
    max_steps
}

pub fn part2(input: &str) -> u16 {
    let num_rows = input.lines().count();
    let num_cols = input.find('\n').unwrap();
    let grid = ArrayView2::from_shape((num_rows, num_cols + 1), input.as_bytes()).unwrap();
    let start_cell = (0, 1);
    let end_cell = (num_rows - 1, num_cols - 2);
    let mut stack = vec![((1, 1), Dir::Down, 1, Rc::new(HashSet::new()))];

    let mut max_steps = 0;

    while let Some(((curr_x, curr_y), dir, steps, forks)) = stack.pop() {
        if (curr_x, curr_y) == end_cell {
            max_steps = steps.max(max_steps);
            continue;
        } else if (curr_x, curr_y) == start_cell {
            continue;
        }

        let new_forks = if [
            grid[(curr_x - 1, curr_y)],
            grid[(curr_x + 1, curr_y)],
            grid[(curr_x, curr_y - 1)],
            grid[(curr_x, curr_y + 1)],
        ]
        .into_iter()
        .filter(|&n| n == b'#')
        .count()
            != 2
        {
            if forks.contains(&(curr_x, curr_y)) {
                continue;
            } else {
                let mut forks = Rc::try_unwrap(forks).unwrap_or_else(|rc| (*rc).clone());
                forks.insert((curr_x, curr_y));
                Rc::new(forks)
            }
        } else {
            forks
        };

        // Up
        if !matches!(dir, Dir::Down) {
            let next = (
                (curr_x - 1, curr_y),
                Dir::Up,
                steps + 1,
                Rc::clone(&new_forks),
            );
            if !matches!(grid[next.0], b'#') {
                stack.push(next);
            }
        }

        // Down
        if !matches!(dir, Dir::Up) {
            let next = (
                (curr_x + 1, curr_y),
                Dir::Down,
                steps + 1,
                Rc::clone(&new_forks),
            );
            if !matches!(grid[next.0], b'#') {
                stack.push(next);
            }
        }

        // Left
        if !matches!(dir, Dir::Right) {
            let next = (
                (curr_x, curr_y - 1),
                Dir::Left,
                steps + 1,
                Rc::clone(&new_forks),
            );
            if !matches!(grid[next.0], b'#') {
                stack.push(next);
            }
        }

        // Right
        if !matches!(dir, Dir::Left) {
            let next = (
                (curr_x, curr_y + 1),
                Dir::Right,
                steps + 1,
                Rc::clone(&new_forks),
            );
            if !matches!(grid[next.0], b'#') {
                stack.push(next);
            }
        }
    }
    max_steps
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
}
