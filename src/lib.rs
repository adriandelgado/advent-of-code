pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

pub const FILES: [&str; 8] = [
    include_str!("../files/day01.txt"),
    include_str!("../files/day02.txt"),
    include_str!("../files/day03.txt"),
    include_str!("../files/day04.txt"),
    include_str!("../files/day05.txt"),
    include_str!("../files/day06.txt"),
    include_str!("../files/day07.txt"),
    include_str!("../files/day08.txt"),
];

pub const SOLUTIONS: [fn(&str) -> String; 16] = [
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
    day6::a,
    day6::b,
    day7::a,
    day7::b,
    day8::a,
    day8::b,
];
