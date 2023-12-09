pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            extrapolate(
                line.split(' ')
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .sum()
}

fn extrapolate(nums: Vec<i64>) -> i64 {
    let mut differences: Vec<Vec<i64>> = vec![nums];

    while !differences.last().unwrap().iter().all(|&d| d == 0) {
        let deltas: Vec<i64> = differences
            .last()
            .unwrap()
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect();

        differences.push(deltas);
    }

    differences.into_iter().map(|d| *d.last().unwrap()).sum()
}

pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            extrapolate_2(
                line.split(' ')
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .sum()
}

fn extrapolate_2(nums: Vec<i64>) -> i64 {
    let mut differences: Vec<Vec<i64>> = vec![nums];

    while !differences.last().unwrap().iter().all(|&d| d == 0) {
        let deltas: Vec<i64> = differences
            .last()
            .unwrap()
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect();

        differences.push(deltas);
    }

    differences
        .into_iter()
        .zip([1, -1].into_iter().cycle())
        .map(|(d, sign)| *d.first().unwrap() * sign)
        .sum()
}
