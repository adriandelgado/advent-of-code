use ndarray::{Array2, Axis};
use ndarray_stats::QuantileExt;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, u32},
    combinator::map,
    sequence::{terminated, tuple},
    IResult,
};

const TOTAL_SECS: usize = 2503;

pub(super) fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .map(|reindeer| compute_kms(reindeer, TOTAL_SECS as u32))
        .max()
        .unwrap()
        .to_string()
}

pub(super) fn part2(input: &str) -> String {
    let reindeer_kms: Vec<u32> = input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .flat_map(|reindeer| (0..TOTAL_SECS).map(move |sec| compute_kms(reindeer, sec as u32)))
        .collect();

    let mut reindeer_kms =
        Array2::from_shape_vec((reindeer_kms.len() / TOTAL_SECS, TOTAL_SECS), reindeer_kms)
            .unwrap();

    for mut col in reindeer_kms.columns_mut().into_iter().skip(1) {
        let max = *col.max().unwrap();
        col.mapv_inplace(|kms| (kms == max).into());
    }
    reindeer_kms.sum_axis(Axis(1)).max().unwrap().to_string()
}

#[derive(Debug, Clone, Copy)]
struct Reindeer {
    velocity: u32,
    stamina_secs: u32,
    rest_secs: u32,
}

fn parse_line(input: &str) -> IResult<&str, Reindeer> {
    map(
        tuple((
            terminated(alpha1, tag(" can fly ")),
            terminated(u32, tag(" km/s for ")),
            terminated(u32, tag(" seconds, but then must rest for ")),
            terminated(u32, tag(" seconds.")),
        )),
        |(_, velocity, stamina_secs, rest_secs)| Reindeer {
            velocity,
            stamina_secs,
            rest_secs,
        },
    )(input)
}

fn compute_kms(
    Reindeer {
        velocity,
        stamina_secs,
        rest_secs,
    }: Reindeer,
    secs: u32,
) -> u32 {
    let secs_per_cycle = stamina_secs + rest_secs;

    let complete_cycles = secs / secs_per_cycle;

    let starting_kms = velocity * stamina_secs * complete_cycles;

    let other_secs = secs % secs_per_cycle;

    let rest_kms = velocity * other_secs.min(stamina_secs);

    starting_kms + rest_kms
}
