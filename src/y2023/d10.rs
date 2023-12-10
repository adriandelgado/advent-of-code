use std::collections::{btree_map::Entry, BTreeMap, BTreeSet};

use bstr::ByteSlice;
use itertools::Itertools;

pub fn part1(input: &str) -> i32 {
    let grid: Vec<_> = input.lines().map(|row| row.as_bytes().to_vec()).collect();
    let row = grid.iter().position(|row| row.contains(&b'S')).unwrap();
    let col = grid[row].find_byte(b'S').unwrap();
    let row_len = grid[0].len();
    let col_len = grid.len();

    let mut stack = vec![(row, col)];
    let mut distances = BTreeMap::new();
    distances.insert((row, col), 0);

    while let Some(pipe) = stack.pop() {
        let curr_distance = distances[&pipe];
        let (curr_row, curr_col) = pipe;
        let curr_type = grid[curr_row][curr_col];
        for (adj, d_row, d_col) in get_adjacent(&grid, pipe, row_len, col_len) {
            match (adj, curr_type, d_row, d_col) {
                (b'|', b'7' | b'F' | b'|' | b'S', 1, 0)
                | (b'|', b'L' | b'J' | b'|' | b'S', -1, 0)
                | (b'-', b'L' | b'F' | b'-' | b'S', 0, 1)
                | (b'-', b'7' | b'J' | b'-' | b'S', 0, -1)
                | (b'L', b'7' | b'F' | b'|' | b'S', 1, 0)
                | (b'L', b'7' | b'J' | b'-' | b'S', 0, -1)
                | (b'J', b'7' | b'F' | b'|' | b'S', 1, 0)
                | (b'J', b'L' | b'F' | b'-' | b'S', 0, 1)
                | (b'7', b'L' | b'J' | b'|' | b'S', -1, 0)
                | (b'7', b'L' | b'F' | b'-' | b'S', 0, 1)
                | (b'F', b'L' | b'J' | b'|' | b'S', -1, 0)
                | (b'F', b'7' | b'J' | b'-' | b'S', 0, -1) => {
                    let new_row = (curr_row as i64 + d_row) as usize;
                    let new_col = (curr_col as i64 + d_col) as usize;
                    let new_pair = (new_row, new_col);
                    let new_distance = curr_distance + 1;
                    match distances.entry(new_pair) {
                        Entry::Vacant(entry) => {
                            entry.insert(new_distance);
                            stack.push(new_pair);
                        }
                        Entry::Occupied(mut entry) => {
                            if new_distance < *entry.get_mut() {
                                entry.insert(new_distance);
                                stack.push(new_pair);
                            }
                        }
                    };
                }
                _ => {}
            }
        }
    }

    *distances.values().max().unwrap()
}

fn get_adjacent(
    grid: &[Vec<u8>],
    (row, col): (usize, usize),
    row_len: usize,
    col_len: usize,
) -> impl Iterator<Item = (u8, i64, i64)> + '_ {
    let row = row as i64;
    let col = col as i64;
    let row_len = row_len as i64;
    let col_len = col_len as i64;
    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .into_iter()
        .filter(move |(d_row, d_col)| {
            row + d_row >= 0 && row + d_row < row_len && col + d_col >= 0 && col + d_col < col_len
        })
        .map(move |(d_row, d_col)| {
            let row = (row + d_row) as usize;
            let col = (col + d_col) as usize;
            (grid[row][col], d_row, d_col)
        })
}

pub fn part2(input: &str) -> i64 {
    let grid: Vec<_> = input.lines().map(|row| row.as_bytes().to_vec()).collect();
    let row = grid.iter().position(|row| row.contains(&b'S')).unwrap();
    let col = grid[row].find_byte(b'S').unwrap();
    let row_len = grid[0].len();
    let col_len = grid.len();

    let mut stack = vec![(row, col)];
    let mut visited = BTreeSet::new();
    visited.insert((row, col));
    let mut surr_loop = vec![(row as i64, col as i64)];

    while let Some(pipe) = stack.pop() {
        let (curr_row, curr_col) = pipe;
        let curr_type = grid[curr_row][curr_col];
        for (adj, d_row, d_col) in get_adjacent(&grid, pipe, row_len, col_len) {
            match (adj, curr_type, d_row, d_col) {
                (b'|', b'7' | b'F' | b'|' | b'S', 1, 0)
                | (b'|', b'L' | b'J' | b'|', -1, 0)
                | (b'-', b'L' | b'F' | b'-', 0, 1)
                | (b'-', b'7' | b'J' | b'-', 0, -1)
                | (b'L', b'7' | b'F' | b'|', 1, 0)
                | (b'L', b'7' | b'J' | b'-', 0, -1)
                | (b'J', b'7' | b'F' | b'|', 1, 0)
                | (b'J', b'L' | b'F' | b'-', 0, 1)
                | (b'7', b'L' | b'J' | b'|', -1, 0)
                | (b'7', b'L' | b'F' | b'-', 0, 1)
                | (b'F', b'L' | b'J' | b'|', -1, 0)
                | (b'F', b'7' | b'J' | b'-', 0, -1) => {
                    let new_row = (curr_row as i64 + d_row) as usize;
                    let new_col = (curr_col as i64 + d_col) as usize;
                    let new_pair = (new_row, new_col);

                    if visited.insert(new_pair) {
                        stack.push(new_pair);
                        surr_loop.push((new_row as i64, new_col as i64));
                    }
                }
                _ => {}
            }
        }
    }

    let loop_len = surr_loop.len() as i64;
    surr_loop.push((row as i64, col as i64));

    let mut twice_area = 0;

    for ((x_0, y_0), (x_1, y_1)) in surr_loop.into_iter().tuple_windows() {
        twice_area += (y_0 + y_1) * (x_0 - x_1);
    }

    twice_area / 2 - (loop_len / 2 - 1)
}
