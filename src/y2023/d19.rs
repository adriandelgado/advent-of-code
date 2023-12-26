use std::collections::{HashMap, VecDeque};

use winnow::{
    ascii::{alpha1, dec_uint},
    combinator::{alt, delimited, preceded, separated, separated_pair},
    token::one_of,
    PResult, Parser,
};

pub fn part1(input: &str) -> u64 {
    let (workflows, ratings) = input.split_once("\n\n").unwrap();
    let workflows: HashMap<&str, Vec<Rule>> = workflows
        .lines()
        .map(|line| parse_workflow.parse(line).unwrap())
        .collect();

    ratings
        .lines()
        .map(|line| parse_rating.parse(line).unwrap())
        .filter(|&rating| {
            let mut current_workflow = "in";
            loop {
                for Rule { condition, action } in workflows[current_workflow].iter() {
                    if condition.is_satisfied(rating) {
                        match action {
                            Action::Accept => return true,
                            Action::Reject => return false,
                            Action::SendTo(wf) => current_workflow = wf,
                        }
                        break;
                    }
                }
            }
        })
        .map(Rating::total)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let (workflows, _) = input.split_once("\n\n").unwrap();
    let workflows: HashMap<&str, Vec<Rule>> = workflows
        .lines()
        .map(|line| parse_workflow.parse(line).unwrap())
        .collect();

    let mut queue = VecDeque::from([("in", [(1, 4001), (1, 4001), (1, 4001), (1, 4001)])]);

    while let Some((wf, ranges)) = queue.pop_front() {
        for Rule {
            condition,
            action: _,
        } in workflows[wf].iter()
        {
            match condition {
                Condition::Gt(part, other) => {
                    let (_true_ranges, _false_ranges) = part.split_gt(*other, ranges);
                }
                Condition::Lt(_part, _other) => todo!(),
                Condition::Default => todo!(),
            }
        }
    }

    todo!()
}

#[derive(Debug, Clone, Copy)]
enum Condition {
    Gt(Part, u16),
    Lt(Part, u16),
    Default,
}

impl Condition {
    fn is_satisfied(self, rating: Rating) -> bool {
        match self {
            Condition::Gt(part, other) => rating.get(part) > other,
            Condition::Lt(part, other) => rating.get(part) < other,
            Condition::Default => true,
        }
    }
}

#[derive(Debug, Clone)]
enum Action<'a> {
    Accept,
    Reject,
    SendTo(&'a str),
}

#[derive(Debug, Clone)]
struct Rule<'a> {
    condition: Condition,
    action: Action<'a>,
}

#[derive(Debug, Clone, Copy)]
struct Rating {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
}

#[derive(Debug, Clone, Copy)]
enum Part {
    X,
    M,
    A,
    S,
}

impl Part {
    fn split_gt(self, _other: u16, _ranges: [(u16, u16); 4]) -> ([(u16, u16); 4], [(u16, u16); 4]) {
        todo!()
    }
}

impl Rating {
    fn total(self) -> u64 {
        (self.x + self.m + self.a + self.s) as u64
    }

    fn get(self, part: Part) -> u16 {
        match part {
            Part::X => self.x,
            Part::M => self.m,
            Part::A => self.a,
            Part::S => self.s,
        }
    }
}

fn parse_workflow<'a>(input: &mut &'a str) -> PResult<(&'a str, Vec<Rule<'a>>)> {
    (alpha1, delimited('{', separated(1.., parse_rule, ','), '}')).parse_next(input)
}

fn parse_rule<'a>(input: &mut &'a str) -> PResult<Rule<'a>> {
    alt((
        separated_pair(parse_condition, ':', parse_action).map(|(c, a)| Rule {
            condition: c,
            action: a,
        }),
        parse_action.map(|action| Rule {
            condition: Condition::Default,
            action,
        }),
    ))
    .parse_next(input)
}

fn parse_condition(input: &mut &str) -> PResult<Condition> {
    alt((
        separated_pair(parse_part, '>', dec_uint).map(|(f, n)| Condition::Gt(f, n)),
        separated_pair(parse_part, '<', dec_uint).map(|(f, n)| Condition::Lt(f, n)),
    ))
    .parse_next(input)
}

fn parse_part(input: &mut &str) -> PResult<Part> {
    alt((
        'x'.value(Part::X),
        'm'.value(Part::M),
        'a'.value(Part::A),
        's'.value(Part::S),
    ))
    .parse_next(input)
}

fn parse_action<'a>(input: &mut &'a str) -> PResult<Action<'a>> {
    alt((
        'A'.value(Action::Accept),
        'R'.value(Action::Reject),
        alpha1.map(Action::SendTo),
    ))
    .parse_next(input)
}

fn parse_rating(input: &mut &str) -> PResult<Rating> {
    delimited(
        '{',
        (
            part_rating,
            ',',
            part_rating,
            ',',
            part_rating,
            ',',
            part_rating,
        ),
        '}',
    )
    .map(|(x, _, m, _, a, _, s)| Rating { x, m, a, s })
    .parse_next(input)
}

fn part_rating(input: &mut &str) -> PResult<u16> {
    preceded((one_of(['x', 'm', 'a', 's']), '='), dec_uint).parse_next(input)
}
