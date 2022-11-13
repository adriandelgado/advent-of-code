mod d01;
mod d02;
mod d03;

#[must_use]
pub fn solve(day: u8, part: u8, input: &str) -> String {
    match (day, part) {
        (1, 1) => d01::part1(input),
        (1, 2) => d01::part2(input),
        (2, 1) => d02::part1(input),
        (2, 2) => d02::part2(input),
        (3, 1) => d03::part1(input),
        (3, 2) => d03::part2(input),
        _ => unimplemented!(),
    }
}
