use std::{cmp::Ordering, ops::Range};

use atoi::FromRadix10;

pub fn part1(input: &str) -> u64 {
    let (seeds, mappings) = extract_info(input);

    seeds
        .into_iter()
        .map(|seed| seed_to_location(seed, &mappings))
        .min()
        .unwrap()
}

fn seed_to_location(seed: u64, mappings: &[SortedMapping]) -> u64 {
    let mut current_out = seed;
    for mapping in mappings {
        current_out = do_the_mapping(current_out, mapping);
    }
    current_out
}

fn do_the_mapping(current_out: u64, mapping: &[(Range<u64>, u64)]) -> u64 {
    mapping
        .binary_search_by(|(source_r, _)| match source_r.start.cmp(&current_out) {
            Ordering::Less if source_r.end.cmp(&current_out) == Ordering::Greater => {
                Ordering::Equal
            }
            order => order,
        })
        .map_or(current_out, |location| {
            let (range, destination) = &mapping[location];
            let shift = current_out - range.start;
            destination + shift
        })
}

pub fn part2(input: &str) -> u64 {
    let (seeds, mappings) = extract_info(input);

    seeds
        .chunks_exact(2)
        .flat_map(|range| {
            let &[start, length] = range else {
                unreachable!()
            };

            start..(start + length)
        })
        .map(|seed| seed_to_location(seed, &mappings))
        .min()
        .unwrap()
}

// destination source range -> source..(source + range) destination
type SortedMapping = Vec<(Range<u64>, u64)>;
fn extract_info(input: &str) -> (Vec<u64>, Vec<SortedMapping>) {
    let (seeds, mappings) = input.split_once("\n\n").unwrap();
    let seeds = seeds[7..]
        .split(' ')
        .map(|seed| u64::from_radix_10(seed.as_bytes()).0)
        .collect();

    let mappings = mappings
        .split("\n\n")
        .map(|mapping| {
            let mut mapping: Vec<_> = mapping
                .split_once('\n')
                .unwrap()
                .1
                .lines()
                .map(|line| {
                    let line = line.as_bytes();
                    let (destination, next) = u64::from_radix_10(line);
                    let line = &line[next + 1..];
                    let (source, next) = u64::from_radix_10(line);
                    let line = &line[next + 1..];
                    let (range, _) = u64::from_radix_10(line);

                    (source..(source + range), destination)
                })
                .collect();

            mapping.sort_unstable_by_key(|(source_r, _)| source_r.start);

            mapping
        })
        .collect();

    (seeds, mappings)
}
