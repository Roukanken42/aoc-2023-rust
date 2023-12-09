use advent_of_code::utils::{parse_input_by_lines, Parsable};

advent_of_code::solution!(9);

fn derive(input: &[i32]) -> Vec<i32> {
    input
        .iter()
        .zip(input.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect::<Vec<_>>()
}

fn predict(input: &[i32]) -> i32 {
    if input.iter().all(|&d| d == 0) {
        return 0;
    }

    input.last().unwrap() + predict(&derive(input))
}

fn predict_past(input: &[i32]) -> i32 {
    if input.iter().all(|&d| d == 0) {
        return 0;
    }

    input.first().unwrap() - predict_past(&derive(input))
}

pub fn part_one(input: &str) -> Option<i32> {
    let (_, inputs) = parse_input_by_lines(Vec::<i32>::parse)(input).unwrap();

    Some(inputs.iter().map(|input| predict(input)).sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    let (_, inputs) = parse_input_by_lines(Vec::<i32>::parse)(input).unwrap();

    Some(inputs.iter().map(|input| predict_past(input)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
