pub(super) fn part1(input: &str) -> String {
    input
        .chars()
        .map(|c| if c == '(' { 1 } else { -1 })
        .sum::<i32>()
        .to_string()
}

pub(super) fn part2(input: &str) -> String {
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
        .to_string()
}
