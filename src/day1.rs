use itertools::Itertools;

#[inline]
pub fn a(file: &str) -> String {
    file.lines()
        .filter_map(|num| num.parse::<u32>().ok())
        .tuple_windows()
        .filter(|(prev, curr)| prev < curr)
        .count()
        .to_string()
}

#[inline]
pub fn b(file: &str) -> String {
    file.lines()
        .filter_map(|num| num.parse::<u32>().ok())
        .tuple_windows()
        .filter(|(a, _, _, d)| a < d)
        .count()
        .to_string()
}

#[test]
fn day1_is_correct() {
    use super::FILES;

    assert_eq!(a(FILES[0]), "1532");
    assert_eq!(b(FILES[0]), "1571");
}
