use std::collections::HashMap;

use itertools::Itertools;

#[inline]
pub fn a(file: &str) -> String {
    file.lines()
        .map(|line| line.split_once(" | ").unwrap().1)
        .flat_map(|line| line.split_ascii_whitespace())
        .map(|pat| pat.len())
        .filter(|len| [2, 3, 4, 7].contains(len))
        .count()
        .to_string()
}

#[inline]
pub fn b(file: &str) -> String {
    let mut sum: usize = 0;
    for (clue, output) in file.lines().map(|line| line.split_once(" | ").unwrap()) {
        for guess in (0_u8..7).permutations(7) {
            let map = generate_numbers(&guess[..].try_into().unwrap());
            if clue.split_ascii_whitespace().all(|letters| {
                let mut numbers: Vec<_> = letters.bytes().map(|byte| byte - 97).collect();
                numbers.sort_unstable();
                map.contains_key(&numbers)
            }) {
                let mut number = 0;
                for (idx, digit) in output
                    .split_ascii_whitespace()
                    .map(|letters| {
                        let mut numbers: Vec<_> = letters.bytes().map(|byte| byte - 97).collect();
                        numbers.sort_unstable();
                        map[&numbers]
                    })
                    .rev()
                    .enumerate()
                {
                    number += digit * 10_usize.pow(idx as u32);
                }
                sum += number;
                break;
            }
        }
    }
    sum.to_string()
}

fn generate_numbers(guess: &[u8; 7]) -> HashMap<Vec<u8>, usize> {
    let mut map = HashMap::with_capacity(7);

    for (idx, number) in [
        vec![0, 1, 2, 4, 5, 6],
        vec![2, 5],
        vec![0, 2, 3, 4, 6],
        vec![0, 2, 3, 5, 6],
        vec![1, 2, 3, 5],
        vec![0, 1, 3, 5, 6],
        vec![0, 1, 3, 4, 5, 6],
        vec![0, 2, 5],
        vec![0, 1, 2, 3, 4, 5, 6],
        vec![0, 1, 2, 3, 5, 6],
    ]
    .into_iter()
    .enumerate()
    {
        let mut new_number: Vec<_> = number.into_iter().map(|n| guess[n]).collect();
        new_number.sort_unstable();
        map.insert(new_number, idx);
    }

    map
}

#[test]
fn day8_is_correct() {
    use super::FILES;

    assert_eq!(a(FILES[7]), "534");
    assert_eq!(b(FILES[7]), "1070188");
}
