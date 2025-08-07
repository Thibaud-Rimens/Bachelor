use nom::{IResult, bytes::complete::{take}, character::complete::space1, combinator::rest, Parser};
use serde_json::{json, Map, Value};
use crate::parser::FileParser;
use crate::structures::{Direction, Holiday};
use crate::utils::parse_string;

pub struct HolidayParser;

impl FileParser for HolidayParser {
    type Output = Holiday;

    fn parse_line<'a>(&mut self, input: &'a str) -> IResult<&'a str, Map<String, Value>> {
        Ok((input, Map::default()))
    }
}