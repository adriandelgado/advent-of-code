use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, i64},
    combinator::value,
    sequence::{delimited, terminated, tuple},
    IResult,
};
use petgraph::{algo::all_simple_paths, prelude::UnGraphMap};

pub(super) fn part1(input: &str) -> String {
    let mut graph = UnGraphMap::new();

    for (a, b, weight) in input.lines().map(|line| parse_line(line).unwrap().1) {
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
        .to_string()
}

pub(super) fn part2(input: &str) -> String {
    let mut graph = UnGraphMap::new();

    for (a, b, weight) in input.lines().map(|line| parse_line(line).unwrap().1) {
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
        .to_string()
}

fn parse_line(input: &str) -> IResult<&str, (&str, &str, i64)> {
    let (rest, (begin, sign, amount, end)) = tuple((
        terminated(alpha1, tag(" would ")),
        alt((value(1, tag("gain ")), value(-1, tag("lose ")))),
        i64,
        delimited(
            tag(" happiness units by sitting next to "),
            alpha1,
            tag("."),
        ),
    ))(input)?;

    Ok((rest, (begin, end, sign * amount)))
}
