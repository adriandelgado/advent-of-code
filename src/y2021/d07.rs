pub fn part1(input: &str) -> u64 {
    let mut numbers: Vec<i64> = input
        .trim_end()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();
    numbers.sort_unstable();

    let median = numbers[numbers.len() / 2];

    numbers.into_iter().map(|n| median.abs_diff(n)).sum()
}

pub fn part2(input: &str) -> u64 {
    let numbers: Vec<i64> = input
        .trim_end()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    let avg = numbers.iter().sum::<i64>() / i64::try_from(numbers.len()).unwrap();

    numbers
        .into_iter()
        .map(|num| {
            let n = avg.abs_diff(num);
            n * (n + 1) / 2
        })
        .sum()
}
