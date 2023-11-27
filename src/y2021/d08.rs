use bstr::ByteSlice;

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| line.rsplit_once(" | ").unwrap().1.split(' '))
        .filter(|word| matches!(word.len(), 2 | 3 | 4 | 7))
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.rsplit_once(" | ").unwrap())
        .map(|(clues, output)| {
            let numbers = extract_numbers(clues);

            output
                .split(' ')
                .map(|num_str| {
                    let bitset = bitset_from_str(num_str.as_bytes());
                    numbers.find_byte(bitset).unwrap()
                })
                .zip([1000, 100, 10, 1])
                .map(|(digit, power)| digit * power)
                .sum::<usize>()
        })
        .sum()
}

fn extract_numbers(clues: &str) -> [u8; 10] {
    let mut one = 0;
    let mut seven = 0;
    let mut four = 0;
    for clue in clues.as_bytes().split_str(" ") {
        match clue.len() {
            2 => one = bitset_from_str(clue),
            3 => seven = bitset_from_str(clue),
            4 => four = bitset_from_str(clue),
            _ => {}
        }
    }
    let mut nine = 0;
    let mut zero = 0;
    let mut three = 0;
    for clue in clues.as_bytes().split_str(" ") {
        let bitset = bitset_from_str(clue);
        if one & bitset == one {
            match (clue.len(), four & bitset == four) {
                (5, _) => three = bitset,
                (6, true) => nine = bitset,
                (6, false) => zero = bitset,
                _ => {}
            }
        }
    }
    let discriminant = four ^ one;
    let mut five = 0;
    let mut six = 0;
    let mut two = 0;
    for clue in clues.as_bytes().split_str(" ") {
        let bitset = bitset_from_str(clue);
        match (clue.len(), bitset & discriminant == discriminant) {
            (5, true) => five = bitset,
            (5, false) if bitset != three => two = bitset,
            (6, true) if bitset != nine => six = bitset,
            _ => {}
        }
    }

    [
        zero, one, two, three, four, five, six, seven, 0b111_1111, nine,
    ]
}

fn bitset_from_str(clue: &[u8]) -> u8 {
    clue.iter()
        .map(|ch| ch - b'a')
        .fold(0, |acc, x| acc | (1 << x))
}
