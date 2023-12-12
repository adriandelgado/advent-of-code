use std::collections::HashMap;

use bstr::ByteSlice;
use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .enumerate()
        .map(|(n, line)| {
            println!("{n}");
            line
        })
        .map(extract_info)
        .map(count_possible_arrangements)
        .sum()
}

fn count_possible_arrangements((given_springs, groups_damaged): (&[u8], Vec<usize>)) -> usize {
    let num_damaged: usize = groups_damaged.iter().sum();
    let clumps_damaged = groups_damaged.len();
    let total_springs = given_springs.len();
    let fixed_functional = groups_damaged.len() - 1;
    let flexible_functional = total_springs - num_damaged - fixed_functional;
    let all_idxs: Vec<_> = (0..(flexible_functional + clumps_damaged)).collect();

    let mut possible = 0;
    for damaged_idxs in all_idxs.into_iter().combinations(clumps_damaged) {
        let mut possible_springs = (0..(flexible_functional + clumps_damaged))
            .map(|idx| {
                if damaged_idxs.contains(&idx) {
                    b'#'
                } else {
                    b'.'
                }
            })
            .collect_vec();
        for (idx, &amount) in std::iter::zip(damaged_idxs, &groups_damaged).rev() {
            for _ in 1..amount {
                possible_springs.insert(idx, b'#');
            }
            possible_springs.insert(idx, b'.');
        }
        possible_springs.remove(0);
        if std::iter::zip(&possible_springs, given_springs)
            .all(|(p, &g)| matches!((p, g), (b'.', b'.' | b'?') | (b'#', b'#' | b'?')))
        {
            possible += 1;
        }
    }

    possible
}

pub fn part2(input: &str) -> usize {
    let mut memoize = HashMap::new();
    input
        .lines()
        .enumerate()
        .map(|(n, line)| {
            println!("{n}");
            line
        })
        .map(extract_info)
        .map(|(given_springs, groups_damaged)| {
            let new_springs = [given_springs; 5].join(&b'?');
            let new_groups_damaged = [&groups_damaged; 5]
                .into_iter()
                .flatten()
                .copied()
                .collect_vec();

            count_possible_arrangements_2((&new_springs, &new_groups_damaged), &mut memoize)
        })
        .sum()
}

fn count_possible_arrangements_2(
    (given_springs, groups_damaged): (&[u8], &[usize]),
    memoize: &mut HashMap<(Vec<u8>, Vec<usize>), usize>,
) -> usize {
    if let Some(&out) = memoize.get(&(given_springs.to_vec(), groups_damaged.to_vec())) {
        return out;
    }
    match given_springs {
        [] => usize::from(groups_damaged.len() == 0),
        [b'?', rest @ ..] => {
            let given_springs = [b"#", rest].concat();
            let out1 = count_possible_arrangements_2((&given_springs, groups_damaged), memoize);
            memoize.insert((given_springs.to_vec(), groups_damaged.to_vec()), out1);

            let given_springs = [b".", rest].concat();
            let out2 = count_possible_arrangements_2((&given_springs, groups_damaged), memoize);
            memoize.insert((given_springs.to_vec(), groups_damaged.to_vec()), out2);

            out1 + out2
        }
        [b'.', ..] => {
            let given_springs = given_springs.trim_with(|ch| ch == '.');
            let out = count_possible_arrangements_2((given_springs, groups_damaged), memoize);
            memoize.insert((given_springs.to_vec(), groups_damaged.to_vec()), out);
            out
        }
        [b'#', ..] => {
            if groups_damaged.len() == 0 {
                return 0;
            }
            if given_springs.len() < groups_damaged[0] {
                return 0;
            }
            if given_springs[0..groups_damaged[0]]
                .into_iter()
                .any(|&ch| ch == b'.')
            {
                return 0;
            }
            if groups_damaged.len() == 1 {
                let given_springs = &given_springs[groups_damaged[0]..];
                let groups_damaged = &groups_damaged[1..];
                let out = count_possible_arrangements_2((given_springs, groups_damaged), memoize);
                memoize.insert((given_springs.to_vec(), groups_damaged.to_vec()), out);
                out
            } else {
                if given_springs.len() == groups_damaged[0]
                    || given_springs[groups_damaged[0]] == b'#'
                {
                    return 0;
                }
                let given_springs = &given_springs[groups_damaged[0] + 1..];
                let groups_damaged = &groups_damaged[1..];
                let out = count_possible_arrangements_2((given_springs, groups_damaged), memoize);
                memoize.insert((given_springs.to_vec(), groups_damaged.to_vec()), out);
                out
            }
        }
        [other, ..] => {
            unreachable!("{}", *other as char);
        }
    }
}

fn extract_info(line: &str) -> (&[u8], Vec<usize>) {
    let (springs, groups_damaged) = line.split_once(' ').unwrap();
    let groups_damaged: Vec<_> = groups_damaged
        .split(',')
        .map(|num| num.parse::<usize>().unwrap())
        .collect();
    (springs.as_bytes(), groups_damaged)
}
