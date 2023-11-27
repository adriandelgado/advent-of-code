pub fn part1(input: &str) -> u64 {
    simulate::<80>(input)
}

pub fn part2(input: &str) -> u64 {
    simulate::<256>(input)
}

fn simulate<const DAYS: usize>(input: &str) -> u64 {
    let mut amounts = [0; 9];
    for fish in input
        .trim_end()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
    {
        amounts[fish] += 1;
    }
    for _ in 0..DAYS {
        amounts.rotate_left(1);
        amounts[6] += amounts[8];
    }
    amounts.into_iter().sum()
}
