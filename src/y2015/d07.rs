use itertools::Itertools;
use petgraph::{algo::toposort, prelude::DiGraphMap, Direction};
use winnow::{
    ascii::{alpha1, dec_uint},
    combinator::{alt, preceded, separated_pair},
    PResult, Parser,
};

use std::collections::HashMap;

pub fn part1(input: &str) -> u16 {
    let circuit = DiGraphMap::from_edges(
        input
            .lines()
            .flat_map(|line| parse_line.parse(line).unwrap()),
    );

    let mut signals = HashMap::new();

    for curr_node in toposort(&circuit, None).unwrap() {
        let Entry::Label(curr_label) = curr_node else {
            continue;
        };
        let mut edges = circuit.edges_directed(curr_node, Direction::Incoming);
        let (prev_node_1, _, operation) = edges.next().unwrap();
        let prev_value_1 = match prev_node_1 {
            Entry::Label(prev_label_1) => signals[prev_label_1],
            Entry::Signal(prev_value_1) => prev_value_1,
        };
        let curr_value = match operation {
            Operator::RShift(shift) => prev_value_1 >> shift,
            Operator::LShift(shift) => prev_value_1 << shift,
            Operator::Not => !prev_value_1,
            Operator::And1 => 1 & prev_value_1,
            Operator::Start => prev_value_1,
            Operator::And => {
                let (prev_node_2, _, Operator::And) = edges.next().unwrap() else {
                    unreachable!()
                };
                let prev_value_2 = match prev_node_2 {
                    Entry::Label(prev_label_2) => signals[prev_label_2],
                    Entry::Signal(prev_value_2) => prev_value_2,
                };
                prev_value_1 & prev_value_2
            }
            Operator::Or => {
                let (prev_node_2, _, Operator::Or) = edges.next().unwrap() else {
                    unreachable!()
                };
                let prev_value_2 = match prev_node_2 {
                    Entry::Label(prev_label_2) => signals[prev_label_2],
                    Entry::Signal(prev_value_2) => prev_value_2,
                };
                prev_value_1 | prev_value_2
            }
        };
        signals.insert(curr_label, curr_value);
    }

    signals["a"]
}

pub fn part2(input: &str) -> u16 {
    let part1_str = part1(input).to_string();
    let part_1_result = part1_str + " -> b";
    let new_input = input
        .lines()
        .map(|line| {
            if line.ends_with(" -> b") {
                &part_1_result
            } else {
                line
            }
        })
        .join("\n");

    part1(&new_input)
}

fn parse_line<'a>(input: &mut &'a str) -> PResult<Connection<'a>> {
    let (input, output) = separated_pair(
        alt((
            preceded("NOT ", entry).map(ConnectionInput::Not),
            preceded("1 AND ", entry).map(ConnectionInput::And1),
            separated_pair(entry, " AND ", entry).map(ConnectionInput::And),
            separated_pair(entry, " OR ", entry).map(ConnectionInput::Or),
            separated_pair(entry, " LSHIFT ", dec_uint).map(ConnectionInput::LShift),
            separated_pair(entry, " RSHIFT ", dec_uint).map(ConnectionInput::RShift),
            entry.map(ConnectionInput::Start),
        )),
        " -> ",
        entry,
    )
    .parse_next(input)?;

    let mut conn = Connection::default();

    match input {
        ConnectionInput::And((lhs, rhs)) => {
            conn.first = Some((lhs, output, Operator::And));
            conn.second = Some((rhs, output, Operator::And));
        }
        ConnectionInput::Or((lhs, rhs)) => {
            conn.first = Some((lhs, output, Operator::Or));
            conn.second = Some((rhs, output, Operator::Or));
        }
        ConnectionInput::RShift((input, shift)) => {
            conn.first = Some((input, output, Operator::RShift(shift)));
        }
        ConnectionInput::LShift((input, shift)) => {
            conn.first = Some((input, output, Operator::LShift(shift)));
        }
        ConnectionInput::Not(input) => {
            conn.first = Some((input, output, Operator::Not));
        }
        ConnectionInput::Start(input) => {
            conn.first = Some((input, output, Operator::Start));
        }
        ConnectionInput::And1(input) => {
            conn.first = Some((input, output, Operator::And1));
        }
    };

    Ok(conn)
}

fn entry<'a>(input: &mut &'a str) -> PResult<Entry<'a>> {
    alt((dec_uint.map(Entry::Signal), alpha1.map(Entry::Label))).parse_next(input)
}

#[derive(Debug)]
enum ConnectionInput<'a> {
    And((Entry<'a>, Entry<'a>)),
    Or((Entry<'a>, Entry<'a>)),
    RShift((Entry<'a>, u16)),
    LShift((Entry<'a>, u16)),
    Not(Entry<'a>),
    And1(Entry<'a>),
    Start(Entry<'a>),
}

#[derive(Debug)]
enum Operator {
    And,
    Or,
    Not,
    RShift(u16),
    LShift(u16),
    And1,
    Start,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Entry<'a> {
    Label(&'a str),
    Signal(u16),
}

#[derive(Debug, Default)]
struct Connection<'a> {
    first: Option<(Entry<'a>, Entry<'a>, Operator)>,
    second: Option<(Entry<'a>, Entry<'a>, Operator)>,
}

impl<'a> Iterator for Connection<'a> {
    type Item = (Entry<'a>, Entry<'a>, Operator);

    fn next(&mut self) -> Option<Self::Item> {
        self.first.take().or_else(|| self.second.take())
    }
}
