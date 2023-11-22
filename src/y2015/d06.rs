use ndarray::{s, Array2, Dim, SliceInfo, SliceInfoElem};
use winnow::{
    ascii::dec_uint,
    combinator::{alt, separated_pair},
    PResult, Parser,
};

use std::ops::Not;

pub fn part1(input: &str) -> u32 {
    let mut arr = Array2::<bool>::default((1000, 1000));
    for action in input.lines().map(|line| action.parse(line).unwrap()) {
        match action.kind {
            ActionKind::TurnOn => arr.slice_mut(action.slice).fill(true),
            ActionKind::TurnOff => arr.slice_mut(action.slice).fill(false),
            ActionKind::Toggle => arr.slice_mut(action.slice).mapv_inplace(Not::not),
        }
    }

    arr.mapv(u32::from).sum()
}

pub fn part2(input: &str) -> u32 {
    let mut arr = Array2::<u32>::default((1000, 1000));
    for action in input.lines().map(|line| action.parse(line).unwrap()) {
        match action.kind {
            ActionKind::TurnOn => arr.slice_mut(action.slice).mapv_inplace(|n| n + 1),
            ActionKind::TurnOff => arr
                .slice_mut(action.slice)
                .mapv_inplace(|n| n.saturating_sub(1)),
            ActionKind::Toggle => arr.slice_mut(action.slice).mapv_inplace(|n| n + 2),
        }
    }
    arr.sum()
}

fn action(input: &mut &str) -> PResult<Action> {
    (
        alt((
            "turn on ".value(ActionKind::TurnOn),
            "turn off ".value(ActionKind::TurnOff),
            "toggle ".value(ActionKind::Toggle),
        )),
        separated_pair(
            separated_pair(dec_uint, ",", dec_uint),
            " through ",
            separated_pair(dec_uint, ",", dec_uint),
        ),
    )
        .map(|(kind, ((x1, y1), (x2, y2))): MapInput| Action {
            kind,
            slice: s![x1..=x2, y1..=y2],
        })
        .parse_next(input)
}

type MapInput = (ActionKind, ((i32, i32), (i32, i32)));

#[derive(Debug, Clone, Copy)]
enum ActionKind {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug)]
struct Action {
    kind: ActionKind,
    slice: SliceInfo<[SliceInfoElem; 2], Dim<[usize; 2]>, Dim<[usize; 2]>>,
}
