use nom::{IResult, bytes::complete::{take}, character::complete::space1, combinator::rest, Parser};
use serde_json::{json, Map, Value};
use crate::parser::FileParser;
use crate::structures::{Direction, Stop};
use crate::utils::parse_string;

pub struct StopParser;

impl FileParser for StopParser {
    type Output = Stop;

    fn parse_line<'a>(&mut self, input: &'a str) -> IResult<&'a str, Map<String, Value>> {
        Ok((input, Map::default()))
    }
}