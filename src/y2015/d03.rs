use std::collections::HashSet;

pub fn part1(input: &str) -> usize {
    let mut houses = HashSet::new();

    houses.insert([0, 0]);

    let mut current = [0, 0];

    for c in input.chars() {
        let vector = match c {
            '>' => [1, 0],
            '^' => [0, 1],
            '<' => [-1, 0],
            'v' => [0, -1],
            _ => unreachable!(),
        };
        current = add_arr(current, vector);
        houses.insert(current);
    }
    houses.len()
}

pub fn part2(input: &str) -> usize {
    let mut houses = HashSet::new();

    houses.insert([0, 0]);

    let mut current_1 = [0, 0];
    let mut current_2 = [0, 0];

    for (i, c) in input.chars().enumerate() {
        let current = if i % 2 == 0 {
            &mut current_1
        } else {
            &mut current_2
        };
        let vector = match c {
            '>' => [1, 0],
            '^' => [0, 1],
            '<' => [-1, 0],
            'v' => [0, -1],
            _ => unreachable!(),
        };
        *current = add_arr(*current, vector);
        houses.insert(*current);
    }
    houses.len()
}

fn add_arr(arr1: [i32; 2], arr2: [i32; 2]) -> [i32; 2] {
    let [x1, y1] = arr1;
    let [x2, y2] = arr2;
    [x1 + x2, y1 + y2]
}
