use itertools::Itertools;
use std::collections::HashMap;
use std::ops::RangeBounds;

use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::sequence::{preceded, separated_pair, tuple};
use nom::IResult;

use advent_of_code::utils::location3d::Location3d;
use advent_of_code::utils::{parse_input_by_lines, Parsable};

advent_of_code::solution!(24);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Hailstone {
    position: Location3d<f64>,
    velocity: Location3d<f64>,
}

impl Parsable<'_> for Hailstone {
    fn parse(input: &str) -> IResult<&str, Self> {
        let parse_num = |input| preceded(space0, i64::parse)(input);
        let parse_coords = |input| tuple((parse_num, tag(", "), parse_num, tag(", "), parse_num))(input);

        let (input, (coords1, coords2)) = separated_pair(parse_coords, tag(" @ "), parse_coords)(input)?;
        let (x, _, y, _, z) = coords1;
        let (vx, _, vy, _, vz) = coords2;

        Ok((
            input,
            Self {
                position: Location3d::new(x, y, z).map(|a| a as f64),
                velocity: Location3d::new(vx, vy, vz).map(|a| a as f64),
            },
        ))
    }
}

fn find_intersections_2d(hailstones: &[Hailstone], is_valid: impl Fn(&Location3d<f64>) -> bool) -> usize {
    let mut intersection_count = 0usize;

    for (i, hailstone1) in hailstones.iter().enumerate() {
        for hailstone2 in hailstones.iter().skip(i + 1) {
            let diff = hailstone2.position - hailstone1.position;
            let intersection_time_2 = (hailstone1.velocity.y * diff.x - hailstone1.velocity.x * diff.y)
                / (hailstone1.velocity.x * hailstone2.velocity.y - hailstone1.velocity.y * hailstone2.velocity.x);

            let intersection_time_1 = (diff.x + hailstone2.velocity.x * intersection_time_2) / hailstone1.velocity.x;
            let intersection = hailstone2.position + hailstone2.velocity * intersection_time_2;

            if intersection_time_1 < 0.0 || intersection_time_2 < 0.0 {
                continue;
            }

            if is_valid(&intersection) {
                intersection_count += 1;
            }
        }
    }

    intersection_count
}

fn parse(input: &str) -> IResult<&str, Vec<Hailstone>> {
    parse_input_by_lines(Hailstone::parse)(input)
}

fn is_in_square_2d(range: impl RangeBounds<f64>) -> impl Fn(&Location3d<f64>) -> bool {
    move |pos| range.contains(&pos.x) && range.contains(&pos.y)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, hailstones) = parse(input).unwrap();

    Some(find_intersections_2d(
        &hailstones,
        is_in_square_2d(200_000_000_000_000.0..=400_000_000_000_000.0),
    ))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let (_, data) = parse(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(find_intersections_2d(&data, is_in_square_2d(7.0..=27.0)), 2);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
