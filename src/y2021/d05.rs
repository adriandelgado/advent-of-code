use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    let mut points = HashMap::new();
    for [start, end] in parse_file(input) {
        let (x0, y0) = start;
        let (x1, y1) = end;
        let dy = (y1 - y0).signum();
        let dx = (x1 - x0).signum();
        if dy * dx != 0 {
            continue;
        }

        for i in 0.. {
            let x = x0 + dx * i;
            let y = y0 + dy * i;
            let count = points.entry((x, y)).or_insert(0);
            *count += 1;
            if (x, y) == (x1, y1) {
                break;
            }
        }
    }
    points.into_values().filter(|count| *count > 1).count()
}

pub fn part2(input: &str) -> usize {
    let mut points = HashMap::new();
    for [start, end] in parse_file(input) {
        let (x0, y0) = start;
        let (x1, y1) = end;
        let dy = (y1 - y0).signum();
        let dx = (x1 - x0).signum();
        for i in 0.. {
            let x = x0 + dx * i;
            let y = y0 + dy * i;
            let count = points.entry((x, y)).or_insert(0);
            *count += 1;
            if (x, y) == (x1, y1) {
                break;
            }
        }
    }
    points.into_values().filter(|count| *count > 1).count()
}

fn parse_file(file: &str) -> impl Iterator<Item = [(i64, i64); 2]> + '_ {
    file.lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(start, end)| {
            let start = parse_coord(start);
            let end = parse_coord(end);
            [start, end]
        })
}

fn parse_coord(coord: &str) -> (i64, i64) {
    let (x, y) = coord.split_once(',').unwrap();
    let x = x.parse().unwrap();
    let y = y.parse().unwrap();
    (x, y)
}
