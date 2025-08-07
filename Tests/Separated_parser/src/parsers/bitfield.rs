use nom::{IResult, bytes::complete::take, character::complete::space1, Parser};
use serde_json::{json, Map, Value};
use crate::parser::FileParser;
use crate::structures::BitField;
use crate::utils::{parse_i32, parse_vec_u8};

pub struct BitFieldParser;

impl FileParser for BitFieldParser {
    type Output = BitField;

    fn parse_line<'a>(&mut self, input: &'a str) -> IResult<&'a str, Map<String, Value>> {
        let mut parser = (
            take(6usize),
            space1,
            take(96usize)
        );

        let (input, (
            id,
            _,
            bits
        )) = parser.parse(input)?;

        let mut obj = Map::new();
        obj.insert("id".into(), json!(parse_i32(id)));
        obj.insert("bits".into(), json!(parse_vec_u8(bits)));
        Ok((input, obj))
    }
}
