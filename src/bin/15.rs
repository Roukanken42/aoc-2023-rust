use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::combinator::{map, value};
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;

use advent_of_code::utils::{parse_input, Parsable};

advent_of_code::solution!(15);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Remove,
    Insert(u32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instruction {
    name: String,
    action: Action,
}

impl Parsable<'_> for Instruction {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, name) = take_while(|c| c != '-' && c != '=' && c != '\n')(input)?;
        let (input, action) = alt((
            value(Action::Remove, tag("-")),
            map(preceded(tag("="), u32::parse), |u| Action::Insert(u)),
        ))(input)?;

        Ok((
            input,
            Self {
                name: name.to_string(),
                action,
            },
        ))
    }
}

fn hash(s: &str) -> u32 {
    s.chars().fold(0, |acc, c| (acc + c as u32) * 17 % 256)
}

fn parse(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(","), take_while(|c| c != ',' && c != '\n'))(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, data) = parse_input(parse)(input).unwrap();
    Some(data.iter().map(|s| hash(s)).sum::<u32>())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HashmapEntry<T> {
    key: String,
    value: T,
}

impl<T> HashmapEntry<T> {
    fn new(key: String, value: T) -> Self {
        Self { key, value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hashmap<T> {
    buckets: Vec<Vec<HashmapEntry<T>>>,
}

impl<T: Clone> Hashmap<T> {
    fn new() -> Hashmap<T> {
        Self {
            buckets: vec![Vec::new(); 256],
        }
    }

    fn insert(&mut self, key: &str, value: T) {
        let hash = hash(key) as usize % self.buckets.len();

        if let Some(entry) = self.buckets[hash].iter_mut().find(|entry| entry.key == key) {
            entry.value = value;
        } else {
            self.buckets[hash].push(HashmapEntry::new(key.to_string(), value));
        }
    }

    fn remove(&mut self, key: &str) {
        let hash = hash(key) as usize % self.buckets.len();
        self.buckets[hash].retain(|entry| entry.key != key);
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, instructions) =
        parse_input(separated_list1(tag(","), Instruction::parse))(input).unwrap();

    let mut hashmap = Hashmap::new();

    for instruction in instructions {
        match instruction.action {
            Action::Remove => hashmap.remove(&instruction.name),
            Action::Insert(value) => hashmap.insert(&instruction.name, value),
        }
    }

    Some(
        hashmap
            .buckets
            .iter()
            .enumerate()
            .map(|(bucket_num, bucket)| {
                bucket
                    .iter()
                    .enumerate()
                    .map(|(slot, entry)| (bucket_num + 1) as u32 * (slot + 1) as u32 * entry.value)
                    .sum::<u32>()
            })
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
