mod day1;
mod day2;
mod day3;

pub const FILES: [&str; 3] = [
    include_str!("../files/day1.txt"),
    include_str!("../files/day2.txt"),
    include_str!("../files/day3.txt"),
];

pub const SOLUTIONS: [fn(&str) -> String; 6] =
    [day1::a, day1::b, day2::a, day2::b, day3::a, day3::b];
