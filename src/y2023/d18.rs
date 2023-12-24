use winnow::{
    ascii::{dec_uint, hex_uint},
    combinator::{delimited, dispatch, rest, separated_pair, success, terminated},
    token::{any, take, take_till},
    PResult, Parser,
};

pub fn part1(input: &str) -> i32 {
    let (mut x, mut y) = (0, 0);
    let mut twice_area = 0;
    let mut total_border = 0;
    for (dir, amount) in input
        .lines()
        .map(|line| trench.parse(line.as_bytes()).unwrap())
    {
        let (last_x, last_y) = (x, y);
        let amount = i32::from(amount);
        match dir {
            Dir::Right => y += amount,
            Dir::Down => x += amount,
            Dir::Left => y -= amount,
            Dir::Up => x -= amount,
        }
        twice_area += (last_y + y) * (last_x - x);
        total_border += amount;
    }
    (twice_area.abs() + total_border) / 2 + 1
}

pub fn part2(input: &str) -> i64 {
    let (mut x, mut y) = (0, 0);
    let mut twice_area = 0;
    let mut total_border = 0;
    for (amount, dir) in input
        .lines()
        .map(|line| hex_trench.parse(line.as_bytes()).unwrap())
    {
        let (last_x, last_y) = (x, y);
        let amount = i64::from(amount);
        match dir {
            Dir::Right => y += amount,
            Dir::Down => x += amount,
            Dir::Left => y -= amount,
            Dir::Up => x -= amount,
        }
        twice_area += (last_y + y) * (last_x - x);
        total_border += amount;
    }
    (twice_area.abs() + total_border) / 2 + 1
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
}

fn trench(input: &mut &[u8]) -> PResult<(Dir, u8)> {
    terminated(
        separated_pair(
            dispatch! {any;
                b'R' => success(Dir::Right),
                b'D' => success(Dir::Down),
                b'L' => success(Dir::Left),
                _ => success(Dir::Up),
            },
            b' ',
            dec_uint,
        ),
        rest,
    )
    .parse_next(input)
}

fn hex_trench(input: &mut &[u8]) -> PResult<(u32, Dir)> {
    delimited(
        (take_till(5.., b'#'), any),
        (
            take(5_usize).and_then(hex_uint),
            dispatch! {any;
                b'0' => success(Dir::Right),
                b'1' => success(Dir::Down),
                b'2' => success(Dir::Left),
                _ => success(Dir::Up),
            },
        ),
        b')',
    )
    .parse_next(input)
}
