#[inline]
pub fn a(file: &str) -> String {
    let mut numbers: Vec<i64> = file
        .trim_end()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();
    numbers.sort_unstable();
    let median = numbers[numbers.len() / 2];

    for num in numbers.iter_mut() {
        *num = (median - *num).abs()
    }
    numbers.into_iter().sum::<i64>().to_string()
}

#[inline]
pub fn b(file: &str) -> String {
    let mut numbers: Vec<i64> = file
        .trim_end()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();
    numbers.sort_unstable();
    let avg = numbers.iter().sum::<i64>() / numbers.len() as i64;

    for num in numbers.iter_mut() {
        let n = (avg - *num).abs();
        *num = n * (n + 1) / 2;
    }
    numbers.into_iter().sum::<i64>().to_string()
}

#[test]
fn day7_is_correct() {
    use super::FILES;

    assert_eq!(a(FILES[6]), "348664");
    assert_eq!(b(FILES[6]), "100220525");
}
