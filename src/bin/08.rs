use std::collections::HashMap;
use std::hash::Hash;
use std::iter::{zip, Cycle, Enumerate};
use std::slice::Iter;

use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{char, line_ending};
use nom::combinator::value;
use nom::multi::{count, many1, separated_list1};
use nom::sequence::{delimited, separated_pair, terminated};
use nom::IResult;
use num::integer::lcm;

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
        let parse_direction = alt((value(Direction::Left, char('L')), value(Direction::Right, char('R'))));

        let (input, directions) = terminated(many1(parse_direction), count(line_ending, 2))(input)?;

        let (input, res) = separated_list1(
            line_ending,
            separated_pair(
                take(3usize),
                tag(" = "),
                delimited(char('('), separated_pair(take(3usize), tag(", "), take(3usize)), char(')')),
            ),
        )(input)?;

        let graph = HashMap::from_iter(res);

        Ok((input, Self { directions, graph }))
    }
}

impl<'a> Day8<'a> {
    fn walk_trough_desert(&'a self, start: &'a str) -> WalkThroughDesert<'a> {
        WalkThroughDesert {
            graph: &self.graph,
            current: start,
            directions: self.directions.iter().enumerate().cycle(),
        }
    }
}

struct WalkThroughDesert<'a> {
    graph: &'a HashMap<&'a str, (&'a str, &'a str)>,
    current: &'a str,
    directions: Cycle<Enumerate<Iter<'a, Direction>>>,
}

impl<'a> Iterator for WalkThroughDesert<'a> {
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        let (left, right) = self.graph.get(self.current)?;
        let (cycle_pos, direction) = self.directions.next()?;

        self.current = match direction {
            Direction::Left => left,
            Direction::Right => right,
        };

        Some((cycle_pos, current))
    }
}

fn find_cycle<T: Hash + Eq>(input: &mut dyn Iterator<Item = T>) -> Option<(usize, usize)> {
    let mut visited = HashMap::new();

    for (pos, item) in input.enumerate() {
        if let Some(prev_pos) = visited.insert(item, pos) {
            return Some((prev_pos, pos));
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, day) = parse_input(Day8::parse)(input).unwrap();

    Some(day.walk_trough_desert("AAA").take_while(|(_, node)| node != &"ZZZ").count() as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, day) = parse_input(Day8::parse)(input).unwrap();

    let starts = day.graph.keys().filter(|key| key.ends_with('A')).collect::<Vec<_>>();

    let cycles = starts
        .iter()
        .map(|start| find_cycle(&mut day.walk_trough_desert(start)))
        .collect::<Vec<_>>();

    let winning_positions = zip(starts.iter(), cycles.iter())
        .filter_map(|(start, cycle)| cycle.map(|(start_pos, end_pos)| (start, (start_pos, end_pos))))
        .map(|(start, (_, length))| {
            day.walk_trough_desert(start)
                .enumerate()
                .take(length)
                .filter(|(_, (_, node))| node.ends_with('Z'))
                .map(|(pos, _)| pos)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let cycle_lengths = cycles
        .iter()
        .map(|cycle| cycle.map(|(start, end)| end - start))
        .collect::<Vec<_>>();

    // only solve the problem for one winning position on cycle, and ensure it's always at start
    let is_solvable = zip(cycle_lengths.iter(), winning_positions.iter()).all(|(cycle_length, winning_position)| {
        cycle_length.is_some() && winning_position.len() == 1 && winning_position[0] == cycle_length.unwrap()
    });

    if !is_solvable {
        return None;
    }

    cycle_lengths.iter().map(|length| length.unwrap() as u64).reduce(lcm)
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
