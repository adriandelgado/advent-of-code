const LINE_LEN: usize = 12;

pub fn part1(input: &str) -> i32 {
    let mut sum = [0; LINE_LEN];

    for line in input.lines() {
        assert_eq!(line.len(), LINE_LEN);

        for (idx, chr) in line.bytes().enumerate() {
            sum[idx] += i32::from(chr - b'0') * 2 - 1;
        }
    }

    let gamma = sum
        .iter()
        .rev()
        .map(|&d| i32::from(d > 0))
        .zip(0..)
        .fold(0, |acc, (digit, idx)| acc + (digit << idx));

    let epsilon = sum
        .iter()
        .rev()
        .map(|&d| i32::from(d < 0))
        .zip(0..)
        .fold(0, |acc, (digit, idx)| acc + (digit << idx));

    gamma * epsilon
}

pub fn part2(input: &str) -> i32 {
    let mut oxygen: Vec<_> = input
        .lines()
        .map(|line| -> [u8; LINE_LEN] { line.as_bytes().try_into().unwrap() })
        .map(|line| line.map(|ch| ch - b'0'))
        .collect();
    let mut co2 = oxygen.clone();

    for idx in 0..LINE_LEN {
        let mut sum_oxygen = 0;
        let mut sum_co2 = 0;

        for line in &oxygen {
            sum_oxygen += i32::from(line[idx]) * 2 - 1;
        }
        for line in &co2 {
            sum_co2 += i32::from(line[idx]) * 2 - 1;
        }

        if oxygen.len() > 1 {
            oxygen.retain(|line| line[idx] == u8::from(sum_oxygen >= 0));
        }
        if co2.len() > 1 {
            co2.retain(|line| line[idx] == u8::from(sum_co2 < 0));
        }
    }

    let oxygen_rating = oxygen[0]
        .iter()
        .map(|&ch| i32::from(ch))
        .rev()
        .zip(0..)
        .fold(0, |acc, (digit, idx)| acc + (digit << idx));

    let co2_rating = co2[0]
        .iter()
        .map(|&ch| i32::from(ch))
        .rev()
        .zip(0..)
        .fold(0, |acc, (digit, idx)| acc + (digit << idx));

    oxygen_rating * co2_rating
}
