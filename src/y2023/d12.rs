use std::collections::HashMap;

use bstr::ByteSlice;

pub fn part1(input: &str) -> u64 {
    let mut memoize = HashMap::new();
    let info: Vec<_> = input.lines().map(extract_info).collect();

    info.iter()
        .map(|(given_springs, groups_damaged)| {
            count_possible_arrangements((given_springs, groups_damaged), &mut memoize)
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let info: Vec<_> = input
        .lines()
        .map(extract_info)
        .map(|(given_springs, groups_damaged)| {
            (
                [given_springs; 5].join(&b'?'),
                [groups_damaged.as_slice(); 5].concat(),
            )
        })
        .collect();

    info.iter()
        .map(|(given_springs, groups_damaged)| {
            let mut memoize = HashMap::new();
            count_possible_arrangements((given_springs, groups_damaged), &mut memoize)
        })
        .sum()
}

fn count_possible_arrangements<'a>(
    (given_springs, groups_damaged): (&[u8], &'a [u8]),
    memoize: &mut HashMap<(Vec<u8>, &'a [u8]), u64>,
) -> u64 {
    let given_springs = given_springs.trim_with(|ch| ch == '.');
    if let Some(&out) = memoize.get(&(given_springs.to_vec(), groups_damaged)) {
        return out;
    }
    match given_springs {
        [] => 0,
        [b'?', rest @ ..] => {
            let given_springs = [b"#", rest].concat();
            let out1 = count_possible_arrangements((&given_springs, groups_damaged), memoize);
            memoize.insert((given_springs.clone(), groups_damaged), out1);

            let out2 = count_possible_arrangements((rest, groups_damaged), memoize);
            memoize.insert((rest.to_vec(), groups_damaged), out2);

            out1 + out2
        }
        [_should_be_hash, ..] => {
            let (&first_group, rest_groups) = groups_damaged.split_first().expect(
                "we check if `rest_groups` is empty so `groups_damaged` always has elements",
            );
            if first_group as usize > given_springs.len() {
                return 0;
            }
            let (start_springs, following_springs) = given_springs.split_at(first_group as usize);
            if start_springs.iter().any(|&ch| ch == b'.') {
                return 0;
            }
            if rest_groups.is_empty() {
                u64::from(following_springs.iter().all(|&ch| ch != b'#'))
            } else {
                let [b'.' | b'?', rest_springs @ ..] = following_springs else {
                    return 0;
                };
                let out = count_possible_arrangements((rest_springs, rest_groups), memoize);
                memoize.insert((rest_springs.to_vec(), rest_groups), out);
                out
            }
        }
    }
}

fn extract_info(line: &str) -> (&[u8], Vec<u8>) {
    let (springs, groups_damaged) = line.split_once(' ').unwrap();
    let groups_damaged: Vec<_> = groups_damaged
        .split(',')
        .map(|num| num.parse::<u8>().unwrap())
        .collect();
    (springs.as_bytes(), groups_damaged)
}
