const LINE_LEN: usize = 12;

pub fn a(file: &str) -> String {
    let mut sum = [0; LINE_LEN];

    for line in file.lines() {
        assert_eq!(line.len(), LINE_LEN);

        for (idx, chr) in line.bytes().enumerate() {
            match chr {
                b'0' => sum[idx] -= 1,
                b'1' => sum[idx] += 1,
                _ => unreachable!(),
            }
        }
    }

    let gamma = sum
        .iter()
        .rev()
        .map(|&d| i64::from(d > 0))
        .enumerate()
        .fold(0, |acc, (idx, digit)| {
            let idx: u8 = idx.try_into().unwrap();
            // acc + 2_i64.pow(idx) * digit
            acc + (digit << idx)
        });
    let epsilon = sum
        .iter()
        .rev()
        .map(|&d| i64::from(d < 0))
        .enumerate()
        .fold(0, |acc, (idx, digit)| {
            let idx: u8 = idx.try_into().unwrap();
            // acc + 2_i64.pow(idx) * digit
            acc + (digit << idx)
        });

    (gamma * epsilon).to_string()
}

pub fn b(file: &str) -> String {
    let mut oxigen: Vec<_> = file.lines().map(|line| line.as_bytes()).collect();
    let mut co2 = oxigen.clone();
    for idx in 0..LINE_LEN {
        let mut sum_oxigen = 0;
        let mut sum_co2 = 0;

        for line in oxigen.iter() {
            sum_oxigen += match line[idx] {
                b'0' => -1,
                b'1' => 1,
                _ => unreachable!(),
            }
        }
        for line in co2.iter() {
            sum_co2 += match line[idx] {
                b'0' => -1,
                b'1' => 1,
                _ => unreachable!(),
            }
        }

        let digit_oxigen = if sum_oxigen < 0 { b'0' } else { b'1' };
        let digit_co2 = if sum_co2 < 0 { b'1' } else { b'0' };

        if oxigen.len() > 1 {
            oxigen.retain(|line| line[idx] == digit_oxigen);
        }
        if co2.len() > 1 {
            co2.retain(|line| line[idx] == digit_co2);
        }
    }

    let oxigen_rating = oxigen[0]
        .iter()
        .map(|ch| match ch {
            b'0' => 0,
            b'1' => 1,
            _ => unreachable!(),
        })
        .rev()
        .enumerate()
        .fold(0, |acc, (idx, digit)| {
            let idx: u8 = idx.try_into().unwrap();
            // acc + 2_i64.pow(idx) * digit
            acc + (digit << idx)
        });
    let co2_rating = co2[0]
        .iter()
        .map(|ch| match ch {
            b'0' => 0,
            b'1' => 1,
            _ => unreachable!(),
        })
        .rev()
        .enumerate()
        .fold(0, |acc, (idx, digit)| {
            let idx: u8 = idx.try_into().unwrap();
            // acc + 2_i64.pow(idx) * digit
            acc + (digit << idx)
        });

    (oxigen_rating * co2_rating).to_string()
}

#[test]
fn day3_is_correct() {
    use super::FILES;

    assert_eq!(a(FILES[2]), "3912944");
    assert_eq!(b(FILES[2]), "4996233");
}
