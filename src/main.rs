use std::env;

use aoc::day1::{day1a, day1b};

fn main() {
    let args: Vec<String> = env::args().collect();

    let problema = args.get(1).map(String::as_str);

    let result = match problema {
        Some("1a") => day1a(),
        Some("1b") => day1b(),
        None => "Please input a day to solve".to_string(),
        _ => "Not solved yet".to_string(),
    };

    println!("{}", result);
}
