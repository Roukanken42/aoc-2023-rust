use std::collections::HashSet;
use std::str::FromStr;

use nom::bytes::complete::{is_a, tag};
use nom::character::complete::{digit1, multispace0, multispace1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

use advent_of_code::utils::{parse_input_by_lines, Parsable};

advent_of_code::solution!(4);

struct LotteryCard {
    id: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Parsable for LotteryCard {
    fn parse(input: &str) -> IResult<&str, Self> {
        let mut parse_numbers = separated_list1(multispace1, map_res(digit1, u32::from_str));

        let (input, _) = tuple((tag("Card"), multispace0))(input)?;
        let (input, id) = map_res(digit1, u32::from_str)(input)?;
        let (input, _) = is_a(": ")(input)?;

        let (input, winning_numbers) = parse_numbers(input)?;
        let (input, _) = is_a(" | ")(input)?;
        let (input, numbers) = parse_numbers(input)?;

        Ok((
            input,
            LotteryCard {
                id,
                winning_numbers,
                numbers,
            },
        ))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, cards) = parse_input_by_lines(LotteryCard::parse)(input).unwrap();

    Some(
        cards
            .iter()
            .map(|card| {
                let winning: HashSet<u32> = card.winning_numbers.iter().copied().collect();
                card.numbers
                    .iter()
                    .filter(|&number| winning.contains(number))
                    .count() as u32
            })
            .map(|count| if count > 0 { 2u32.pow(count - 1) } else { 0 })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
