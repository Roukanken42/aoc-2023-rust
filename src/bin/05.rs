use std::cmp::{max, min};

use itertools::Itertools;
use nom::bytes::complete::{tag, take_till1};
use nom::character::complete::{line_ending, space1};
use nom::multi::{count, separated_list1};
use nom::sequence::{tuple, Tuple};
use nom::IResult;

use advent_of_code::utils::parse_input;
use advent_of_code::utils::Parsable;

advent_of_code::solution!(5);

#[derive(Copy, Clone, Debug)]
struct Range {
    start: i64,
    end: i64,
}

struct MappingRange {
    start_from: i64,
    start_to: i64,
    size: i64,
}

struct GardenMapping {
    from: String,
    to: String,
    ranges: Vec<MappingRange>,
}

struct Day5 {
    seeds: Vec<i64>,
    mappings: Vec<GardenMapping>,
}

impl<'a> Parsable<'a> for MappingRange {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (start_to, _, start_from, _, size)) =
            tuple((i64::parse, space1, i64::parse, space1, i64::parse))(input)?;

        Ok((
            input,
            MappingRange {
                start_from,
                start_to,
                size,
            },
        ))
    }
}

impl<'a> Parsable<'a> for GardenMapping {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, from) = take_till1(|c| c == '-')(input)?;
        let (input, _) = tag("-to-")(input)?;
        let (input, to) = take_till1(|c| c == ' ')(input)?;
        let (input, _) = tag(" map:")(input)?;
        let (input, _) = line_ending(input)?;

        let (input, ranges) = separated_list1(line_ending, MappingRange::parse)(input)?;

        Ok((input, GardenMapping::new(from, to, ranges)))
    }
}

impl<'a> Parsable<'a> for Day5 {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("seeds: ")(input)?;
        let (input, seeds) = separated_list1(space1, i64::parse)(input)?;
        let (input, _) = count(line_ending, 2)(input)?;
        let (input, mappings) =
            separated_list1(count(line_ending, 2), GardenMapping::parse)(input)?;

        Ok((input, Day5 { seeds, mappings }))
    }
}

impl MappingRange {
    fn map(&self, input: i64) -> Option<i64> {
        if input < self.start_from || input >= self.start_from + self.size {
            return None;
        }

        let offset = input - self.start_from;
        Some(self.start_to + offset)
    }

    fn overlaps(&self, input: &Range) -> bool {
        input.start <= self.start_from + self.size && input.end >= self.start_from
    }

    fn map_range(&self, input: &Range) -> Option<(Range, Range, Range)> {
        if !self.overlaps(input) {
            return None;
        }

        let overlap_start = max(self.start_from, input.start);
        let overlap_end = min(self.start_from + self.size - 1, input.end);
        let mapped_overlap = Range::new(self.map(overlap_start)?, self.map(overlap_end)?);

        let left = Range::new(input.start, overlap_start - 1);
        let right = Range::new(overlap_end + 1, input.end);

        Some((left, mapped_overlap, right))
    }
}

impl Range {
    fn new(start: i64, end: i64) -> Self {
        Range { start, end }
    }

    fn is_empty(&self) -> bool {
        self.start > self.end
    }
}

impl GardenMapping {
    fn new(from: &str, to: &str, mut ranges: Vec<MappingRange>) -> Self {
        ranges.sort_by_key(|range| range.start_from);

        GardenMapping {
            from: from.to_string(),
            to: to.to_string(),
            ranges,
        }
    }

    fn map(&self, input: i64) -> i64 {
        self.ranges
            .iter()
            .find_map(|range| range.map(input))
            .unwrap_or(input)
    }

    fn map_range(&self, input: &Range) -> Vec<Range> {
        let mut result = vec![];
        let mut remaining = input.clone();

        for range in &self.ranges {
            if let Some((left, mapped, right)) = range.map_range(&remaining) {
                if !left.is_empty() {
                    result.push(left);
                }

                result.push(mapped);
                remaining = right;
            }
        }

        if !remaining.is_empty() {
            result.push(remaining);
        }

        result
    }
}

impl Day5 {
    fn map(&self, input: i64, from: &str, to: &str) -> i64 {
        if from == to {
            return input;
        }

        let mapping = self
            .mappings
            .iter()
            .find(|mapping| mapping.from == from)
            .unwrap();

        self.map(mapping.map(input), &mapping.to, to)
    }

    fn map_ranges(&self, input: Vec<Range>, from: &str, to: &str) -> Vec<Range> {
        if from == to {
            return input;
        }

        let mapping = self
            .mappings
            .iter()
            .find(|mapping| mapping.from == from)
            .unwrap();

        let mapped = input
            .iter()
            .map(|range| mapping.map_range(range))
            .flatten()
            .collect();

        self.map_ranges(mapped, &mapping.to, to)
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let (_, data) = parse_input(Day5::parse)(input).unwrap();

    Some(
        data.seeds
            .iter()
            .map(|&seed| data.map(seed, "seed", "location"))
            .min()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let (_, data) = parse_input(Day5::parse)(input).unwrap();

    Some(
        data.seeds
            .iter()
            .tuples()
            .map(|(&start, &size)| Range::new(start, start + size - 1))
            .flat_map(|range| data.map_ranges(vec![range], "seed", "location"))
            .filter(|range| !range.is_empty())
            .map(|range| range.start)
            .min()
            .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_parses() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = parse_input(Day5::parse)(&input);

        assert_eq!(result.err(), None);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
