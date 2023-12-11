use advent_of_code::utils::location::{Access2d, Location};
use itertools::Itertools;
use num::abs;
advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<i32> {
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows_expansion = data
        .iter()
        .scan(0, |state, row| {
            if row.iter().all(|&c| c == '.') {
                *state += 1;
                Some(*state)
            } else {
                Some(*state)
            }
        })
        .collect_vec();

    let cols_expansion = (0..data[0].len())
        .into_iter()
        .scan(0, |state, col| {
            if data.iter().map(|row| row[col]).all(|c| c == '.') {
                *state += 1;
                Some(*state)
            } else {
                Some(*state)
            }
        })
        .collect_vec();

    let galaxy_coordinates = Location::new(0, 0)
        .iter_range(Location::new(data[0].len() as i32, data.len() as i32))
        .filter(|location| *data.get_2d(*location).unwrap_or(&'.') == '#')
        .map(|location| {
            let row = rows_expansion[location.y as usize];
            let col = cols_expansion[location.x as usize];
            location + Location::new(col, row)
        })
        .collect_vec();

    Some(
        galaxy_coordinates
            .iter()
            .cartesian_product(galaxy_coordinates.iter())
            .map(|(a, b)| abs(a.x - b.x) + abs(a.y - b.y))
            .sum::<i32>()
            / 2,
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
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
