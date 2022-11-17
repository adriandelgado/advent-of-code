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

#[must_use]
pub fn solve(day: u8, part: u8, input: &str) -> String {
    match (day, part) {
        (1, 1) => d01::part1(input),
        (1, 2) => d01::part2(input),
        (2, 1) => d02::part1(input),
        (2, 2) => d02::part2(input),
        (3, 1) => d03::part1(input),
        (3, 2) => d03::part2(input),
        (4, 1) => d04::part1(input),
        (4, 2) => d04::part2(input),
        (5, 1) => d05::part1(input),
        (5, 2) => d05::part2(input),
        (6, 1) => d06::part1(input),
        (6, 2) => d06::part2(input),
        (7, 1) => d07::part1(input),
        (7, 2) => d07::part2(input),
        (8, 1) => d08::part1(input),
        (8, 2) => d08::part2(input),
        (9, 1) => d09::part1(input),
        (9, 2) => d09::part2(input),
        (10, 1) => d10::part1(input),
        (10, 2) => d10::part2(input),
        _ => unimplemented!(),
    }
}
