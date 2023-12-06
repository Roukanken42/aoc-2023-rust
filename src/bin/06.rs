use advent_of_code::utils::{parse_input, Parsable};
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space0, space1};
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::IResult;
use num::integer::Roots;

advent_of_code::solution!(6);

// (time - x) * x > distance
// time * x - x^2 > distance
// x^2 - time * x + distance < 0
// x = (time +- sqrt(time^2 - 4 * distance)) / 2

fn parse(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (input, _) = tag("Time:")(input)?;
    let (input, times) =
        delimited(space0, separated_list1(space1, u32::parse), line_ending)(input)?;

    let (input, _) = tag("Distance:")(input)?;
    let (input, distances) =
        delimited(space0, separated_list1(space1, u32::parse), line_ending)(input)?;

    Ok((input, (times, distances)))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, (times, distances)) = parse_input(parse)(input).unwrap();

    Some(
        times
            .iter()
            .zip(distances.iter())
            .map(|(&time, &distance)| {
                let discriminant = ((time * time) as f64 - 4f64 * distance as f64).sqrt();
                let min = ((time as f64 - discriminant) / 2.0).max(0.0);
                let max = (time as f64 + discriminant) / 2.0;

                (max.ceil() - min.floor() - 1.0) as u32
            })
            .product(),
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
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
