use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet, VecDeque},
};

use itertools::Itertools;
use winnow::{
    ascii::alpha1,
    combinator::{separated, separated_pair},
    PResult, Parser,
};

pub fn part1(input: &str) -> usize {
    let mut graph: HashMap<_, HashSet<_>> = HashMap::new();
    let mut edges: HashMap<(u16, u16), u16> = HashMap::new();

    for (k, v) in input.lines().map(|line| node.parse(line).unwrap()) {
        for &n in &v {
            graph
                .entry(n)
                .and_modify(|neigh| {
                    neigh.insert(k);
                })
                .or_insert(HashSet::from([k]));

            edges.insert((k.min(n), k.max(n)), 0);
        }
        graph
            .entry(k)
            .and_modify(|neigh| {
                neigh.extend(&v);
            })
            .or_insert(v);
    }

    for &node in graph.keys() {
        let mut queue = VecDeque::from([node]);
        let mut visited = HashSet::from([node]);

        while let Some(curr) = queue.pop_front() {
            for &neigh in &graph[&curr] {
                if visited.insert(neigh) {
                    *edges.get_mut(&(curr.min(neigh), curr.max(neigh))).unwrap() += 1;
                    queue.push_back(neigh);
                }
            }
        }
    }

    let mut edges: Vec<_> = edges.into_iter().collect();
    edges.sort_unstable_by_key(|e| Reverse(e.1));

    for (&a, &b, &c) in edges.iter().map(|(e, _)| e).tuple_combinations() {
        let (node_0, node_1) = a;

        let mut seen_0 = HashSet::from([node_0]);
        let mut stack = vec![node_0];

        while let Some(curr) = stack.pop() {
            for &neigh in &graph[&curr] {
                let curr_edge = (curr.min(neigh), curr.max(neigh));
                if ![a, b, c].contains(&curr_edge) && seen_0.insert(neigh) {
                    stack.push(neigh);
                }
            }
        }

        if seen_0.len() == graph.len() {
            continue;
        }

        let mut seen_1 = HashSet::from([node_1]);
        let mut stack = vec![node_1];

        while let Some(curr) = stack.pop() {
            for &neigh in &graph[&curr] {
                let curr_edge = (curr.min(neigh), curr.max(neigh));
                if ![a, b, c].contains(&curr_edge) && seen_1.insert(neigh) {
                    stack.push(neigh);
                }
            }
        }

        if seen_0.len() + seen_1.len() == graph.len() {
            return seen_0.len() * seen_1.len();
        }
    }

    unreachable!()
}

pub fn part2(_input: &str) -> &'static str {
    "Happy Holidays!"
}

fn node(input: &mut &str) -> PResult<(u16, HashSet<u16>)> {
    separated_pair(
        alpha1.map(str_to_u16),
        ": ",
        separated(1.., alpha1.map(str_to_u16), ' '),
    )
    .parse_next(input)
}

fn str_to_u16(input: &str) -> u16 {
    let &[a, b, c] = input.as_bytes() else {
        unreachable!()
    };
    u16::from(a) * 26 * 26 + u16::from(b) * 26 + u16::from(c)
}
