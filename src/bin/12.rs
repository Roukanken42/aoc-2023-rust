use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use itertools::Itertools;
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

fn try_a_lot(springs: &[Spring], sizes: &[usize], ident: usize) -> u64 {
    if springs.len() + 1 < sizes.iter().sum::<usize>() + sizes.len() {
        return 0;
    }

    if sizes.is_empty() {
        return if !springs.iter().any(|&spring| spring == Spring::Broken) {
            1
        } else {
            0
        };
    }

    if springs.iter().all(|&spring| spring == Spring::Working) && !sizes.is_empty() {
        return 0;
    }

    let next_size = sizes[0];

    (0..springs.len())
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
        .sum()
}

impl Day12 {
    fn calculate_possible_arrangements(&self) -> u64 {
        try_a_lot(&self.springs, &self.broken_lengths, 0)
    }

    fn calculate_possible_arrangements_dynamic(&self) -> u64 {
        let mut state = HashMap::<(usize, usize, usize), u64>::new();

        state.insert((0, 0, 0), 1);

        for pos in 0..self.springs.len() {
            let spring = self.springs[pos];

            for groups_pos in 0..=self.broken_lengths.len() {
                let group = self.broken_lengths.get(groups_pos).copied();

                for current_len in 0..=group.unwrap_or(0) {
                    let current = state
                        .get(&(pos, groups_pos, current_len))
                        .copied()
                        .unwrap_or(0);

                    if current == 0 {
                        continue;
                    }

                    if spring != Spring::Broken {
                        let dont_have_active_group = current_len == 0;
                        let can_end_active_group = if let Some(group) = group {
                            current_len == group
                        } else {
                            false
                        };

                        if dont_have_active_group {
                            *state.entry((pos + 1, groups_pos, 0)).or_insert(0) += current;
                        }

                        if can_end_active_group {
                            *state.entry((pos + 1, groups_pos + 1, 0)).or_insert(0) += current;
                        }
                    }

                    if spring != Spring::Working {
                        *state
                            .entry((pos + 1, groups_pos, current_len + 1))
                            .or_insert(0) += current;
                    }
                }
            }
        }

        state
            .get(&(self.springs.len(), self.broken_lengths.len(), 0))
            .copied()
            .unwrap_or(0)
    }

    fn unfold(self, times: usize) -> Day12 {
        let springs = self
            .springs
            .iter()
            .chain([Spring::Unknown].iter())
            .cycle()
            .take(self.springs.len() * times + times - 1)
            .chain([Spring::Working].iter())
            .copied()
            .collect_vec();

        let broken_lengths = self
            .broken_lengths
            .iter()
            .cycle()
            .take(self.broken_lengths.len() * times)
            .copied()
            .collect_vec();

        Day12 {
            springs,
            broken_lengths,
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, data) = parse_input_by_lines(Day12::parse)(input).unwrap();

    Some(
        data.iter()
            .map(|day| day.calculate_possible_arrangements())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, data) = parse_input_by_lines(Day12::parse)(input).unwrap();

    Some(
        data.into_iter()
            .map(|day| day.unfold(5))
            .map(|day| day.calculate_possible_arrangements_dynamic())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
