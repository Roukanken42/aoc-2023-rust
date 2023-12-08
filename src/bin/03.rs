use advent_of_code::utils::location::Location;
use std::collections::HashSet;

advent_of_code::solution!(3);

trait Access2d<T> {
    fn get_2d(&self, loc: Location<i32>) -> Option<&T>;
    fn set_2d(&mut self, loc: Location<i32>, element: T) -> Option<()>;
}

impl<T> Access2d<T> for Vec<Vec<T>> {
    fn get_2d(&self, loc: Location<i32>) -> Option<&T> {
        self.get(usize::try_from(loc.y).ok()?)
            .and_then(|row| row.get(usize::try_from(loc.x).ok()?))
    }

    fn set_2d(&mut self, loc: Location<i32>, element: T) -> Option<()> {
        self.get_mut(usize::try_from(loc.y).ok()?)
            .and_then(|row| {
                row.insert(usize::try_from(loc.x).ok()?, element);
                Some(())
            })
            .map(|_| ())
    }
}

#[derive(Debug, Copy, Clone)]
struct NumberPointer {
    value: u32,
    has_neighbour: bool,
}

struct NumbersInCharMatrix {
    numbers: Vec<NumberPointer>,
    locations: Vec<Vec<Option<usize>>>,
}

fn find_numbers(data: &Vec<Vec<char>>) -> NumbersInCharMatrix {
    let mut numbers = vec![];
    let mut number = NumberPointer {
        value: 0,
        has_neighbour: false,
    };

    let mut locations: Vec<Vec<Option<_>>> = data
        .iter()
        .map(|row| row.iter().map(|_| None).collect())
        .collect();

    for location in
        Location::new(0, 0).iter_range(Location::new(data[0].len() as i32, data.len() as i32))
    {
        let char = data.get_2d(location).unwrap();

        if !('0'..='9').contains(char) || location.x == 0 {
            numbers.push(number);
            number = NumberPointer {
                value: 0,
                has_neighbour: false,
            };
        }

        if ('0'..='9').contains(char) {
            number.value = number.value * 10 + char.to_digit(10).unwrap();
            locations.set_2d(location, Some(numbers.len()));
        }
    }

    numbers.push(number);

    NumbersInCharMatrix { numbers, locations }
}

pub fn part_one(input: &str) -> Option<u32> {
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut numbers = find_numbers(&data);

    for location in
        Location::new(0, 0).iter_range(Location::new(data[0].len() as i32, data.len() as i32))
    {
        if let Some(number) = numbers.locations.get_2d(location).unwrap() {
            location
                .neighbours()
                .iter()
                .filter_map(|loc| data.get_2d(*loc))
                .for_each(|char| match char {
                    '0'..='9' => {}
                    '.' => {}
                    _ => numbers.numbers[*number].has_neighbour = true,
                });
        }
    }

    Some(
        numbers
            .numbers
            .into_iter()
            .filter(|a| a.has_neighbour)
            .map(|a| a.value)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut numbers = find_numbers(&data);

    Some(
        Location::new(0, 0)
            .iter_range(Location::new(data[0].len() as i32, data.len() as i32))
            .filter(|loc| data.get_2d(*loc).unwrap() == &'*')
            .map(|loc| {
                loc.neighbours()
                    .iter()
                    .filter_map(|loc| numbers.locations.get_2d(*loc))
                    .filter_map(|number| *number)
                    .collect()
            })
            .filter(|set: &HashSet<_>| set.len() == 2)
            .map(|set| {
                set.into_iter()
                    .map(|index| numbers.numbers.get(index).unwrap().value)
                    .product::<u32>()
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

        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
