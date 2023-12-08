use std::collections::HashMap;
use std::iter::Cycle;
use std::slice::Iter;

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{char, line_ending};
use nom::combinator::value;
use nom::multi::{count, many1, separated_list1};
use nom::sequence::{delimited, separated_pair, terminated};
use nom::IResult;

use advent_of_code::utils::{parse_input, Parsable};

advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

struct Day8<'a> {
    directions: Vec<Direction>,
    graph: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> Parsable<'a> for Day8<'a> {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        let parse_direction = alt((
            value(Direction::Left, char('L')),
            value(Direction::Right, char('R')),
        ));

        let (input, directions) = terminated(many1(parse_direction), count(line_ending, 2))(input)?;

        let (input, res) = separated_list1(
            line_ending,
            separated_pair(
                take(3usize),
                tag(" = "),
                delimited(
                    char('('),
                    separated_pair(take(3usize), tag(", "), take(3usize)),
                    char(')'),
                ),
            ),
        )(input)?;

        let graph = HashMap::from_iter(res);

        Ok((input, Self { directions, graph }))
    }
}

impl<'a> Day8<'a> {
    fn walk_trough_desert<'b>(&'b self, start: &'b str) -> impl Iterator<Item = &str> {
        WalkThroughDesert {
            graph: &self.graph,
            current: start,
            directions: self.directions.iter().cycle(),
        }
    }
}

struct WalkThroughDesert<'a> {
    graph: &'a HashMap<&'a str, (&'a str, &'a str)>,
    current: &'a str,
    directions: Cycle<Iter<'a, Direction>>,
}

impl<'a> Iterator for WalkThroughDesert<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let (left, right) = self.graph.get(self.current)?;
        self.current = match self.directions.next()? {
            Direction::Left => left,
            Direction::Right => right,
        };

        Some(self.current)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, day) = parse_input(Day8::parse)(input).unwrap();

    Some(
        day.walk_trough_desert("AAA")
            .take_while(|&node| node != "ZZZ")
            .count() as u32
            + 1,
    )
}

struct Multizip<T>(Vec<T>);

impl<T> Iterator for Multizip<T>
where
    T: Iterator,
{
    type Item = Vec<T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter_mut().map(Iterator::next).collect()
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, day) = parse_input(Day8::parse)(input).unwrap();

    let starts = day
        .graph
        .keys()
        .filter(|key| key.ends_with("A"))
        .collect::<Vec<_>>();

    Some(
        Multizip(
            starts
                .iter()
                .map(|start| day.walk_trough_desert(start))
                .collect::<Vec<_>>(),
        )
        .take_while(|nodes| !nodes.iter().all(|node| node.ends_with("Z")))
        .count() as u32
            + 1,
    )
}

pub fn part_two_brute(input: &str) -> Option<u32> {
    let (_, day) = parse_input(Day8::parse)(input).unwrap();

    let starts = day
        .graph
        .keys()
        .filter(|key| key.ends_with("A"))
        .collect::<Vec<_>>();

    Some(
        Multizip(
            starts
                .iter()
                .map(|start| day.walk_trough_desert(start))
                .collect::<Vec<_>>(),
        )
        .take_while(|nodes| !nodes.iter().all(|node| node.ends_with("Z")))
        .count() as u32
            + 1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
