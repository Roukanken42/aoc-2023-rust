use std::str::FromStr;

use itertools::Itertools;
use nom::character::complete::{one_of, space1};
use nom::combinator::map_res;
use nom::multi::count;
use nom::sequence::separated_pair;
use nom::IResult;

use advent_of_code::utils::{parse_input_by_lines, Parsable};

advent_of_code::solution!(7);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
enum CardValue {
    Value(u32),
}

impl CardValue {
    fn to_with_jokers(self) -> Self {
        match self {
            Self::Value(11) => Self::Value(1),
            Self::Value(_) => self,
        }
    }
}

impl FromStr for CardValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "T" => Ok(Self::Value(10)),
            "J" => Ok(Self::Value(11)),
            "Q" => Ok(Self::Value(12)),
            "K" => Ok(Self::Value(13)),
            "A" => Ok(Self::Value(14)),
            _ if s.len() == 1 => Ok(Self::Value(s.chars().next().unwrap().to_digit(10).unwrap())),
            _ => Err(()),
        }
    }
}

impl Parsable for CardValue {
    fn parse(input: &str) -> IResult<&str, Self> {
        map_res(one_of("23456789TJQKA"), |s: char| {
            CardValue::from_str(&s.to_string())
        })(input)
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
struct BetHand {
    cards: Vec<CardValue>,
    value: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
enum CamelCardsResult {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl CamelCardsResult {
    fn upgrade_with_jokers(self, jokers: u32) -> CamelCardsResult {
        match (jokers, self) {
            (5, Self::HighCard) => Self::FiveOfAKind,
            (4, Self::HighCard) => Self::FiveOfAKind,
            (3, Self::OnePair) => Self::FiveOfAKind,
            (3, Self::HighCard) => Self::FourOfAKind,
            (2, Self::ThreeOfAKind) => Self::FiveOfAKind,
            (2, Self::OnePair) => Self::FourOfAKind,
            (2, Self::HighCard) => Self::ThreeOfAKind,
            (1, Self::FourOfAKind) => Self::FiveOfAKind,
            (1, Self::ThreeOfAKind) => Self::FourOfAKind,
            (1, Self::TwoPairs) => Self::FullHouse,
            (1, Self::OnePair) => Self::ThreeOfAKind,
            (1, Self::HighCard) => Self::OnePair,
            (0, x) => x,
            (_, _) => panic!("Invalid hand"),
        }
    }

    fn from_hand(hand: &Vec<CardValue>) -> Self {
        let counts = hand.iter().counts_by(|card| match card {
            CardValue::Value(value) => *value,
        });

        let mut pairs = 0;
        let mut three = false;

        for (_, count) in counts.iter() {
            match count {
                2 => pairs += 1,
                3 => three = true,
                4 => return Self::FourOfAKind,
                5 => return Self::FiveOfAKind,
                _ => {}
            }
        }

        match (pairs, three) {
            (0, false) => Self::HighCard,
            (1, false) => Self::OnePair,
            (2, false) => Self::TwoPairs,
            (0, true) => Self::ThreeOfAKind,
            (1, true) => Self::FullHouse,
            _ => panic!("Invalid hand"),
        }
    }

    fn from_hand_with_jokers(hand: &Vec<CardValue>) -> Self {
        let hand_without_jokers: Vec<CardValue> = hand
            .iter()
            .copied()
            .filter(|card| match card {
                CardValue::Value(11) => false,
                CardValue::Value(_) => true,
            })
            .collect();

        let jokers = hand.len() - hand_without_jokers.len();
        let result = Self::from_hand(&hand_without_jokers);

        return result.upgrade_with_jokers(jokers as u32);
    }
}

impl Parsable for BetHand {
    fn parse(input: &str) -> IResult<&str, Self> {
        let parse_cards = count(CardValue::parse, 5);
        let (input, (cards, value)) = separated_pair(parse_cards, space1, u32::parse)(input)?;

        Ok((input, Self { cards, value }))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, data) = parse_input_by_lines(BetHand::parse)(input).unwrap();

    Some(
        data.iter()
            .sorted_by_key(|hand| (CamelCardsResult::from_hand(&hand.cards), hand.cards.clone()))
            .enumerate()
            .map(|(index, hand)| hand.value * (index as u32 + 1))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, data) = parse_input_by_lines(BetHand::parse)(input).unwrap();

    Some(
        data.iter()
            .sorted_by_key(|hand| {
                (
                    CamelCardsResult::from_hand_with_jokers(&hand.cards),
                    hand.cards
                        .iter()
                        .map(|&value| value.to_with_jokers())
                        .collect::<Vec<CardValue>>(),
                )
            })
            .enumerate()
            .map(|(index, hand)| hand.value * (index as u32 + 1))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
