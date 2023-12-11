use advent_of_code::utils::location::{Access2d, Location};
use itertools::Itertools;
use num::abs;
advent_of_code::solution!(11);

fn count_row_expansion(data: &Vec<Vec<char>>) -> Vec<i64> {
    data.iter()
        .scan(0, |state, row| {
            if row.iter().all(|&c| c == '.') {
                *state += 1;
            }
            Some(*state as i64)
        })
        .collect_vec()
}

fn count_cols_expansion(data: &Vec<Vec<char>>) -> Vec<i64> {
    (0..data[0].len())
        .into_iter()
        .scan(0, |state, col| {
            if data.iter().map(|row| row[col]).all(|c| c == '.') {
                *state += 1;
            }
            Some(*state as i64)
        })
        .collect_vec()
}

fn calculate_galaxy_coordinates(data: &Vec<Vec<char>>, expansion: i64) -> Vec<Location<i64>> {
    let rows_expansion = count_row_expansion(&data);
    let cols_expansion = count_cols_expansion(&data);

    Location::new(0, 0)
        .iter_range(Location::new(data[0].len() as i32, data.len() as i32))
        .filter(|location| *data.get_2d(*location).unwrap_or(&'.') == '#')
        .map(|location| {
            let row = rows_expansion[location.y as usize] as i64;
            let col = cols_expansion[location.x as usize] as i64;
            Location::new(location.x as i64, location.y as i64)
                + (Location::new(col, row) * expansion)
        })
        .collect_vec()
}

fn sum_all_paths(galaxy_coordinates: Vec<Location<i64>>) -> i64 {
    galaxy_coordinates
        .iter()
        .cartesian_product(galaxy_coordinates.iter())
        .map(|(a, b)| abs(a.x - b.x) + abs(a.y - b.y))
        .sum::<i64>()
        / 2
}

pub fn part_one(input: &str) -> Option<i64> {
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let galaxy_coordinates = calculate_galaxy_coordinates(&data, 1);
    Some(sum_all_paths(galaxy_coordinates))
}

pub fn part_two(input: &str) -> Option<i64> {
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let galaxy_coordinates = calculate_galaxy_coordinates(&data, 999999);
    Some(sum_all_paths(galaxy_coordinates))
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
        assert_eq!(result, Some(82000210));
    }
}
