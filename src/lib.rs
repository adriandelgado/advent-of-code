pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

pub const FILES: [&str; 4] = [
    include_str!("../files/day01.txt"),
    include_str!("../files/day02.txt"),
    include_str!("../files/day03.txt"),
    include_str!("../files/day04.txt"),
];

pub const SOLUTIONS: [fn(&str) -> String; 8] = [
    day1::a,
    day1::b,
    day2::a,
    day2::b,
    day3::a,
    day3::b,
    day4::a,
    day4::b,
];
