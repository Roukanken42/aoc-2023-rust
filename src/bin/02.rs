use std::collections::HashMap;
use std::slice::Iter;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{map, value};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

use advent_of_code::utils::{parse_input_by_lines, Parsable};

use crate::Color::*;

advent_of_code::solution!(2);

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn iterator() -> Iter<'static, Color> {
        static COLORS: [Color; 3] = [Red, Green, Blue];
        COLORS.iter()
    }
}

impl<'a> Parsable<'a> for Color {
    fn parse(input: &str) -> IResult<&str, Color> {
        alt((
            value(Red, tag("red")),
            value(Green, tag("green")),
            value(Blue, tag("blue")),
        ))(input)
    }
}

fn parse_color_map(input: &str) -> IResult<&str, HashMap<Color, u32>> {
    let color_count = map(
        separated_pair(u32::parse, char(' '), Color::parse),
        |(count, color)| (color, count),
    );

    let (input, color_list) = separated_list1(tag(", "), color_count)(input)?;

    return Ok((input, color_list.into_iter().collect()));
}

#[derive(Eq, PartialEq, Debug)]
struct DiceGame {
    id: u32,
    sets: Vec<HashMap<Color, u32>>,
}

impl<'a> Parsable<'a> for DiceGame {
    fn parse(input: &str) -> IResult<&str, DiceGame> {
        let (input, _) = tag("Game ")(input)?;
        let (input, id) = u32::parse(input)?;
        let (input, _) = tag(": ")(input)?;

        let (input, sets) = separated_list1(tag("; "), parse_color_map)(input)?;

        Ok((input, DiceGame { id, sets }))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, games) = parse_input_by_lines(DiceGame::parse)(input).unwrap();

    let target = HashMap::from([(Red, 12u32), (Green, 13u32), (Blue, 14u32)]);

    let set_is_possible = |set: &HashMap<Color, u32>| {
        set.iter()
            .all(|(color, count)| target.get(color).is_some_and(|max_dice| count <= max_dice))
    };

    Some(
        games
            .iter()
            .filter(|game| game.sets.iter().all(set_is_possible))
            .map(|game| game.id)
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, games) = parse_input_by_lines(DiceGame::parse)(input).unwrap();

    let minimum_dices_for_game = |game: &DiceGame| -> HashMap<Color, u32> {
        Color::iterator()
            .map(|&color| {
                (
                    color,
                    game.sets
                        .iter()
                        .map(|set| set.get(&color).unwrap_or(&0).clone())
                        .max()
                        .unwrap_or(0),
                )
            })
            .collect()
    };

    Some(
        games
            .iter()
            .map(minimum_dices_for_game)
            .map(|dices| {
                dices
                    .iter()
                    .map(|(_, &count)| count)
                    .reduce(|a, b| a * b)
                    .unwrap_or(0)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use advent_of_code::utils::Parsable;

    use super::*;

    #[test]
    fn test_color_parse() {
        let result = Color::parse("red");
        let (input, color) = result.expect("Expected parser to success, but it returned");

        assert_eq!(input, "");
        assert_eq!(color, Red);
    }

    #[test]
    fn test_color_map_parse() {
        assert_eq!(
            parse_color_map("8 green, 6 blue, 20 red"),
            Ok(("", HashMap::from([(Green, 8), (Blue, 6), (Red, 20)])))
        );
    }

    #[test]
    fn test_dice_game_parse() {
        assert_eq!(
            DiceGame::parse(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            ),
            Ok((
                "",
                DiceGame {
                    id: 3,
                    sets: vec![
                        HashMap::from([(Green, 8), (Blue, 6), (Red, 20)]),
                        HashMap::from([(Blue, 5), (Red, 4), (Green, 13)]),
                        HashMap::from([(Green, 5), (Red, 1)])
                    ]
                }
            ))
        );
    }

    #[test]
    fn test_part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2720));
    }

    #[test]
    fn test_part_two_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(71535));
    }
}
