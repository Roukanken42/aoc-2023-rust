use itertools::Itertools;
use std::fmt::{Debug, Formatter};

use nom::branch::alt;
use nom::character::complete::{char, space1};
use nom::combinator::value;
use nom::multi::{many1, separated_list1};
use nom::IResult;

use advent_of_code::utils::{parse_input_by_lines, Parsable};

advent_of_code::solution!(12);

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
enum Spring {
    Working,
    Broken,
    Unknown,
}

impl Debug for Spring {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Spring::Working => ".",
            Spring::Broken => "#",
            Spring::Unknown => "?",
        })
    }
}

struct Day12 {
    springs: Vec<Spring>,
    broken_lengths: Vec<usize>,
}

impl Parsable<'_> for Day12 {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, springs) = many1(alt((
            value(Spring::Working, char('.')),
            value(Spring::Broken, char('#')),
            value(Spring::Unknown, char('?')),
        )))(input)?;

        let (input, _) = space1(input)?;
        let (input, broken_lengths) = separated_list1(char(','), usize::parse)(input)?;

        Ok((
            input,
            Self {
                springs,
                broken_lengths,
            },
        ))
    }
}

fn try_a_lot(springs: &[Spring], sizes: &[usize], ident: usize) -> usize {
    // the sizes don't fit in the springs
    if springs.len() + 1 < sizes.iter().sum::<usize>() + sizes.len() {
        return 0;
    }

    // we placed all the sizes and left no broken springs
    if sizes.is_empty() {
        let res = if !springs.iter().any(|&spring| spring == Spring::Broken) {
            1
        } else {
            0
        };
        return res;
    }

    // we have size to place, but no place to place it
    if springs.iter().all(|&spring| spring == Spring::Working) && !sizes.is_empty() {
        return 0;
    }

    let next_size = sizes[0];

    let res = (0..springs.len())
        .take_while_inclusive(|&index| springs[index] != Spring::Broken)
        .map(|index| {
            if index + next_size > springs.len() {
                return 0;
            }

            let can_be_placed = !springs[index..index + next_size]
                .iter()
                .any(|spring| *spring == Spring::Working)
                && springs.get(index + next_size) != Some(&Spring::Broken);

            if !can_be_placed {
                return 0;
            }

            if springs.len() == index + next_size {
                if sizes.len() == 1 {
                    1
                } else {
                    0
                }
            } else {
                try_a_lot(&springs[index + next_size + 1..], &sizes[1..], ident + 1)
            }
        })
        .sum();

    res
}

impl Day12 {
    fn calculate_possible_arrangements(&self) -> usize {
        println!();
        try_a_lot(&self.springs, &self.broken_lengths, 0)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, data) = parse_input_by_lines(Day12::parse)(input).unwrap();

    Some(
        data.iter()
            .map(|day| day.calculate_possible_arrangements())
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
