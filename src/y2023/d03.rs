use atoi::FromRadix10;

pub fn part1(input: &str) -> u32 {
    let modulo = input.find('\n').unwrap() + 1;

    input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            let mut sum = 0;
            let mut col = 0;
            while col < line.len() {
                let (number, len) = u32::from_radix_10(line[col..].as_bytes());
                if len > 0 && has_adjacent_sym(len, (row, col), input.as_bytes(), modulo) {
                    sum += number;
                    col += len;
                } else {
                    col += 1;
                }
            }
            sum
        })
        .sum()
}

fn has_adjacent_sym(len: usize, (row, col): (usize, usize), input: &[u8], modulo: usize) -> bool {
    let min_row = row.saturating_sub(1);
    let max_row = (row + 1).min((input.len() / modulo) - 1);
    let min_col = col.saturating_sub(1);
    let max_col = (col + len).min(modulo - 2);

    for row in min_row..=max_row {
        for col in min_col..=max_col {
            if !matches!(input[row * modulo + col], b'0'..=b'9' | b'.') {
                return true;
            }
        }
    }

    false
}

pub fn part2(input: &str) -> u32 {
    let modulo = input.find('\n').unwrap() + 1;

    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter(|(_, &ch)| (ch == b'*'))
                .map(move |(col, _)| get_gear_ratio((row, col), input.as_bytes(), modulo))
        })
        .sum()
}

fn get_gear_ratio((row, col): (usize, usize), input: &[u8], modulo: usize) -> u32 {
    let mut gear_ratio = 1;
    let mut amount_of_parts = 0;

    let min_row = row.saturating_sub(1);
    let max_row = (row + 1).min((input.len() / modulo) - 1);

    for row in min_row..=max_row {
        let slice = &input[row * modulo..][..modulo - 1];
        for num in get_numbers(slice, col) {
            gear_ratio *= num;
            amount_of_parts += 1;
        }
    }

    gear_ratio * u32::from(amount_of_parts == 2)
}

fn get_numbers(input: &[u8], col: usize) -> impl Iterator<Item = u32> + '_ {
    let mut start = 0;
    let min_col = col.saturating_sub(1);
    let max_col = (col + 1).min(input.len() - 1);
    std::iter::from_fn(move || {
        while start <= max_col {
            let (number, len) = u32::from_radix_10(&input[start..]);

            if (min_col..=max_col).any(|c| (start..start + len).contains(&c)) {
                start += len;
                return Some(number);
            }

            start += len.max(1);
        }
        None
    })
}
