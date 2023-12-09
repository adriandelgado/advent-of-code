use atoi::FromRadix10Signed;

fn get_nums<const REVERSE: bool>(line: &str) -> Vec<i32> {
    if REVERSE {
        line.split(' ')
            .rev()
            .map(|n| i32::from_radix_10_signed(n.as_bytes()).0)
            .collect()
    } else {
        line.split(' ')
            .map(|n| i32::from_radix_10_signed(n.as_bytes()).0)
            .collect()
    }
}

pub fn part1(input: &str) -> i32 {
    input.lines().map(get_nums::<false>).map(extrapolate).sum()
}

fn extrapolate(mut last_diff: Vec<i32>) -> i32 {
    let mut sum = last_diff.last().copied().unwrap();

    while last_diff.iter().any(|&d| d != 0) {
        for i in 1..last_diff.len() {
            last_diff[i - 1] = last_diff[i] - last_diff[i - 1];
        }
        last_diff.pop();
        sum += last_diff.last().copied().unwrap();
    }

    sum
}

pub fn part2(input: &str) -> i32 {
    input.lines().map(get_nums::<true>).map(extrapolate).sum()
}
