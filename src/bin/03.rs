advent_of_code::solution!(3);

trait Utils2d<T> {
    fn get_2d(&self, x: usize, y: usize) -> Option<&T>;
}

impl<T> Utils2d<T> for Vec<Vec<T>> {
    fn get_2d(&self, x: usize, y: usize) -> Option<&T> {
        self.get(y).and_then(|row| row.get(x))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let x: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

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
