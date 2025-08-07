use nom::{IResult, bytes::complete::{take}, character::complete::space1, combinator::rest, Parser};
use serde_json::{json, Map, Value};
use crate::parser::FileParser;
use crate::structures::Direction;
use crate::utils::parse_string;

pub struct DirectionParser;

impl FileParser for DirectionParser {
    type Output = Direction;

    fn parse_line<'a>(&mut self, input: &'a str) -> IResult<&'a str, Map<String, Value>> {
        let mut parser = (
            take(7usize),
            space1,
            rest
        );
        let (input, (
            id,
            _,
            name
        )) = parser.parse(input)?;

        let mut obj = Map::new();
        obj.insert("id".into(), json!(parse_string(id)));
        obj.insert("name".into(), json!(parse_string(name)));
        Ok((input, obj))
    }
}
