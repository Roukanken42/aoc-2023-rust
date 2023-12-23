use advent_of_code::utils::location::{Access2d, Location};
use advent_of_code::utils::{parse_input_by_lines, Parsable};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::sequence::{separated_pair, tuple};
use nom::IResult;
use num::Bounded;
use std::cmp::{max, min};
use std::collections::HashSet;
advent_of_code::solution!(22);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Brick {
    start: Location<i32>,
    end: Location<i32>,
    height: i32,
    y: i32,
}

impl Parsable<'_> for Brick {
    fn parse(input: &str) -> IResult<&str, Self> {
        let parse_coords = |input| tuple((i32::parse, tag(","), i32::parse, tag(","), i32::parse))(input);

        let (input, (coords1, coords2)) = separated_pair(parse_coords, tag("~"), parse_coords)(input)?;
        let (x1, _, z1, _, y1) = coords1;
        let (x2, _, z2, _, y2) = coords2;

        Ok((
            input,
            Self {
                start: Location::new(x1, z1),
                end: Location::new(x2 + 1, z2 + 1),
                height: y2 - y1 + 1,
                y: y1,
            },
        ))
    }
}

fn calculate_supported_by(bricks: &Vec<Brick>) -> Vec<HashSet<usize>> {
    let mut min_loc = Location::max_value();
    let mut max_loc = Location::min_value();

    for brick in bricks {
        min_loc.x = min(min_loc.x, brick.start.x);
        min_loc.y = min(min_loc.y, brick.start.y);
        max_loc.x = max(max_loc.x, brick.end.x);
        max_loc.y = max(max_loc.y, brick.end.y);
    }

    let mut grid = vec![vec![(0, None); (max_loc.x - min_loc.x) as usize]; (max_loc.y - min_loc.y) as usize];

    let mut supported_by: Vec<HashSet<usize>> = vec![HashSet::new(); bricks.len()];

    for (i, brick) in bricks.iter().enumerate().sorted_by_key(|elem| elem.1.y) {
        let mut max = 0;
        let mut supports = HashSet::new();

        for loc in brick.start.iter_range(brick.end) {
            let Some((support_y, Some(support_id))) = grid.get_2d(loc - min_loc) else {
                continue;
            };

            match () {
                () if *support_y == max => {
                    supports.insert(*support_id);
                }
                () if *support_y > max => {
                    max = *support_y;
                    supports = HashSet::from([*support_id]);
                }
                _ => {}
            }
        }

        for loc in brick.start.iter_range(brick.end) {
            grid.set_2d(loc - min_loc, (max + brick.height, Some(i)));
        }

        supported_by[i] = supports;
    }
    supported_by
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, bricks) = parse_input_by_lines(Brick::parse)(input).unwrap();

    let supported_by = calculate_supported_by(&bricks);

    let non_removable_count = supported_by
        .iter()
        .filter(|supports| supports.len() == 1)
        .map(|supports| supports.iter().next().unwrap())
        .unique()
        .count();

    Some(bricks.len() - non_removable_count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, bricks) = parse_input_by_lines(Brick::parse)(input).unwrap();

    let supported_by = calculate_supported_by(&bricks);
    let mut supports = vec![HashSet::new(); bricks.len()];

    for (i, supported_by) in supported_by.iter().enumerate() {
        for support in supported_by {
            supports[*support].insert(i);
        }
    }

    let causes_to_fall = |id: usize| {
        let mut causes_to_fall = HashSet::from([id]);
        let mut queue = Vec::from_iter(supports[id].iter());

        while let Some(supported_id) = queue.pop() {
            if causes_to_fall.is_superset(&supported_by[*supported_id]) {
                causes_to_fall.insert(*supported_id);
                queue.extend(&supports[*supported_id]);
            }
        }

        causes_to_fall
    };

    (0..bricks.len()).map(|id| causes_to_fall(id).len() - 1).sum::<usize>().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
