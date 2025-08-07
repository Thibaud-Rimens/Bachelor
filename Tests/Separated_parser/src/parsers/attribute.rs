use nom::{IResult, bytes::complete::{take}, character::complete::space1, combinator::rest, Parser};
use nom::bytes::complete::{tag, take_till};
use serde_json::{json, Map, Value};
use crate::parser::FileParser;
use crate::structures::{Attribute, HMap};
use crate::utils::parse_string;

#[derive(Default)]
pub struct AttributeParser {
    data: HMap,
}

impl AttributeParser {
    fn parse_1(input: &str) -> IResult<&str, ()> {
        let mut parser = (
            take(2usize), // string
            space1,
            take(1usize), // i16
            space1,
            take(3usize), // i16
            space1,
            take(2usize), // i16

        );
        let (_input, _) = parser.parse(input)?;
        Ok((input, ()))
    }
    fn parse_2(input: &str) -> IResult<&str, ()> {
        let mut parser = (
            tag("#"),
            space1,
            rest
        );
        let (_input, _) = parser.parse(input)?;
        Ok((input, ()))
    }
    fn parse_3(input: &str) -> IResult<&str, ()> {
        let mut parser = (
            tag("<"),
            take_till(|c| c == '>'),
        );
        let (_input, _) = parser.parse(input)?;
        Ok((input, ()))
    }
    fn parse_4(input: &str) -> IResult<&str, ()> {
        let mut parser = (
            take_till(|c| c == ' '),
            space1,
            rest
        );
        let (_input, (
            _,
            _,
            name
        )) = parser.parse(input)?;
        Ok((input, ()))
    }
}

impl FileParser for AttributeParser {
    type Output = Attribute;

    fn parse_line<'a>(&mut self, input: &'a str) -> IResult<&'a str, Map<String, Value>> {
        Ok((input, Map::default()))
    }
}
