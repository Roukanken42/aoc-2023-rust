#![feature(if_let_guard)]

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::multi::many1;
use nom::IResult;
use std::collections::HashMap;

use advent_of_code::utils::location::{direction, Access2d, Location};
use advent_of_code::utils::{parse_input_by_lines, Parsable};

advent_of_code::solution!(14);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    RoundedRock,
    CubeShapedRock,
}

impl Parsable<'_> for Tile {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Self::Empty, tag(".")),
            value(Self::RoundedRock, tag("O")),
            value(Self::CubeShapedRock, tag("#")),
        ))(input)
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    parse_input_by_lines(many1(Tile::parse))(input)
}

fn find_lowest_position<T>(data: &Vec<Vec<T>>, gravity: Location<i32>) -> Vec<Location<i32>> {
    data.iter_2d_keys()
        .filter(|&location| data.get_2d(location.try_map(TryFrom::try_from).unwrap() + gravity).is_none())
        .filter_map(|location| location.try_map(TryFrom::try_from).ok())
        .collect_vec()
}

fn tilt(data: &mut Vec<Vec<Tile>>, gravity: Location<i32>) {
    let lowest_points = find_lowest_position(data, gravity);

    for lowest_point in lowest_points {
        let mut first_empty = None;

        let column = lowest_point
            .iter_ray(-gravity)
            .map(|loc| Some((loc, *data.get_2d(loc)?)))
            .take_while(|elem| elem.is_some())
            .flatten()
            .collect_vec();

        for (location, current) in column {
            match current {
                Tile::Empty if first_empty.is_none() => first_empty = Some(location),
                Tile::RoundedRock if let Some(target) = first_empty => {
                    data.set_2d(target, Tile::RoundedRock);
                    data.set_2d(location, Tile::Empty);
                    first_empty = Some(target - gravity);
                }
                Tile::CubeShapedRock => {
                    first_empty = None;
                }
                _ => {}
            }
        }
    }
}

fn calc_load(data: Vec<Vec<Tile>>) -> u32 {
    data.iter()
        .rev()
        .enumerate()
        .map(|(y, row)| row.iter().filter(|&&tile| tile == Tile::RoundedRock).count() as u32 * (y as u32 + 1))
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, mut data) = parse(input).unwrap();
    tilt(&mut data, direction::UP);
    Some(calc_load(data))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, mut data) = parse(input).unwrap();

    let mut first_encountered = HashMap::new();
    let mut cycle_detected = None;

    for i in 0..1_000_000_000u32 {
        if let Some(last) = first_encountered.insert(data.clone(), i) {
            cycle_detected = Some(last..i);
            break;
        }

        tilt(&mut data, direction::UP);
        tilt(&mut data, direction::LEFT);
        tilt(&mut data, direction::DOWN);
        tilt(&mut data, direction::RIGHT);
    }

    if let Some(cycle) = cycle_detected {
        let end_cycle = (1_000_000_000 - cycle.start) % cycle.len() as u32 + cycle.start;

        let (end, _) = first_encountered.iter().find(|(_, &value)| value == end_cycle).unwrap();
        data = end.clone();
    }

    Some(calc_load(data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
