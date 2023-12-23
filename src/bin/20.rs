use advent_of_code::utils::{parse_input_by_lines, Parsable};
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::combinator::{success, value};
use nom::multi::separated_list1;
use nom::IResult;
use std::collections::{HashMap, VecDeque};
advent_of_code::solution!(20);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SwitchVariant {
    FlipFlop,
    Conjunction,
    Broadcast,
}

impl Parsable<'_> for SwitchVariant {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Self::FlipFlop, tag("%")),
            value(Self::Conjunction, tag("&")),
            success(Self::Broadcast),
        ))(input)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Rule {
    name: String,
    variant: SwitchVariant,
    targets: Vec<String>,
}

impl Parsable<'_> for Rule {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, variant) = SwitchVariant::parse(input)?;
        let (input, name) = alpha1(input)?;
        let (input, _) = tag(" -> ")(input)?;

        let (input, targets) = separated_list1(tag(", "), alpha1)(input)?;

        Ok((
            input,
            Self {
                name: name.to_string(),
                variant,
                targets: targets.into_iter().map(String::from).collect(),
            },
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Broadcast,
}

fn parse(input: &str) -> IResult<&str, Vec<Rule>> {
    parse_input_by_lines(Rule::parse)(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, rules) = parse(input).unwrap();

    let rules = rules
        .into_iter()
        .map(|rule| (rule.name.clone(), rule))
        .collect::<HashMap<_, _>>();

    let inputs = rules
        .iter()
        .flat_map(|(_, rule)| rule.targets.iter().map(|target| (target.clone(), &rule.name)))
        .into_group_map();

    let mut state = rules
        .iter()
        .map(|(name, rule)| {
            (
                name.clone(),
                match rule.variant {
                    SwitchVariant::FlipFlop => State::FlipFlop(false),
                    SwitchVariant::Conjunction => State::Conjunction(
                        inputs
                            .get(&rule.name)
                            .unwrap()
                            .iter()
                            .map(|&name| (name.clone(), false))
                            .collect(),
                    ),
                    SwitchVariant::Broadcast => State::Broadcast,
                },
            )
        })
        .collect::<HashMap<_, _>>();

    let mut signal_queue = VecDeque::new();
    let mut lows = 0u64;
    let mut highs = 0u64;

    for _ in 0..1000 {
        signal_queue.push_back(("broadcaster", "button", false));

        while let Some((current, source, signal)) = signal_queue.pop_front() {
            *if signal { &mut highs } else { &mut lows } += 1;

            let Some(rule) = rules.get(current) else {
                continue;
            };
            let Some(state) = state.get_mut(current) else {
                continue;
            };

            let emit = match state {
                State::FlipFlop(state) => {
                    if signal {
                        continue;
                    }
                    *state = !*state;
                    *state
                }
                State::Conjunction(state) => {
                    let Some(con_state) = state.get_mut(source) else {
                        continue;
                    };
                    *con_state = signal;

                    !state.iter().all(|(_, &state)| state)
                }
                State::Broadcast => signal,
            };

            for target in &rule.targets {
                signal_queue.push_back((target, current, emit));
            }
        }
    }

    Some(lows * highs)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
