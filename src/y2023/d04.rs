pub fn part1(input: &str) -> u16 {
    input
        .lines()
        .map(get_matching)
        .map(|matching| 1 << matching >> 1)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut cards: Vec<_> = input
        .lines()
        .map(get_matching)
        .map(|matching| (1, matching))
        .collect();

    for idx in 0..cards.len() {
        let (amount, matching) = cards[idx];
        for next_card in cards.iter_mut().skip(idx + 1).take(matching) {
            next_card.0 += amount;
        }
    }

    cards.into_iter().map(|(amount, _)| amount).sum()
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
