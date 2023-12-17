use std::collections::{hash_map::Entry, HashMap, VecDeque};

use ndarray::ArrayView2;

pub fn part1(input: &str) -> u32 {
    let num_rows = input.lines().count();
    let num_cols = input.find('\n').unwrap();
    let grid = ArrayView2::from_shape((num_rows, num_cols + 1), input.as_bytes()).unwrap();
    let end_coords = (num_rows - 1, num_cols - 1);

    let mut visited = HashMap::from([
        (
            ((0, 1), Dir::Right),
            [ch_to_u32(grid[(0, 1)]), u32::MAX, u32::MAX],
        ),
        (
            ((1, 0), Dir::Down),
            [ch_to_u32(grid[(1, 0)]), u32::MAX, u32::MAX],
        ),
    ]);

    let mut queue = VecDeque::from([
        ((0, 1), (Dir::Right, 1, ch_to_u32(grid[(0, 1)]))),
        ((1, 0), (Dir::Down, 1, ch_to_u32(grid[(1, 0)]))),
    ]);

    while let Some(((c_row, c_col), (c_dir, c_count, current_heat_loss))) = queue.pop_front() {
        if (c_row, c_col) == end_coords {
            continue;
        }

        let next_dir = Dir::Up;
        let next_count = if c_dir == next_dir { c_count + 1 } else { 1 };
        if c_row > 0 && c_dir != Dir::Down && next_count <= 3 {
            let next = (c_row - 1, c_col);
            let next_heat_loss = ch_to_u32(grid[next]) + current_heat_loss;
            match visited.entry((next, next_dir)) {
                Entry::Occupied(mut e) => {
                    let mut v_heat_loss_arr = *e.get();
                    if v_heat_loss_arr[next_count - 1] > next_heat_loss {
                        v_heat_loss_arr[next_count - 1] = next_heat_loss;
                        e.insert(v_heat_loss_arr);
                        queue.push_back((next, (next_dir, next_count, next_heat_loss)));
                    }
                }
                Entry::Vacant(e) => {
                    let mut v_heat_loss_arr = [u32::MAX; 3];
                    v_heat_loss_arr[next_count - 1] = next_heat_loss;
                    e.insert(v_heat_loss_arr);
                    queue.push_back((next, (next_dir, next_count, next_heat_loss)));
                }
            }
        }

        let next_dir = Dir::Down;
        let next_count = if c_dir == next_dir { c_count + 1 } else { 1 };
        if c_row + 1 < num_rows && c_dir != Dir::Up && next_count <= 3 {
            let next = (c_row + 1, c_col);
            let next_heat_loss = ch_to_u32(grid[next]) + current_heat_loss;
            match visited.entry((next, next_dir)) {
                Entry::Occupied(mut e) => {
                    let mut v_heat_loss_arr = *e.get();
                    if v_heat_loss_arr[next_count - 1] > next_heat_loss {
                        v_heat_loss_arr[next_count - 1] = next_heat_loss;
                        e.insert(v_heat_loss_arr);
                        queue.push_back((next, (next_dir, next_count, next_heat_loss)));
                    }
                }
                Entry::Vacant(e) => {
                    let mut v_heat_loss_arr = [u32::MAX; 3];
                    v_heat_loss_arr[next_count - 1] = next_heat_loss;
                    e.insert(v_heat_loss_arr);
                    queue.push_back((next, (next_dir, next_count, next_heat_loss)));
                }
            }
        }

        let next_dir = Dir::Left;
        let next_count = if c_dir == next_dir { c_count + 1 } else { 1 };
        if c_col > 0 && c_dir != Dir::Right && next_count <= 3 {
            let next = (c_row, c_col - 1);
            let next_heat_loss = ch_to_u32(grid[next]) + current_heat_loss;
            match visited.entry((next, next_dir)) {
                Entry::Occupied(mut e) => {
                    let mut v_heat_loss_arr = *e.get();
                    if v_heat_loss_arr[next_count - 1] > next_heat_loss {
                        v_heat_loss_arr[next_count - 1] = next_heat_loss;
                        e.insert(v_heat_loss_arr);
                        queue.push_back((next, (next_dir, next_count, next_heat_loss)));
                    }
                }
                Entry::Vacant(e) => {
                    let mut v_heat_loss_arr = [u32::MAX; 3];
                    v_heat_loss_arr[next_count - 1] = next_heat_loss;
                    e.insert(v_heat_loss_arr);
                    queue.push_back((next, (next_dir, next_count, next_heat_loss)));
                }
            }
        }

        let next_dir = Dir::Right;
        let next_count = if c_dir == next_dir { c_count + 1 } else { 1 };
        if c_col + 1 < num_cols && c_dir != Dir::Left && next_count <= 3 {
            let next = (c_row, c_col + 1);
            let next_heat_loss = ch_to_u32(grid[next]) + current_heat_loss;
            match visited.entry((next, next_dir)) {
                Entry::Occupied(mut e) => {
                    let mut v_heat_loss_arr = *e.get();
                    if v_heat_loss_arr[next_count - 1] > next_heat_loss {
                        v_heat_loss_arr[next_count - 1] = next_heat_loss;
                        e.insert(v_heat_loss_arr);
                        queue.push_back((next, (next_dir, next_count, next_heat_loss)));
                    }
                }
                Entry::Vacant(e) => {
                    let mut v_heat_loss_arr = [u32::MAX; 3];
                    v_heat_loss_arr[next_count - 1] = next_heat_loss;
                    e.insert(v_heat_loss_arr);
                    queue.push_back((next, (next_dir, next_count, next_heat_loss)));
                }
            }
        }
    }
    let mut min_visited = HashMap::new();
    for (k, v) in visited.into_iter().map(|(k, v)| (k.0, v)) {
        let min_val = v.into_iter().min().unwrap();
        min_visited
            .entry(k)
            .and_modify(|hl| *hl = min_val.min(*hl))
            .or_insert(min_val);
    }

    min_visited[&end_coords]
}

fn ch_to_u32(ch: u8) -> u32 {
    u32::from(ch - b'0')
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub fn part2(input: &str) -> u32 {
    let num_rows = input.lines().count();
    let num_cols = input.find('\n').unwrap();
    let grid = ArrayView2::from_shape((num_rows, num_cols + 1), input.as_bytes()).unwrap();
    let end_coords = (num_rows - 1, num_cols - 1);

    let mut visited = HashMap::from([
        (
            ((0, 1), Dir::Right),
            std::array::from_fn(|i| {
                if i == 1 {
                    ch_to_u32(grid[(0, 1)])
                } else {
                    u32::MAX
                }
            }),
        ),
        (
            ((1, 0), Dir::Down),
            std::array::from_fn(|i| {
                if i == 1 {
                    ch_to_u32(grid[(1, 0)])
                } else {
                    u32::MAX
                }
            }),
        ),
    ]);

    let mut queue = VecDeque::from([
        ((0, 1), (Dir::Right, 1, ch_to_u32(grid[(0, 1)]))),
        ((1, 0), (Dir::Down, 1, ch_to_u32(grid[(1, 0)]))),
    ]);

    while let Some(((c_row, c_col), (c_dir, c_count, current_heat_loss))) = queue.pop_front() {
        if (c_row, c_col) == end_coords {
            continue;
        }

        let next_dir = Dir::Up;
        let next_count = if c_dir == next_dir { c_count + 1 } else { 1 };
        if c_row > 0
            && c_dir != Dir::Down
            && next_count <= 10
            && (c_dir == next_dir || c_count >= 4)
        {
            let next = (c_row - 1, c_col);
            let next_heat_loss = ch_to_u32(grid[next]) + current_heat_loss;
            match visited.entry((next, next_dir)) {
                Entry::Occupied(mut e) => {
                    let mut v_heat_loss_arr = *e.get();
                    if v_heat_loss_arr[next_count - 1] > next_heat_loss {
                        v_heat_loss_arr[next_count - 1] = next_heat_loss;
                        e.insert(v_heat_loss_arr);
                        queue.push_back((next, (next_dir, next_count, next_heat_loss)));
                    }
                }
                Entry::Vacant(e) => {
                    let mut v_heat_loss_arr = [u32::MAX; 10];
                    v_heat_loss_arr[next_count - 1] = next_heat_loss;
                    e.insert(v_heat_loss_arr);
                    queue.push_back((next, (next_dir, next_count, next_heat_loss)));
                }
            }
        }

        let next_dir = Dir::Down;
        let next_count = if c_dir == next_dir { c_count + 1 } else { 1 };
        if c_row + 1 < num_rows
            && c_dir != Dir::Up
            && next_count <= 10
            && (c_dir == next_dir || c_count >= 4)
        {
            let next = (c_row + 1, c_col);
            let next_heat_loss = ch_to_u32(grid[next]) + current_heat_loss;
            match visited.entry((next, next_dir)) {
                Entry::Occupied(mut e) => {
                    let mut v_heat_loss_arr = *e.get();
                    if v_heat_loss_arr[next_count - 1] > next_heat_loss {
                        v_heat_loss_arr[next_count - 1] = next_heat_loss;
                        e.insert(v_heat_loss_arr);
                        queue.push_back((next, (next_dir, next_count, next_heat_loss)));
                    }
                }
                Entry::Vacant(e) => {
                    let mut v_heat_loss_arr = [u32::MAX; 10];
                    v_heat_loss_arr[next_count - 1] = next_heat_loss;
                    e.insert(v_heat_loss_arr);
                    queue.push_back((next, (next_dir, next_count, next_heat_loss)));
                }
            }
        }

        let next_dir = Dir::Left;
        let next_count = if c_dir == next_dir { c_count + 1 } else { 1 };
        if c_col > 0
            && c_dir != Dir::Right
            && next_count <= 10
            && (c_dir == next_dir || c_count >= 4)
        {
            let next = (c_row, c_col - 1);
            let next_heat_loss = ch_to_u32(grid[next]) + current_heat_loss;
            match visited.entry((next, next_dir)) {
                Entry::Occupied(mut e) => {
                    let mut v_heat_loss_arr = *e.get();
                    if v_heat_loss_arr[next_count - 1] > next_heat_loss {
                        v_heat_loss_arr[next_count - 1] = next_heat_loss;
                        e.insert(v_heat_loss_arr);
                        queue.push_back((next, (next_dir, next_count, next_heat_loss)));
                    }
                }
                Entry::Vacant(e) => {
                    let mut v_heat_loss_arr = [u32::MAX; 10];
                    v_heat_loss_arr[next_count - 1] = next_heat_loss;
                    e.insert(v_heat_loss_arr);
                    queue.push_back((next, (next_dir, next_count, next_heat_loss)));
                }
            }
        }

        let next_dir = Dir::Right;
        let next_count = if c_dir == next_dir { c_count + 1 } else { 1 };
        if c_col + 1 < num_cols
            && c_dir != Dir::Left
            && next_count <= 10
            && (c_dir == next_dir || c_count >= 4)
        {
            let next = (c_row, c_col + 1);
            let next_heat_loss = ch_to_u32(grid[next]) + current_heat_loss;
            match visited.entry((next, next_dir)) {
                Entry::Occupied(mut e) => {
                    let mut v_heat_loss_arr = *e.get();
                    if v_heat_loss_arr[next_count - 1] > next_heat_loss {
                        v_heat_loss_arr[next_count - 1] = next_heat_loss;
                        e.insert(v_heat_loss_arr);
                        queue.push_back((next, (next_dir, next_count, next_heat_loss)));
                    }
                }
                Entry::Vacant(e) => {
                    let mut v_heat_loss_arr = [u32::MAX; 10];
                    v_heat_loss_arr[next_count - 1] = next_heat_loss;
                    e.insert(v_heat_loss_arr);
                    queue.push_back((next, (next_dir, next_count, next_heat_loss)));
                }
            }
        }
    }

    let mut min_visited = HashMap::new();
    for (k, v) in visited.into_iter().map(|(k, v)| (k.0, v)) {
        let min_val = v.into_iter().skip(3).min().unwrap();
        min_visited
            .entry(k)
            .and_modify(|hl| *hl = min_val.min(*hl))
            .or_insert(min_val);
    }

    min_visited[&end_coords]
}
