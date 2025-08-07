use nom::{IResult, bytes::complete::take, character::complete::space1, combinator::opt, Parser};
use serde_json::{json, Map, Value};
use crate::parser::FileParser;
use crate::structures::ExchangeTimeJourney;
use crate::utils::{parse_i16, parse_i32, parse_option_i32, parse_string};

pub struct ExchangeTimeJourneyParser;

impl FileParser for ExchangeTimeJourneyParser {
    type Output = ExchangeTimeJourney;

    fn parse_line<'a>(&mut self, input: &'a str) -> IResult<&'a str, Map<String, Value>> {
        let mut parser = (
            take(7usize),
            space1,
            take(6usize),
            space1,
            take(6usize),
            space1,
            take(6usize),
            space1,
            take(6usize),
            space1,
            take(3usize),
            take(2usize),
            opt(take(6usize)),
        );

        let (input, (
            stop_id,
            _,
            journey_legacy_id_1,
            _,
            administration_1,
            _,
            journey_legacy_id_2,
            _,
            administration_2,
            _,
            duration,
            _,
            bit_field_id
        )) = parser.parse(input)?;

        let mut obj = Map::new();
        obj.insert("stop_id".into(), json!(parse_i32(stop_id)));
        obj.insert("journey_legacy_id_1".into(), json!(parse_i32(journey_legacy_id_1)));
        obj.insert("administration_1".into(), json!(parse_string(administration_1)));
        obj.insert("journey_legacy_id_2".into(), json!(parse_i32(journey_legacy_id_2)));
        obj.insert("administration_2".into(), json!(parse_string(administration_2)));
        obj.insert("duration".into(), json!(parse_i16(duration)));
        obj.insert("bit_field_id".into(), json!(parse_option_i32(bit_field_id)));
        Ok((input, obj))
    }
}
