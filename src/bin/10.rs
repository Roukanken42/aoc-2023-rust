use std::collections::HashMap;
use std::iter::{repeat_with, RepeatWith};

use itertools::Itertools;
use num::Zero;

use advent_of_code::utils::location::direction::*;
use advent_of_code::utils::location::{Access2d, Location};

advent_of_code::solution!(10);

fn turn(direction: Location<i32>, tile: char) -> Option<Location<i32>> {
    match (tile, direction) {
        ('|', dir) if dir.x.is_zero() => Some(dir),
        ('-', dir) if dir.y.is_zero() => Some(dir),
        ('L', dir) if dir == DOWN => Some(RIGHT),
        ('L', dir) if dir == LEFT => Some(UP),
        ('J', dir) if dir == DOWN => Some(LEFT),
        ('J', dir) if dir == RIGHT => Some(UP),
        ('7', dir) if dir == RIGHT => Some(DOWN),
        ('7', dir) if dir == UP => Some(LEFT),
        ('F', dir) if dir == LEFT => Some(DOWN),
        ('F', dir) if dir == UP => Some(RIGHT),
        ('S', dir) => Some(dir),
        _ => None,
    }
}

fn move_trough<'a>(
    start: Location<i32>,
    start_direction: Location<i32>,
    map: &'a Vec<Vec<char>>,
) -> RepeatWith<impl FnMut() -> Option<Location<i32>> + 'a + Clone> {
    let mut current = start;
    let mut direction = start_direction;

    let repeater = move || -> Option<Location<i32>> {
        let tile = map.get_2d(current + direction)?;

        current = current + direction;
        direction = turn(direction, *tile)?;

        Some(current)
    };

    repeat_with(repeater)
}

pub fn part_one(input: &str) -> Option<usize> {
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let start = Location::new(0, 0)
        .iter_range(Location::new(data[0].len() as i32, data.len() as i32))
        .find(|loc| data.get_2d(*loc).unwrap() == &'S')
        .unwrap();

    let cardinal_directions: Vec<Location<i32>> = vec![LEFT, RIGHT, UP, DOWN];

    let count = cardinal_directions
        .iter()
        .filter_map(|direction| {
            let mut iter = move_trough(start, *direction, &data).while_some();
            let count = iter.take_while_ref(|loc| *loc != start).count();

            if iter.next() == Some(start) {
                Some(count)
            } else {
                None
            }
        })
        .max();

    count.map(|c| (c + 1) / 2)
}

pub fn get_edge_fill_map() -> HashMap<char, Vec<Location<i32>>> {
    HashMap::from([
        ('|', vec![ZERO, UP, DOWN]),
        ('-', vec![ZERO, LEFT, RIGHT]),
        ('L', vec![ZERO, UP, RIGHT]),
        ('J', vec![ZERO, UP, LEFT]),
        ('7', vec![ZERO, DOWN, LEFT]),
        ('F', vec![ZERO, DOWN, RIGHT]),
        ('S', vec![ZERO]),
    ])
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Insides {
    Inside,
    Edge,
    Outside,
}

pub fn part_two(input: &str) -> Option<usize> {
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let map_end = Location::new(data[0].len() as i32, data.len() as i32);
    let start = Location::new(0, 0)
        .iter_range(map_end)
        .find(|loc| data.get_2d(*loc).unwrap() == &'S')
        .unwrap();

    let cardinal_directions: Vec<Location<i32>> = vec![LEFT, RIGHT, UP, DOWN];

    let directions = cardinal_directions
        .iter()
        .filter_map(|direction| {
            let mut iter = move_trough(start, *direction, &data).while_some();
            iter.take_while_ref(|loc| *loc != start).for_each(drop);

            if iter.next() == Some(start) {
                Some(*direction)
            } else {
                None
            }
        })
        .collect_vec();

    let edge_fill_map = get_edge_fill_map();
    let mut fill = vec![vec![Insides::Inside; data[0].len() * 3]; data.len() * 3];
    let direction = directions[0];

    move_trough(start, direction, &data)
        .while_some()
        .take_while(|loc| *loc != start)
        .for_each(|loc| {
            let base_loc = (loc * 3) + Location::new(1, 1);
            let tile = data.get_2d(loc).unwrap();

            for &fill_loc in edge_fill_map.get(tile).unwrap() {
                fill.set_2d(base_loc + fill_loc, Insides::Edge);
            }
        });

    let start_base_loc = (start * 3) + Location::new(1, 1);
    directions.iter().for_each(|dir| {
        fill.set_2d(start_base_loc + *dir, Insides::Edge);
    });
    fill.set_2d(start_base_loc, Insides::Edge);

    let mut next = vec![Location::new(0, 0)];
    while !next.is_empty() {
        let current = next.pop().unwrap();

        match fill.get_2d(current) {
            Some(Insides::Inside) => {
                fill.set_2d(current, Insides::Outside);
                next.extend(current.neighbours().iter());
            }
            _ => {}
        }
    }

    Some(
        Location::new(0, 0)
            .iter_range(map_end)
            .filter_map(|loc| fill.get_2d((loc * 3) + Location::new(1, 1)))
            .filter(|&&x| x == Insides::Inside)
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
