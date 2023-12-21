use std::collections::VecDeque;

use advent_of_code::utils::location::{Access2d, Location};

advent_of_code::solution!(21);

fn count(data: &Vec<Vec<char>>, max_distance: i32, start: Location<i32>) -> i64 {
    // let start = data
    //     .iter_2d_keys()
    //     .find(|loc| data.get_2d(loc.map(|x| x as i32)) == Some(&'S'))
    //     .unwrap()
    //     .map(|x| x as i32);

    let mut visitable = vec![vec![false; data[0].len()]; data.len()];
    let mut queue = VecDeque::from([(start, 0)]);

    while let Some((current, distance)) = queue.pop_front() {
        if distance > max_distance {
            continue;
        };

        if data.get_2d(current).unwrap_or(&'#') == &'#' {
            continue;
        }
        if *visitable.get_2d(current).unwrap_or(&true) {
            continue;
        }

        if distance % 2 == max_distance % 2 {
            visitable.set_2d(current, true);
        }

        for next in current.iter_adjacent() {
            queue.push_back((next, distance + 1));
        }
    }

    visitable.iter().flatten().filter(|x| **x).count() as i64
}

pub fn part_one(input: &str) -> Option<i64> {
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let size = data.len() as i32;
    count(&data, 64, Location::new(size / 2, size / 2)).into()
}

pub fn part_two(input: &str) -> Option<i64> {
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let expansion = 21;
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
    let one = count(&data, 2 * 131 + 65, start);
    let two = count(&data, 4 * 131 + 65, start);

    // for i in 0..11 {
    //     println!("{}", count(&data, i * 131 + 65, start));
    // }

    // 3778, 33695, 93438, 183007, 302402, 451623, 630670, 839543, 1078242, 1346767, 1645118
    // 3778, 93438, 302402, 630670, 1078242, 1645118

    println!("{} {} {}", zero, one, two);

    let a = 59652;
    let b = 30008;
    let c = 3778;

    // let diff_one = one - zero;
    // let diff_two = two - zero;
    //
    // let a = (diff_one * 2 - diff_two) / ();
    //
    // println!("{} {} {}", a, b, c);
    //
    let three = count(&data, 10 * 131 + 65, start);
    assert_eq!(three, a * 5 * 5 + b * 5 + c);
    //
    Some(a * 202300 / 2 * 202300 / 2 + b * 202300 / 2 + c)
    // None
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
