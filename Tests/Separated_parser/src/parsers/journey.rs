use nom::bytes::complete::{tag, take};
use nom::character::complete::space1;
use nom::combinator::rest;
use nom::{IResult, Parser};
use serde_json::{Map, Value};
use crate::parser::FileParser;
use crate::structures::{HMap, Line};

#[derive(Default)]
pub struct JourneyParser {
    data: HMap,
}


impl JourneyParser {
    // TODO: Create enough line types
    fn parse_1/*TODO : name*/(input: &str) -> IResult<&str, ()> {
        let mut parser = (
            tag("*Z"),
            space1,
            take(6usize), // i32
            space1,
            take(6usize), // string
        );

        let (input, _) = parser.parse(input)?;
        Ok((input, ()))
    }
    fn parse_2/*TODO : name*/(input: &str) -> IResult<&str, ()> {
        let mut parser = (
            tag("*G"),
            space1,
            take(3usize), // string
            space1,
            take(7usize), // option i32
            space1,
            take(7usize), // option i32
        );

        let (input, _) = parser.parse(input)?;
        Ok((input, ()))
    }
    fn parse_3/*TODO : name*/(input: &str) -> IResult<&str, ()> {
        let mut parser = (
            tag("*A VE"),
            space1,
            take(7usize), // option i32
            space1,
            take(7usize), // option i32
            space1,
            take(6usize), // option i32
        );

        let (input, _) = parser.parse(input)?;
        Ok((input, ()))
    }
    fn parse_4/*TODO : name*/(input: &str) -> IResult<&str, ()> {
        let mut parser = (
            tag("*A"),
            space1,
            take(2usize), // string
            space1,
            take(7usize), // option i32
            space1,
            take(7usize), // option i32
        );

        let (input, _) = parser.parse(input)?;
        Ok((input, ()))
    }
    fn parse_5/*TODO : name*/(input: &str) -> IResult<&str, ()> {
        let mut parser = (
            tag("*I"),
            space1,
            take(2usize), // string
            space1,
            take(7usize), // option i32
            space1,
            take(7usize), // option i32
            space1,
            take(6usize), // option i32
            space1,
            take(9usize), // i32
            space1,
            take(6usize), // option i32
            space1,
            take(6usize), // option i32
        );

        let (input, _) = parser.parse(input)?;
        Ok((input, ()))
    }
    fn parse_6/*TODO : name*/(input: &str) -> IResult<&str, ()> {
        let mut parser = (
            tag("*L"),
            space1,
            take(8usize), // string
            space1,
            take(7usize), // option i32
            space1,
            take(7usize), // option i32
            space1,
            take(6usize), // option i32
            space1,
            take(6usize), // option i32
        );

        let (input, _) = parser.parse(input)?;
        Ok((input, ()))
    }
    fn parse_7/*TODO : name*/(input: &str) -> IResult<&str, ()> {
        let mut parser = (
            tag("*R"),
            space1,
            take(1usize), // string
            space1,
            take(7usize), // string
            space1,
            take(7usize), // option i32
            space1,
            take(7usize), // option i32
            space1,
            take(6usize), // option i32
            space1,
            take(6usize), // option i32
        );

        let (input, _) = parser.parse(input)?;
        Ok((input, ()))
    }
    fn parse_8/*TODO : name*/(input: &str) -> IResult<&str, ()> {
        let mut parser = (
            tag("*CI"), // see if need to be changed
            space1,
            take(4usize), // i32
            space1,
            take(7usize), // option i32
            space1,
            take(7usize), // option i32
        );

        let (input, _) = parser.parse(input)?;
        Ok((input, ()))
    }
    fn parse_9/*TODO : name*/(input: &str) -> IResult<&str, ()> {
        let mut parser = (
            tag("*CO"), // see if need to be changed
            space1,
            take(4usize), // i32
            space1,
            take(7usize), // option i32
            space1,
            take(7usize), // option i32
        );

        let (input, _) = parser.parse(input)?;
        Ok((input, ()))
    }
    fn parse_10/*TODO : name*/(input: &str) -> IResult<&str, ()> {
        let mut parser = (
            take(7usize), // i32
            space1,
            take(6usize), // option i32
            space1,
            take(6usize), // option i32
        );

        let (input, _) = parser.parse(input)?;
        Ok((input, ()))
    }
}

impl FileParser for JourneyParser {
    type Output = Line;

    fn parse_line<'a>(&mut self, input: &'a str) -> IResult<&'a str, Map<String, Value>> {
        // TODO: fplan obj code
        Ok((input, Map::default()))
    }
}
