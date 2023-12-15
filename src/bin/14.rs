#![feature(min_specialization)]

use advent_of_code::utils::location::{Access2d, Location};
use itertools::Itertools;
advent_of_code::solution!(14);

fn find_lowest_position<T>(data: &Vec<Vec<T>>, gravity: Location<i32>) -> Vec<Location<i32>> {
    data.iter_2d_keys()
        .filter(|&location| data.get_2d(location.into() + gravity) == None)
        .collect_vec()
}

fn tilt(data: &mut Vec<Vec<char>>, direction: Location<i32>) -> bool {
    todo!()
}

pub fn part_one(input: &str) -> Option<u32> {
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
