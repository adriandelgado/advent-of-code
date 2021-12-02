use itertools::Itertools;

pub fn day1a() -> String {
    let file = std::fs::read_to_string("./files/day1a.txt").unwrap();

    file.lines()
        .filter_map(|num| num.parse::<u32>().ok())
        .tuple_windows()
        .filter(|(prev, curr)| prev < curr)
        .count()
        .to_string()
}

pub fn day1b() -> String {
    let file = std::fs::read_to_string("./files/day1a.txt").unwrap();

    file.lines()
        .filter_map(|num| num.parse::<u32>().ok())
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(prev, curr)| prev < curr)
        .count()
        .to_string()
}

#[test]
fn day1_is_correct() {
    assert_eq!(day1a(), "1532");
    assert_eq!(day1b(), "1571");
}
