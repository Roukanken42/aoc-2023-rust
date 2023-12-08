use advent_of_code::utils::location::Location;

advent_of_code::solution!(3);

trait Access2d<T> {
    fn get_2d(&self, loc: Location<i32>) -> Option<&T>;
}

impl<T> Access2d<T> for Vec<Vec<T>> {
    fn get_2d(&self, loc: Location<i32>) -> Option<&T> {
        self.get(usize::try_from(loc.y).ok()?)
            .and_then(|row| row.get(usize::try_from(loc.x).ok()?))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut number = 0;
    let mut has_neighbour = false;

    let mut numbers = Vec::<u32>::new();

    for location in
        Location::new(0, 0).iter_range(Location::new(data[0].len() as i32, data.len() as i32))
    {
        let char = data.get_2d(location).unwrap();

        match char {
            '0'..='9' => number = number * 10 + char.to_digit(10).unwrap(),
            _ => {
                if has_neighbour && number != 0 {
                    numbers.push(number)
                }
                number = 0;
                has_neighbour = false;
                continue;
            }
        };

        location
            .neighbours()
            .iter()
            .filter_map(|loc| data.get_2d(*loc))
            .for_each(|char| match char {
                '0'..='9' => {}
                '.' => {}
                _ => has_neighbour = true,
            });
    }

    Some(numbers.into_iter().sum())
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

        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
