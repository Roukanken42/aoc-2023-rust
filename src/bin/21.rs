use std::collections::VecDeque;

use advent_of_code::utils::location::{Access2d, Location};

advent_of_code::solution!(21);

fn count(data: &Vec<Vec<char>>, max_distance: i32, start: Location<i32>) -> i64 {
    let mut visitable = vec![vec![None; data[0].len()]; data.len()];
    let mut queue = VecDeque::from([(start, 0)]);

    while let Some((current, distance)) = queue.pop_front() {
        if distance > max_distance {
            continue;
        };

        if data.get_2d(current).unwrap_or(&'#') == &'#' {
            continue;
        }
        if visitable.get_2d(current).unwrap_or(&Some(false)).is_some() {
            continue;
        }

        visitable.set_2d(current, Some(distance % 2 == max_distance % 2));

        for next in current.iter_adjacent() {
            queue.push_back((next, distance + 1));
        }
    }

    visitable
        .iter()
        .flatten()
        .filter(|x| x.is_some_and(|a| a))
        .count() as i64
}

pub fn part_one(input: &str) -> Option<i64> {
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let size = data.len() as i32;
    count(&data, 64, Location::new(size / 2, size / 2)).into()
}

pub fn part_two(input: &str) -> Option<i64> {
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // TODO: optimize
    let expansion = 7;
    let data = data
        .iter()
        .cycle()
        .map(|row| {
            row.iter()
                .cycle()
                .copied()
                .take(row.len() * expansion)
                .collect::<Vec<_>>()
        })
        .take(data.len() * expansion)
        .collect::<Vec<_>>();

    let size = data.len() as i32;
    let start = Location::new(size / 2, size / 2);

    let zero = count(&data, 65, start);
    let one = count(&data, 131 + 65, start);
    let two = count(&data, 2 * 131 + 65, start);

    let c = zero;
    let a = (two - 2 * one + c) / 2;
    let b = one - a - c;

    let three = count(&data, 3 * 131 + 65, start);
    assert_eq!(three, a * 3 * 3 + b * 3 + c);

    Some(a * 202300 * 202300 + b * 202300 + c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
