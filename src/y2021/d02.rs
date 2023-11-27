use winnow::{
    ascii::dec_uint,
    combinator::{alt, separated_pair},
    PResult, Parser,
};

pub fn part1(input: &str) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    for (direction, units) in input.lines().map(|line| line_parser.parse(line).unwrap()) {
        match direction {
            Direction::Forward => horizontal += units,
            Direction::Down => depth += units,
            Direction::Up => depth -= units,
        }
    }
    horizontal * depth
}

pub fn part2(input: &str) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for (direction, units) in input.lines().map(|line| line_parser.parse(line).unwrap()) {
        match direction {
            Direction::Forward => {
                horizontal += units;
                depth += aim * units;
            }
            Direction::Down => aim += units,
            Direction::Up => aim -= units,
        }
    }
    horizontal * depth
}

#[derive(Clone, Copy)]
enum Direction {
    Forward,
    Up,
    Down,
}

fn line_parser(input: &mut &str) -> PResult<(Direction, i32)> {
    separated_pair(
        alt((
            "forward".value(Direction::Forward),
            "up".value(Direction::Up),
            "down".value(Direction::Down),
        )),
        ' ',
        dec_uint,
    )
    .parse_next(input)
}
