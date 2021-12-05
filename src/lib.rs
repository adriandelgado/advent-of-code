pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

pub const FILES: [&str; 5] = [
    include_str!("../files/day01.txt"),
    include_str!("../files/day02.txt"),
    include_str!("../files/day03.txt"),
    include_str!("../files/day04.txt"),
    include_str!("../files/day05.txt"),
];

pub const SOLUTIONS: [fn(&str) -> String; 10] = [
    day1::a,
    day1::b,
    day2::a,
    day2::b,
    day3::a,
    day3::b,
    day4::a,
    day4::b,
    day5::a,
    day5::b,
];
