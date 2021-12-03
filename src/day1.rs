use itertools::Itertools;

pub fn a(file: &str) -> String {
    file.lines()
        .filter_map(|num| num.parse::<u32>().ok())
        .tuple_windows()
        .filter(|(prev, curr)| prev < curr)
        .count()
        .to_string()
}

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
    let file = include_str!("../files/day1.txt");
    assert_eq!(a(file), "1532");
    assert_eq!(b(file), "1571");
}
