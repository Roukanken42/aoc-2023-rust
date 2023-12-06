use advent_of_code::utils::{parse_input, Parsable};
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending, space0, space1};
use nom::combinator::{map_res, recognize};
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::IResult;
use num::integer::Roots;
use std::str::FromStr;

advent_of_code::solution!(6);

fn parse_part1(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)> {
    let (input, _) = tag("Time:")(input)?;
    let (input, times) =
        delimited(space0, separated_list1(space1, u64::parse), line_ending)(input)?;

    let (input, _) = tag("Distance:")(input)?;
    let (input, distances) =
        delimited(space0, separated_list1(space1, u64::parse), line_ending)(input)?;

    Ok((input, (times, distances)))
}

fn solve(time: u64, distance: u64) -> u64 {
    let discriminant = ((time * time) as f64 - 4f64 * distance as f64).sqrt();
    let min = ((time as f64 - discriminant) / 2.0).max(0.0);
    let max = (time as f64 + discriminant) / 2.0;

    (max.ceil() - min.floor() - 1.0) as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, (times, distances)) = parse_input(parse_part1)(input).unwrap();

    Some(
        times
            .iter()
            .zip(distances.iter())
            .map(|(&time, &distance)| solve(time, distance))
            .product(),
    )
}

fn parse_part2(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, _) = tag("Time:")(input)?;
    let mut delimited1 = delimited(
        space0,
        map_res(
            recognize(separated_list1(space1, digit1)),
            |digits: &str| u64::from_str(&digits.replace(" ", "")),
        ),
        line_ending,
    );

    let (input, times) = delimited1(input)?;

    let (input, _) = tag("Distance:")(input)?;
    let (input, distances) = delimited1(input)?;

    Ok((input, (times, distances)))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, (time, distance)) = parse_input(parse_part2)(input).unwrap();
    Some(solve(time, distance) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
