use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| min_3_vowels(line) && twice_in_a_row(line) && not_forbidden(line))
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| twice_non_overlapping(line) && twice_skipping(line))
        .count()
}

fn min_3_vowels(line: &str) -> bool {
    line.chars()
        .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
        .count()
        >= 3
}

fn twice_in_a_row(line: &str) -> bool {
    line.as_bytes().windows(2).any(|arr| arr[0] == arr[1])
}

fn not_forbidden(line: &str) -> bool {
    line.as_bytes()
        .windows(2)
        .all(|arr| !matches!(arr, b"ab" | b"cd" | b"pq" | b"xy"))
}

fn twice_non_overlapping(line: &str) -> bool {
    let mut prev = HashMap::new();
    for (new_idx, pair) in line.as_bytes().windows(2).enumerate() {
        let Some(old_idx) = prev.insert(pair, new_idx) else {
            continue;
        };
        if old_idx + 1 != new_idx {
            return true;
        }
        prev.insert(pair, old_idx);
    }

    false
}

fn twice_skipping(line: &str) -> bool {
    line.as_bytes().windows(3).any(|arr| arr[0] == arr[2])
}
