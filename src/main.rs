use std::env;

use aoc::{
    day1::{day1a, day1b},
    day2::{day2a, day2b},
    day3::{day3a, day3b},
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let problem = args.get(1).map(String::as_str);

    let result = match problem {
        // Some("1a") => day1a(),
        // Some("1b") => day1b(),
        // Some("2a") => day2a(),
        // Some("2b") => day2b(),
        // Some("3a") => day3a(),
        // Some("3b") => day3b(),
        None => "Please input a day to solve".to_string(),
        _ => "Not solved yet".to_string(),
    };

    println!("{}", result);
}
