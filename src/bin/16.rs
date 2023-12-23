use std::collections::HashSet;

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::multi::many1;
use nom::IResult;
use rayon::prelude::*;

use advent_of_code::utils::location::{direction, Access2d, Location};
use advent_of_code::utils::{parse_input_by_lines, Parsable};

advent_of_code::solution!(16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    DownwardsMirror,
    UpwardsMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl Parsable<'_> for Tile {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Tile::Empty, tag(".")),
            value(Tile::DownwardsMirror, tag("\\")),
            value(Tile::UpwardsMirror, tag("/")),
            value(Tile::VerticalSplitter, tag("|")),
            value(Tile::HorizontalSplitter, tag("-")),
        ))(input)
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    parse_input_by_lines(many1(Tile::parse))(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Ray {
    location: Location<i32>,
    direction: Location<i32>,
}

impl Ray {
    fn new(location: Location<i32>, direction: Location<i32>) -> Self {
        Self { location, direction }
    }

    fn go_straight(&self) -> Self {
        Self::new(self.location + self.direction, self.direction)
    }

    fn go_right(&self) -> Ray {
        let direction_right = self.direction.rotate_90_cw();
        Ray::new(self.location + direction_right, direction_right)
    }

    fn go_left(&self) -> Ray {
        let direction_left = self.direction.rotate_90_ccw();
        Ray::new(self.location + direction_left, direction_left)
    }

    fn step(&self, tile: &Tile) -> (Self, Option<Self>) {
        let direction_is_horizontal = self.direction.x != 0;

        match (tile, direction_is_horizontal) {
            (Tile::Empty, _) => (self.go_straight(), None),
            (Tile::VerticalSplitter, false) => (self.go_straight(), None),
            (Tile::VerticalSplitter, true) => (self.go_left(), Some(self.go_right())),
            (Tile::HorizontalSplitter, true) => (self.go_straight(), None),
            (Tile::HorizontalSplitter, false) => (self.go_left(), Some(self.go_right())),
            (Tile::DownwardsMirror, true) => (self.go_right(), None),
            (Tile::DownwardsMirror, false) => (self.go_left(), None),
            (Tile::UpwardsMirror, true) => (self.go_left(), None),
            (Tile::UpwardsMirror, false) => (self.go_right(), None),
        }
    }
}

fn brute_raytrace(data: &Vec<Vec<Tile>>, starting_ray: Ray) -> usize {
    let mut processed_rays = HashSet::from([]);
    let mut unprocessed_rays = vec![starting_ray];

    while let Some(ray) = unprocessed_rays.pop() {
        if let Some(tile) = data.get_2d(ray.location) {
            if !processed_rays.insert(ray) {
                continue;
            }

            let (a, b) = ray.step(tile);
            unprocessed_rays.push(a);
            if let Some(b) = b {
                unprocessed_rays.push(b);
            }
        }
    }

    processed_rays.iter().map(|ray| ray.location).unique().count()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, data) = parse(input).unwrap();

    Some(brute_raytrace(&data, Ray::new(Location::new(0, 0), direction::RIGHT)))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, data) = parse(input).unwrap();

    let last_y = data.len() as i32;
    let last_x = data[0].len() as i32;
    let starting_rays = (0..last_x)
        .map(|x| Ray::new(Location::new(x, 0), direction::DOWN))
        .chain((0..last_x).map(|x| Ray::new(Location::new(x, last_y - 1), direction::UP)))
        .chain((0..last_y).map(|y| Ray::new(Location::new(0, y), direction::RIGHT)))
        .chain((0..last_y).map(|y| Ray::new(Location::new(last_x - 1, y), direction::LEFT)))
        .collect_vec();

    starting_rays.into_par_iter().map(|ray| brute_raytrace(&data, ray)).max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
