use std::fmt::Display;

mod d01;

#[must_use]
pub fn solve(day: u8, part: u8, input: &str) -> Box<dyn Display> {
    match (day, part) {
        (1, 1) => Box::new(d01::part1(input)),
        (1, 2) => Box::new(d01::part2(input)),
        _ => unimplemented!(), // TODO: unreachable once completed
    }
}
