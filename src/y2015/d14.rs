use ndarray::{Array2, Axis};
use ndarray_stats::QuantileExt;
use winnow::{
    ascii::{alpha1, dec_uint},
    combinator::terminated,
    PResult, Parser,
};

const TOTAL_SECS: u32 = 2503;

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| parse_line.parse(line).unwrap())
        .map(|reindeer| compute_kms(reindeer, TOTAL_SECS))
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> u32 {
    let reindeer_kms: Vec<u32> = input
        .lines()
        .map(|line| parse_line.parse(line).unwrap())
        .flat_map(|reindeer| (0..TOTAL_SECS).map(move |sec| compute_kms(reindeer, sec)))
        .collect();

    let mut reindeer_kms = Array2::from_shape_vec(
        (
            reindeer_kms.len() / TOTAL_SECS as usize,
            TOTAL_SECS as usize,
        ),
        reindeer_kms,
    )
    .unwrap();

    for mut col in reindeer_kms.columns_mut().into_iter().skip(1) {
        let max = *col.max().unwrap();
        col.mapv_inplace(|kms| (kms == max).into());
    }
    *reindeer_kms.sum_axis(Axis(1)).max().unwrap()
}

#[derive(Debug, Clone, Copy)]
struct Reindeer {
    velocity: u32,
    stamina_secs: u32,
    rest_secs: u32,
}

fn parse_line(input: &mut &str) -> PResult<Reindeer> {
    (
        terminated(alpha1, " can fly "),
        terminated(dec_uint, " km/s for "),
        terminated(dec_uint, " seconds, but then must rest for "),
        terminated(dec_uint, " seconds."),
    )
        .map(|(_, velocity, stamina_secs, rest_secs)| Reindeer {
            velocity,
            stamina_secs,
            rest_secs,
        })
        .parse_next(input)
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
