use std::collections::{HashMap, HashSet};

use ndarray::Array3;
use winnow::{ascii::dec_uint, combinator::separated_pair, PResult, Parser};

pub fn part1(input: &str) -> usize {
    let shape = (10, 10, 400);
    let mut space = Array3::<u16>::zeros(shape);

    let mut bricks: Vec<_> = input
        .lines()
        .map(|line| brick.parse(line).unwrap())
        .collect();

    bricks.sort_unstable_by_key(|b| b.0[2].min(b.1[2]));

    for (brick, idx) in bricks.iter_mut().zip(1..) {
        let ([x_0, y_0, z_0], [x_1, y_1, z_1]) = *brick;
        let mut true_z_0 = 1;
        for x in x_0.min(x_1)..=x_0.max(x_1) {
            for y in y_0.min(y_1)..=y_0.max(y_1) {
                for z in (1..z_0.min(z_1)).rev() {
                    if space[(x as usize, y as usize, z as usize)] != 0 {
                        true_z_0 = true_z_0.max(z + 1);
                        break;
                    }
                }
            }
        }

        let true_z_1 = true_z_0 + z_0.abs_diff(z_1);
        *brick = ([x_0, y_0, true_z_0], [x_1, y_1, true_z_1]);
        for x in x_0.min(x_1)..=x_0.max(x_1) {
            for y in y_0.min(y_1)..=y_0.max(y_1) {
                for z in true_z_0..=true_z_1 {
                    space[(x as usize, y as usize, z as usize)] = idx;
                }
            }
        }
    }

    let mut cant_disintegrate = HashSet::new();

    'outer: for &([x_0, y_0, z_0], [x_1, y_1, z_1]) in &bricks {
        let z_below = (z_0.min(z_1) - 1) as usize;
        if z_below == 0 {
            continue;
        }
        let mut brick_below = None;
        for x in x_0.min(x_1)..=x_0.max(x_1) {
            for y in y_0.min(y_1)..=y_0.max(y_1) {
                match space[(x as usize, y as usize, z_below)] {
                    0 => {}
                    idx_below if brick_below.is_none() => brick_below = Some(idx_below),
                    idx_below if brick_below == Some(idx_below) => {}
                    _ => {
                        continue 'outer;
                    }
                }
            }
        }
        cant_disintegrate.insert(brick_below.unwrap());
    }

    bricks.len() - cant_disintegrate.len()
}

pub fn part2(input: &str) -> usize {
    let shape = (10, 10, 400);
    let mut space = Array3::<u16>::zeros(shape);

    let mut bricks: Vec<_> = input
        .lines()
        .map(|line| brick.parse(line).unwrap())
        .collect();

    bricks.sort_unstable_by_key(|b| b.0[2].min(b.1[2]));

    for (brick, idx) in bricks.iter_mut().zip(1..) {
        let ([x_0, y_0, z_0], [x_1, y_1, z_1]) = *brick;
        let mut true_z_0 = 1;
        for x in x_0.min(x_1)..=x_0.max(x_1) {
            for y in y_0.min(y_1)..=y_0.max(y_1) {
                for z in (1..z_0.min(z_1)).rev() {
                    if space[(x as usize, y as usize, z as usize)] != 0 {
                        true_z_0 = true_z_0.max(z + 1);
                        break;
                    }
                }
            }
        }

        let true_z_1 = true_z_0 + z_0.abs_diff(z_1);
        *brick = ([x_0, y_0, true_z_0], [x_1, y_1, true_z_1]);
        for x in x_0.min(x_1)..=x_0.max(x_1) {
            for y in y_0.min(y_1)..=y_0.max(y_1) {
                for z in true_z_0..=true_z_1 {
                    space[(x as usize, y as usize, z as usize)] = idx;
                }
            }
        }
    }

    let mut total_fallen = 0;

    // TODO: change to adj matrix
    let mut dependency_graph: HashMap<u16, (HashSet<u16>, HashSet<u16>)> = HashMap::new();

    for (&([x_0, y_0, z_0], [x_1, y_1, z_1]), curr_idx) in bricks.iter().zip(1..) {
        let z_below = (z_0.min(z_1) - 1) as usize;
        if z_below == 0 {
            continue;
        }

        for x in x_0.min(x_1)..=x_0.max(x_1) {
            for y in y_0.min(y_1)..=y_0.max(y_1) {
                match space[(x as usize, y as usize, z_below)] {
                    0 => {}
                    idx_below => {
                        dependency_graph
                            .entry(curr_idx)
                            .or_default()
                            .0
                            .insert(idx_below);
                        dependency_graph
                            .entry(idx_below)
                            .or_default()
                            .1
                            .insert(curr_idx);
                    }
                }
            }
        }
    }

    let starting_bricks: HashSet<_> = dependency_graph
        .values()
        .filter(|&(c, _)| c.len() == 1)
        .map(|(c, _)| *c.iter().next().unwrap())
        .collect();

    for disintegrated in starting_bricks {
        let mut next_parents = dependency_graph[&disintegrated].1.clone();
        let mut previous_fallen = HashSet::from([disintegrated]);
        while !next_parents.is_empty() {
            let current_parents = next_parents.clone();
            next_parents.clear();
            for brick in current_parents {
                if dependency_graph[&brick].0.is_subset(&previous_fallen) {
                    previous_fallen.insert(brick);
                    next_parents.extend(&dependency_graph[&brick].1);
                }
            }
        }
        total_fallen += previous_fallen.len() - 1;
    }

    total_fallen
}

fn brick(input: &mut &str) -> PResult<([u16; 3], [u16; 3])> {
    separated_pair(cube, '~', cube).parse_next(input)
}

fn cube(input: &mut &str) -> PResult<[u16; 3]> {
    (dec_uint, ',', dec_uint, ',', dec_uint)
        .map(|(a, _, b, _, c)| [a, b, c])
        .parse_next(input)
}
