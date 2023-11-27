use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|num| num.parse::<u32>().ok())
        .tuple_windows()
        .filter(|(prev, curr)| prev < curr)
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter_map(|num| num.parse::<u32>().ok())
        .tuple_windows()
        .filter(|(a, _, _, d)| a < d)
        .count()
}
