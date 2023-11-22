use itertools::Itertools;
use petgraph::{algo::all_simple_paths, prelude::UnGraphMap};
use winnow::{
    ascii::{alpha1, dec_int},
    combinator::{alt, delimited, terminated},
    PResult, Parser,
};

pub fn part1(input: &str) -> i64 {
    let mut graph = UnGraphMap::new();

    for (a, b, weight) in input.lines().map(|line| parse_line.parse(line).unwrap()) {
        if let Some(w) = graph.edge_weight_mut(a, b) {
            *w += weight;
        } else {
            graph.add_edge(a, b, weight);
        }
    }

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
            graph.edge_weight(path[0], path.last().unwrap()).unwrap()
                + path
                    .into_iter()
                    .tuple_windows()
                    .filter_map(|(a, b)| graph.edge_weight(a, b))
                    .sum::<i64>()
        })
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> i64 {
    let mut graph = UnGraphMap::new();

    for (a, b, weight) in input.lines().map(|line| parse_line.parse(line).unwrap()) {
        if let Some(w) = graph.edge_weight_mut(a, b) {
            *w += weight;
        } else {
            graph.add_edge(a, b, weight);
        }
    }

    for node in graph.nodes().collect::<Vec<_>>() {
        graph.add_edge("Adrian", node, 0);
    }

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
            graph.edge_weight(path[0], path.last().unwrap()).unwrap()
                + path
                    .into_iter()
                    .tuple_windows()
                    .filter_map(|(a, b)| graph.edge_weight(a, b))
                    .sum::<i64>()
        })
        .max()
        .unwrap()
}

fn parse_line<'a>(input: &mut &'a str) -> PResult<(&'a str, &'a str, i64)> {
    (
        terminated(alpha1, " would "),
        alt(("gain ".value(1), "lose ".value(-1))),
        dec_int,
        delimited(" happiness units by sitting next to ", alpha1, "."),
    )
        .map(|(begin, sign, amount, end): (_, _, i64, _)| (begin, end, sign * amount))
        .parse_next(input)
}
