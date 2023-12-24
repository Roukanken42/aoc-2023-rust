use advent_of_code::utils::{parse_input, Parsable};
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::{count, many1, separated_list1};
use nom::IResult;
advent_of_code::solution!(13);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Wall,
}

impl Parsable<'_> for Tile {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((value(Self::Empty, tag(".")), value(Self::Wall, tag("#"))))(input)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pattern {
    tiles: Vec<Vec<Tile>>,
}

impl Pattern {
    fn rotate(&self) -> Self {
        let mut tiles = vec![vec![Tile::Empty; self.tiles.len()]; self.tiles[0].len()];

        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                tiles[x][self.tiles.len() - y - 1] = *tile;
            }
        }

        Self { tiles }
    }

    fn find_mirror_row(&self) -> Option<usize> {
        for (i, _) in self.tiles.iter().enumerate() {
            let mut current = i;
            let mut mirrored = i + 1;

            while mirrored < self.tiles.len() {
                if self.tiles[current] != self.tiles[mirrored] {
                    break;
                }

                if current == 0 || mirrored == self.tiles.len() - 1 {
                    return Some(i + 1);
                }

                current -= 1;
                mirrored += 1;
            }
        }
        None
    }

    fn find_mirror_row_with_smudge(&self) -> Option<usize> {
        for (i, _) in self.tiles.iter().enumerate() {
            let mut current = i;
            let mut mirrored = i + 1;

            let mut had_smudge = false;

            while mirrored < self.tiles.len() {
                let differences = self.tiles[current]
                    .iter()
                    .zip(self.tiles[mirrored].iter())
                    .filter(|(a, b)| a != b)
                    .count();

                match had_smudge {
                    false if differences > 1 => break,
                    false if differences == 1 => had_smudge = true,
                    true if differences > 0 => break,
                    _ => (),
                }

                if current == 0 || mirrored == self.tiles.len() - 1 {
                    if had_smudge {
                        return Some(i + 1);
                    } else {
                        break;
                    }
                }

                current -= 1;
                mirrored += 1;
            }
        }
        None
    }
}

impl Parsable<'_> for Pattern {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, tiles) = separated_list1(line_ending, many1(Tile::parse))(input)?;

        Ok((input, Self { tiles }))
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Pattern>> {
    parse_input(separated_list1(count(line_ending, 2), Pattern::parse))(input)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, patterns) = parse(input).unwrap();

    Some(
        patterns
            .iter()
            .flat_map(|pattern| {
                pattern
                    .find_mirror_row()
                    .map(|x| x * 100)
                    .or_else(|| pattern.rotate().find_mirror_row())
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, patterns) = parse(input).unwrap();

    Some(
        patterns
            .iter()
            .flat_map(|pattern| {
                pattern
                    .find_mirror_row_with_smudge()
                    .map(|x| x * 100)
                    .or_else(|| pattern.rotate().find_mirror_row_with_smudge())
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
