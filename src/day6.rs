#[inline]
pub fn a(file: &str) -> String {
    let mut amounts = [0; 9];
    for fish in file
        .trim_end()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
    {
        amounts[fish] += 1;
    }
    for _ in 0..80 {
        amounts.rotate_left(1);
        amounts[6] += amounts[8];
    }
    amounts.into_iter().sum::<u64>().to_string()
}

#[inline]
pub fn b(file: &str) -> String {
    let mut amounts = [0; 9];
    for fish in file
        .trim_end()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
    {
        amounts[fish] += 1;
    }
    for _ in 0..256 {
        amounts.rotate_left(1);
        amounts[6] += amounts[8];
    }
    amounts.into_iter().sum::<u64>().to_string()
}

#[test]
fn day6_is_correct() {
    use super::FILES;

    assert_eq!(a(FILES[5]), "360761");
    assert_eq!(b(FILES[5]), "1632779838045");
}
