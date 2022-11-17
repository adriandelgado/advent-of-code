pub(super) fn part1(input: &str) -> String {
    let mut new_input = Vec::from(input.trim_end());
    for _ in 0..40 {
        process_line(&mut new_input);
    }
    new_input.len().to_string()
}

pub(super) fn part2(input: &str) -> String {
    let mut new_input = Vec::from(input.trim_end());
    for _ in 0..50 {
        process_line(&mut new_input);
    }
    new_input.len().to_string()
}

fn process_line(line: &mut Vec<u8>) {
    let prev_line = std::mem::replace(line, Vec::with_capacity(line.len() * 103 / 79));
    let mut iter = prev_line.into_iter();
    let mut current = iter.next().unwrap();
    let mut count = 1;
    for char in iter {
        if char == current {
            count += 1;
        } else {
            line.push(digit_to_byte(count));
            line.push(current);
            current = char;
            count = 1;
        }
    }
    line.push(digit_to_byte(count));
    line.push(current);
}

#[inline]
fn digit_to_byte(input: u8) -> u8 {
    input + b'0'
}
