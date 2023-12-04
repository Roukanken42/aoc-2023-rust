use nom::character::complete::{digit1, line_ending};
use nom::combinator::{all_consuming, map_res};
use nom::error::ParseError;
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::{IResult, Parser};
use std::str::FromStr;

pub fn parse_input_by_lines<'a, O, E, F>(f: F) -> impl FnMut(&'a str) -> IResult<&str, Vec<O>, E>
where
    F: Parser<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    all_consuming(terminated(separated_list1(line_ending, f), line_ending))
}

pub trait Parsable {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized;
}

impl Parsable for u32 {
    fn parse(input: &str) -> IResult<&str, Self> {
        map_res(digit1, u32::from_str)(input)
    }
}