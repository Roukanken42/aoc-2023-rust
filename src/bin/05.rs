use nom::bytes::complete::{tag, take_till1};
use nom::character::complete::{line_ending, space1};
use nom::multi::{count, separated_list1};
use nom::sequence::tuple;
use nom::IResult;

use advent_of_code::utils::parse_input;
use advent_of_code::utils::Parsable;

advent_of_code::solution!(5);

struct MappingRange {
    start_from: u64,
    start_to: u64,
    size: u64,
}

struct GardenMapping {
    from: String,
    to: String,
    ranges: Vec<MappingRange>,
}

struct Day5 {
    seeds: Vec<u64>,
    mappings: Vec<GardenMapping>,
}

impl Parsable for MappingRange {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (start_to, _, start_from, _, size)) =
            tuple((u64::parse, space1, u64::parse, space1, u64::parse))(input)?;

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

impl Parsable for GardenMapping {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, from) = take_till1(|c| c == '-')(input)?;
        let (input, _) = tag("-to-")(input)?;
        let (input, to) = take_till1(|c| c == ' ')(input)?;
        let (input, _) = tag(" map:")(input)?;
        let (input, _) = line_ending(input)?;

        let (input, ranges) = separated_list1(line_ending, MappingRange::parse)(input)?;

        Ok((
            input,
            GardenMapping {
                from: from.to_string(),
                to: to.to_string(),
                ranges,
            },
        ))
    }
}

impl Parsable for Day5 {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("seeds: ")(input)?;
        let (input, seeds) = separated_list1(space1, u64::parse)(input)?;
        let (input, _) = count(line_ending, 2)(input)?;
        let (input, mappings) =
            separated_list1(count(line_ending, 2), GardenMapping::parse)(input)?;

        Ok((input, Day5 { seeds, mappings }))
    }
}

impl MappingRange {
    fn map(&self, input: u64) -> Option<u64> {
        if input < self.start_from || input > self.start_from + self.size {
            return None;
        }

        let offset = input - self.start_from;
        Some(self.start_to + offset)
    }
}

impl GardenMapping {
    fn map(&self, input: u64) -> u64 {
        self.ranges
            .iter()
            .find_map(|range| range.map(input))
            .unwrap_or(input)
    }
}

impl Day5 {
    fn map(&self, input: u64, from: &str, to: &str) -> u64 {
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
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, data) = parse_input(Day5::parse)(input).unwrap();

    Some(
        data.seeds
            .iter()
            .map(|&seed| data.map(seed, "seed", "location"))
            .min()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
