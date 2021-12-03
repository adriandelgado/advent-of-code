use num::Num;

use atoi::ascii_to_digit;

const FORWARD_LEN: usize = 8;
const DOWN_LEN: usize = 5;
const UP_LEN: usize = 3;

fn process_line_a(line: &str) -> [i64; 2] {
    let (&last, command) = line.as_bytes().split_last().unwrap();
    let amount = ascii_to_digit(last).unwrap();
    match command.len() {
        FORWARD_LEN => [amount, 0],
        DOWN_LEN => [0, amount],
        UP_LEN => [0, -amount],
        _ => unreachable!(),
    }
}

fn process_line_b(aim: &mut i64, line: &str) -> Option<[i64; 2]> {
    let (&last, command) = line.as_bytes().split_last().unwrap();
    let amount = ascii_to_digit(last).unwrap();
    Some(match command.len() {
        FORWARD_LEN => [amount, *aim * amount],
        DOWN_LEN => {
            *aim += amount;
            [0, 0]
        }
        UP_LEN => {
            *aim -= amount;
            [0, 0]
        }
        _ => unreachable!(),
    })
}

fn add_arrays<T, const N: usize>(arr1: [T; N], arr2: [T; N]) -> [T; N]
where
    T: Num + Copy,
{
    let mut result = [T::zero(); N];
    for i in 0..N {
        result[i] = arr1[i] + arr2[i];
    }
    result
}

pub fn a(file: &str) -> String {
    let [horizontal, depth] = file.lines().map(process_line_a).fold([0, 0], add_arrays);
    (horizontal * depth).to_string()
}

pub fn b(file: &str) -> String {
    let [horizontal, depth] = file
        .lines()
        .scan(0, process_line_b)
        .fold([0, 0], add_arrays);

    (horizontal * depth).to_string()
}

#[test]
fn day2_is_correct() {
    use super::FILES;

    assert_eq!(a(FILES[1]), "1383564");
    assert_eq!(b(FILES[1]), "1488311643");
}
