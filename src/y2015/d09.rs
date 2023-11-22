use itertools::Itertools;
use petgraph::{algo::all_simple_paths, prelude::*};
use winnow::{
    ascii::{alpha1, dec_uint},
    combinator::terminated,
    PResult, Parser,
};

pub fn part1(input: &str) -> u32 {
    let graph = UnGraphMap::from_edges(input.lines().map(|line| parse_line.parse(line).unwrap()));
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
                .sum()
        })
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> u32 {
    let graph = UnGraphMap::from_edges(input.lines().map(|line| parse_line.parse(line).unwrap()));
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
                .sum()
        })
        .max()
        .unwrap()
}

fn parse_line<'a>(input: &mut &'a str) -> PResult<(&'a str, &'a str, u32)> {
    (
        terminated(alpha1, " to "),
        terminated(alpha1, " = "),
        dec_uint,
    )
        .parse_next(input)
}
