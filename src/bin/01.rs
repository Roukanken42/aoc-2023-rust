use rayon::prelude::*;

advent_of_code::solution!(1);

fn get_digits(input: &str) -> Vec<u32> {
    input.chars().filter_map(|c| c.to_digit(10)).collect()
}

static DIGITS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

fn get_digits_spelled(input: &str) -> (u32, u32) {
    let &first = DIGITS
        .iter()
        .min_by_key(|&&digit| input.find(digit).unwrap_or(usize::MAX))
        .unwrap();

    let &last = DIGITS
        .iter()
        .filter(|&&digit| input.contains(digit))
        .max_by_key(|&&digit| input.rfind(digit).unwrap_or(usize::MIN))
        .unwrap();

    let first_digit = DIGITS
        .iter()
        .enumerate()
        .find(|(_, &digit)| digit == first)
        .unwrap()
        .0 as u32;

    let last_digit = DIGITS
        .iter()
        .enumerate()
        .find(|(_, &digit)| digit == last)
        .unwrap()
        .0 as u32;

    (first_digit % 9 + 1, last_digit % 9 + 1)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .par_bridge()
            .map(get_digits)
            .map(|digits| digits.first().unwrap() * 10 + digits.last().unwrap())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .par_bridge()
            .map(get_digits_spelled)
            .map(|(first, last)| first * 10 + last)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_digits_spelled() {
        // example ones
        assert_eq!(get_digits_spelled("two1nine"), (2, 9));
        assert_eq!(get_digits_spelled("eightwothree"), (8, 3));
        assert_eq!(get_digits_spelled("abcone2threexyz"), (1, 3));
        assert_eq!(get_digits_spelled("xtwone3four"), (2, 4));
        assert_eq!(get_digits_spelled("4nineeightseven2"), (4, 2));
        assert_eq!(get_digits_spelled("zoneight234"), (1, 4));
        assert_eq!(get_digits_spelled("7pqrstsixteen"), (7, 6));

        // custom ones
        assert_eq!(get_digits_spelled("oneight"), (1, 8));
        assert_eq!(get_digits_spelled("2eight4264"), (2, 4));
        assert_eq!(get_digits_spelled("three"), (3, 3));
    }

    #[test]
    fn test_part_one_examples() {
        let input = r#"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "#
        .trim();

        let result = part_one(input);
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(54561));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(54076));
    }
}
