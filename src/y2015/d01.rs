pub fn part1(input: &str) -> i32 {
    input.chars().map(|c| if c == '(' { 1 } else { -1 }).sum()
}

pub fn part2(input: &str) -> usize {
    input
        .chars()
        .map(|c| if c == '(' { 1 } else { -1 })
        .scan(0, |sum, n| {
            *sum += n;
            Some(*sum)
        })
        .position(|sum| sum == -1)
        .map(|pos| pos + 1)
        .unwrap()
}
