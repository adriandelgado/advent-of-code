use std::collections::HashMap;

use bstr::ByteSlice;

pub fn part1(input: &str) -> usize {
    let mut memoize = HashMap::new();
    input
        .lines()
        .map(extract_info)
        .map(|(given_springs, groups_damaged)| {
            count_possible_arrangements((given_springs, &groups_damaged), &mut memoize)
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut memoize = HashMap::new();
    input
        .lines()
        .map(extract_info)
        .map(|(given_springs, groups_damaged)| {
            let new_springs = [given_springs; 5].join(&b'?');
            let new_groups_damaged = [groups_damaged.as_slice(); 5].concat();

            count_possible_arrangements((&new_springs, &new_groups_damaged), &mut memoize)
        })
        .sum()
}

fn count_possible_arrangements(
    (given_springs, groups_damaged): (&[u8], &[usize]),
    memoize: &mut HashMap<(Vec<u8>, Vec<usize>), usize>,
) -> usize {
    let given_springs = given_springs.trim_with(|ch| ch == '.');
    if let Some(&out) = memoize.get(&(given_springs.to_vec(), groups_damaged.to_vec())) {
        return out;
    }
    match given_springs {
        [] => 0,
        [b'?', rest @ ..] => {
            let given_springs = [b"#", rest].concat();
            let out1 = count_possible_arrangements((&given_springs, groups_damaged), memoize);
            memoize.insert((given_springs.clone(), groups_damaged.to_vec()), out1);

            let out2 = count_possible_arrangements((rest, groups_damaged), memoize);
            memoize.insert((rest.to_vec(), groups_damaged.to_vec()), out2);

            out1 + out2
        }
        [b'#', ..] => {
            let &[first_group, ref rest_groups @ ..] = groups_damaged else {
                unreachable!(
                    "we check if `rest_groups` is empty so `groups_damaged` always has elements"
                )
            };
            if first_group > given_springs.len() {
                return 0;
            }
            let (start_springs, following_springs) = given_springs.split_at(first_group);
            if start_springs.iter().any(|&ch| ch == b'.') {
                return 0;
            }
            if rest_groups.is_empty() {
                usize::from(following_springs.iter().all(|&ch| ch != b'#'))
            } else {
                let [b'.' | b'?', rest_springs @ ..] = following_springs else {
                    return 0;
                };
                let out = count_possible_arrangements((rest_springs, rest_groups), memoize);
                memoize.insert((rest_springs.to_vec(), rest_groups.to_vec()), out);
                out
            }
        }
        [_other, ..] => unreachable!(),
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
