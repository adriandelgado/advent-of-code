pub mod day1;
pub mod day2;
pub mod day3;

pub const FILES: [&str; 3] = [
    include_str!("../files/day01.txt"),
    include_str!("../files/day02.txt"),
    include_str!("../files/day03.txt"),
];

pub const SOLUTIONS: [fn(&str) -> String; 6] =
    [day1::a, day1::b, day2::a, day2::b, day3::a, day3::b];
