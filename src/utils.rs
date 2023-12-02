use nom::character::complete::line_ending;
use nom::combinator::all_consuming;
use nom::error::ParseError;
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::{IResult, Parser};

pub fn parse_input_by_lines<'a, O, E, F>(f: F) -> impl FnMut(&'a str) -> IResult<&str, Vec<O>, E>
where
    F: Parser<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    all_consuming(terminated(separated_list1(line_ending, f), line_ending))
}
