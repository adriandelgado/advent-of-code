use std::fmt::Display;

use crate::timing_fn;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;

#[must_use]
pub fn solve(day: u8, part: u8, input: &str) -> Box<dyn Display> {
    match (day, part) {
        (1, 1) => Box::new(timing_fn(|| d01::part1(input))),
        (1, 2) => Box::new(timing_fn(|| d01::part2(input))),
        (2, 1) => Box::new(timing_fn(|| d02::part1(input))),
        (2, 2) => Box::new(timing_fn(|| d02::part2(input))),
        (3, 1) => Box::new(timing_fn(|| d03::part1(input))),
        (3, 2) => Box::new(timing_fn(|| d03::part2(input))),
        (4, 1) => Box::new(timing_fn(|| d04::part1(input))),
        (4, 2) => Box::new(timing_fn(|| d04::part2(input))),
        (5, 1) => Box::new(timing_fn(|| d05::part1(input))),
        (5, 2) => Box::new(timing_fn(|| d05::part2(input))),
        (6, 1) => Box::new(timing_fn(|| d06::part1(input))),
        (6, 2) => Box::new(timing_fn(|| d06::part2(input))),
        (7, 1) => Box::new(timing_fn(|| d07::part1(input))),
        (7, 2) => Box::new(timing_fn(|| d07::part2(input))),
        (8, 1) => Box::new(timing_fn(|| d08::part1(input))),
        (8, 2) => Box::new(timing_fn(|| d08::part2(input))),
        _ => unimplemented!(), // TODO: unreachable once completed
    }
}
