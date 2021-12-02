use atoi::ascii_to_digit;

const FORWARD_LEN: usize = 8;
const DOWN_LEN: usize = 5;
const UP_LEN: usize = 3;

pub fn day2a() -> String {
    let file = std::fs::read_to_string("./files/day2a.txt").unwrap();

    let (horizontal, depth) = file
        .lines()
        .map(|line| {
            let (&last, command) = line.as_bytes().split_last().unwrap();
            let amount = ascii_to_digit(last).unwrap();
            match command.len() {
                FORWARD_LEN => (amount, 0),
                DOWN_LEN => (0, amount),
                UP_LEN => (0, -amount),
                _ => unreachable!(),
            }
        })
        .fold((0_i64, 0_i64), |(ax, ay), (bx, by)| (ax + bx, ay + by));

    (horizontal * depth).to_string()
}

pub fn day2b() -> String {
    let file = std::fs::read_to_string("./files/day2a.txt").unwrap();

    let (horizontal, depth) = file
        .lines()
        .scan(0, |aim, line| {
            let (&last, command) = line.as_bytes().split_last().unwrap();
            let amount = ascii_to_digit(last).unwrap();
            Some(match command.len() {
                FORWARD_LEN => (amount, *aim * amount),
                DOWN_LEN => {
                    *aim += amount;
                    (0, 0)
                }
                UP_LEN => {
                    *aim -= amount;
                    (0, 0)
                }
                _ => unreachable!(),
            })
        })
        .fold((0_i64, 0_i64), |(ax, ay), (bx, by)| (ax + bx, ay + by));

    (horizontal * depth).to_string()
}

#[test]
fn day2_is_correct() {
    assert_eq!(day2a(), "1383564");
    assert_eq!(day2b(), "1488311643");
}
