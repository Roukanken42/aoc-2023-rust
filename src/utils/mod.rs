use std::str::FromStr;

use nom::character::complete::{digit1, line_ending};
use nom::combinator::{all_consuming, map_res, opt};
use nom::error::ParseError;
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::{IResult, Parser};

pub mod location;

pub fn parse_input_by_lines<'a, O, E, F>(f: F) -> impl FnMut(&'a str) -> IResult<&str, Vec<O>, E>
where
    F: Parser<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    parse_input(separated_list1(line_ending, f))
}

pub fn parse_input<'a, O, E, F>(f: F) -> impl FnMut(&'a str) -> IResult<&str, O, E>
where
    F: Parser<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    all_consuming(terminated(f, opt(line_ending)))
}

pub trait Parsable<'a> {
    fn parse(input: &'a str) -> IResult<&'a str, Self>
    where
        Self: Sized;
}

macro_rules! impl_parsable_uint {
    (for $($t:ty),+) => {
        $(
            impl<'a> Parsable<'a> for $t {
                fn parse(input: &str) -> IResult<&str, Self> {
                    map_res(digit1, Self::from_str)(input)
                }
            }
        )+
    };
}

impl_parsable_uint!(for u8, u16, u32, u64, u128);

impl<'a> Parsable<'a> for i64 {
    fn parse(input: &str) -> IResult<&str, Self> {
        // TODO: fix negative numbers
        map_res(digit1, i64::from_str)(input)
    }
}
