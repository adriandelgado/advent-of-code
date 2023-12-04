pub fn part1(input: &str) -> u16 {
    input
        .lines()
        .map(get_matching)
        .map(|matching| 1 << matching >> 1)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut cards: Vec<_> = input.lines().map(|_| 1).collect();

    for (idx, matching) in input.lines().map(get_matching).enumerate() {
        let [amount, next_cards @ ..] = &mut cards[idx..=(idx + matching)] else {
            unreachable!()
        };
        next_cards.iter_mut().for_each(|c| *c += *amount);
    }

    cards.into_iter().sum()
}

fn get_matching(input: &str) -> usize {
    let input = input.as_bytes();
    assert_eq!(input.len(), 116);
    let have = &input[9..39];
    let winning = &input[41..];
    let mut matching = 0;
    for h in have.chunks_exact(3) {
        for w in winning.chunks_exact(3) {
            matching += usize::from(h == w);
        }
    }
    matching
}
