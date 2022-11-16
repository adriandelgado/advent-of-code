use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, u32},
    sequence::{terminated, tuple},
    IResult,
};
use petgraph::{algo::all_simple_paths, prelude::*};

pub(super) fn part1(input: &str) -> String {
    let graph = UnGraphMap::from_edges(input.lines().map(|line| parse_line(line).unwrap().1));
    graph
        .nodes()
        .combinations(2)
        .flat_map(|extrema| {
            all_simple_paths::<Vec<_>, _>(
                &graph,
                extrema[0],
                extrema[1],
                graph.node_count() - 2,
                None,
            )
        })
        .map(|path| {
            path.into_iter()
                .tuple_windows()
                .filter_map(|(a, b)| graph.edge_weight(a, b))
                .sum::<u32>()
        })
        .min()
        .unwrap()
        .to_string()
}

pub(super) fn part2(input: &str) -> String {
    let graph = UnGraphMap::from_edges(input.lines().map(|line| parse_line(line).unwrap().1));
    graph
        .nodes()
        .combinations(2)
        .flat_map(|extrema| {
            all_simple_paths::<Vec<_>, _>(
                &graph,
                extrema[0],
                extrema[1],
                graph.node_count() - 2,
                None,
            )
        })
        .map(|path| {
            path.into_iter()
                .tuple_windows()
                .filter_map(|(a, b)| graph.edge_weight(a, b))
                .sum::<u32>()
        })
        .max()
        .unwrap()
        .to_string()
}

fn parse_line(input: &str) -> IResult<&str, (&str, &str, u32)> {
    tuple((
        terminated(alpha1, tag(" to ")),
        terminated(alpha1, tag(" = ")),
        u32,
    ))(input)
}
