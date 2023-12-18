use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{char, space1};
use nom::combinator::{map_res, value};
use nom::sequence::{delimited, pair};
use nom::IResult;

use advent_of_code::utils::location::{direction, Location};
use advent_of_code::utils::{parse_input_by_lines, Parsable};

advent_of_code::solution!(18);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_location(self) -> Location<i32> {
        match self {
            Direction::Up => direction::UP,
            Direction::Down => direction::DOWN,
            Direction::Left => direction::LEFT,
            Direction::Right => direction::RIGHT,
        }
    }
}

impl Parsable<'_> for Direction {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Direction::Up, char('U')),
            value(Direction::Down, char('D')),
            value(Direction::Left, char('L')),
            value(Direction::Right, char('R')),
        ))(input)
    }
}

struct DigPlan {
    direction: Direction,
    length: i32,
    decoded_direction: Direction,
    decoded_length: i64,
}

impl Parsable<'_> for DigPlan {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, direction) = Direction::parse(input)?;
        let (input, _) = space1(input)?;
        let (input, length) = i32::parse(input)?;
        let (input, _) = space1(input)?;

        let parse_decoded_length = map_res(take(5usize), |x| i64::from_str_radix(x, 16));
        let parse_decoded_direction = alt((
            value(Direction::Right, char('0')),
            value(Direction::Down, char('1')),
            value(Direction::Left, char('2')),
            value(Direction::Up, char('3')),
        ));

        let (input, (decoded_length, decoded_direction)) = delimited(
            tag("(#"),
            pair(parse_decoded_length, parse_decoded_direction),
            char(')'),
        )(input)?;

        Ok((
            input,
            Self {
                direction,
                length,
                decoded_direction,
                decoded_length,
            },
        ))
    }
}

fn parse(input: &str) -> IResult<&str, Vec<DigPlan>> {
    parse_input_by_lines(DigPlan::parse)(input)
}

pub fn part_one(input: &str) -> Option<i32> {
    let (_, data) = parse(input).unwrap();

    let mut current = Location::new(0, 0);
    let mut edge_length = 0;
    let mut area = 0;

    for plan in data {
        let direction = plan.direction.to_location();
        let length = plan.length;

        edge_length += length;
        current = current + direction * length;
        area += current.y * direction.x * length;
    }

    Some(area.abs() + edge_length / 2 + 1)
}

pub fn part_two(input: &str) -> Option<i64> {
    let (_, data) = parse(input).unwrap();

    let mut current = Location::new(0, 0);
    let mut edge_length = 0;
    let mut area = 0;

    for plan in data {
        let direction = plan.decoded_direction.to_location().map(From::from);
        let length = plan.decoded_length;

        edge_length += length;
        current = current + direction * length;
        area += current.y * direction.x * length;
    }

    Some(area.abs() + edge_length / 2 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
