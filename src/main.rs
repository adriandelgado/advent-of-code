use std::env;

use aoc::{day1, day2, day3};

fn main() {
    let args: Vec<String> = env::args().collect();

    let problem = args.get(1).map(String::as_str);

    let result = match problem {
        Some("1a") => day1::a(aoc::FILE1),
        Some("1b") => day1::b(aoc::FILE1),
        Some("2a") => day2::a(aoc::FILE2),
        Some("2b") => day2::b(aoc::FILE2),
        Some("3a") => day3::a(aoc::FILE3),
        Some("3b") => day3::b(aoc::FILE3),
        None => "Please input a day to solve".to_string(),
        _ => "Not solved yet".to_string(),
    };

    println!("{}", result);
}
