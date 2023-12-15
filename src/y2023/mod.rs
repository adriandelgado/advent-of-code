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
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;

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
        (9, 1) => Box::new(timing_fn(|| d09::part1(input))),
        (9, 2) => Box::new(timing_fn(|| d09::part2(input))),
        (10, 1) => Box::new(timing_fn(|| d10::part1(input))),
        (10, 2) => Box::new(timing_fn(|| d10::part2(input))),
        (11, 1) => Box::new(timing_fn(|| d11::part1(input))),
        (11, 2) => Box::new(timing_fn(|| d11::part2(input))),
        (12, 1) => Box::new(timing_fn(|| d12::part1(input))),
        (12, 2) => Box::new(timing_fn(|| d12::part2(input))),
        (13, 1) => Box::new(timing_fn(|| d13::part1(input))),
        (13, 2) => Box::new(timing_fn(|| d13::part2(input))),
        (14, 1) => Box::new(timing_fn(|| d14::part1(input))),
        (14, 2) => Box::new(timing_fn(|| d14::part2(input))),
        (15, 1) => Box::new(timing_fn(|| d15::part1(input))),
        (15, 2) => Box::new(timing_fn(|| d15::part2(input))),
        _ => unimplemented!(), // TODO: unreachable once completed
    }
}
