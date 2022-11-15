use ndarray::{Array2, Dim, SliceInfo, SliceInfoElem};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u16,
    combinator::value,
    sequence::{separated_pair, tuple},
    IResult,
};
use std::ops::Not;

pub(super) fn part1(input: &str) -> String {
    let mut arr = Array2::<bool>::default((1000, 1000));
    for action in input.lines().map(|line| action(line).unwrap().1) {
        match action.kind {
            ActionKind::TurnOn => arr.slice_mut(action.slice).fill(true),
            ActionKind::TurnOff => arr.slice_mut(action.slice).fill(false),
            ActionKind::Toggle => arr.slice_mut(action.slice).mapv_inplace(Not::not),
        }
    }

    arr.mapv(u32::from).sum().to_string()
}

pub(super) fn part2(input: &str) -> String {
    let mut arr = Array2::<u32>::default((1000, 1000));
    for action in input.lines().map(|line| action(line).unwrap().1) {
        match action.kind {
            ActionKind::TurnOn => arr.slice_mut(action.slice).mapv_inplace(|n| n + 1),
            ActionKind::TurnOff => arr
                .slice_mut(action.slice)
                .mapv_inplace(|n| n.saturating_sub(1)),
            ActionKind::Toggle => arr.slice_mut(action.slice).mapv_inplace(|n| n + 2),
        }
    }
    arr.sum().to_string()
}

fn action(input: &str) -> IResult<&str, Action> {
    let (rest, (kind, ((x1, y1), (x2, y2)))) = tuple((
        alt((
            value(ActionKind::TurnOn, tag("turn on ")),
            value(ActionKind::TurnOff, tag("turn off ")),
            value(ActionKind::Toggle, tag("toggle ")),
        )),
        separated_pair(
            separated_pair(u16, tag(","), u16),
            tag(" through "),
            separated_pair(u16, tag(","), u16),
        ),
    ))(input)?;

    let slice_start = pair_to_slice(x1, x2);
    let slice_end = pair_to_slice(y1, y2);
    Ok((
        rest,
        Action {
            kind,
            slice: [slice_start, slice_end].try_into().unwrap(),
        },
    ))
}

#[allow(clippy::cast_possible_wrap)]
fn pair_to_slice(first: u16, last: u16) -> SliceInfoElem {
    SliceInfoElem::Slice {
        start: first as isize,
        end: Some(last as isize + 1),
        step: 1,
    }
}

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
