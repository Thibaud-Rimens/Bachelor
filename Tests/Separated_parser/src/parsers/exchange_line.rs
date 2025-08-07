use nom::{IResult, bytes::complete::{take}, character::complete::space1, combinator::rest, Parser};
use nom::combinator::opt;
use serde_json::{json, Map, Value};
use crate::parser::FileParser;
use crate::structures::{ExchangeTimeLine};
use crate::utils::{parse_bool, parse_i16, parse_i32, parse_option_i32, parse_string};

#[derive(Default)]
pub struct ExchangeTimeLineParser {
    current_id: i32
}

impl ExchangeTimeLineParser {
    fn get_current_id(&mut self) -> i32 {
        self.current_id += 1;
        self.current_id - 1
    }
}

impl FileParser for  ExchangeTimeLineParser {
    type Output =  ExchangeTimeLine;

    fn parse_line<'a>(&mut self, input: &'a str) -> IResult<&'a str, Map<String, Value>> {
        let mut parser = (
            opt(take(7usize)), // option i32
            space1,
            take(6usize), // string
            space1,
            take(3usize), // string
            space1,
            take(8usize), // string
            space1,
            take(1usize), // string
            space1,
            take(6usize), // string
            space1,
            take(3usize), // string
            space1,
            take(8usize), // string
            space1,
            take(1usize), // string
            space1,
            take(3usize), // i16
            space1,
            take(1usize), // string
        );

        let (input, (
            stop_id,
            _,
            administration_1,
            _,
            transport_type_id_1,
            _,
            line_id_1,
            _,
            direction_1,
            _,
            administration_2,
            _,
            transport_type_id_2,
            _,
            line_id_2,
            _,
            direction_2,
            _,
            duration,
            _,
            is_guaranteed,
        )) = parser.parse(input)?;

        let mut obj = Map::new();
        obj.insert("id".into(), json!(self.get_current_id()));
        obj.insert("stop_id".into(), json!(parse_option_i32(stop_id)));
        obj.insert("administration_1".into(), json!(parse_string(administration_1)));
        obj.insert("administration_2".into(), json!(parse_string(administration_2)));
        obj.insert("duration".into(), json!(parse_i16(duration)));
        obj.insert("is_guaranteed".into(), json!(parse_bool(is_guaranteed)));
        Ok((input, obj))
    }
}