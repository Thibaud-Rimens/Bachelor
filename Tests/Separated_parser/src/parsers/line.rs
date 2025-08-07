use nom::{IResult, bytes::complete::{take, tag}, character::complete::space1, combinator::rest, Parser};
use serde_json::{json, Map, Value};
use crate::parser::FileParser;
use crate::structures::{Line, HMap};
use crate::utils::{parse_color, parse_i32, parse_string};

#[derive(Default)]
pub struct LineParser {
    data: HMap,
}

impl LineParser {
    fn parse_name(input: &str) -> IResult<&str, &str> {
        let mut parser = (
            tag("K"),
            space1,
            rest
        );
        let (_input, (
            _,
            _,
            name
        )) = parser.parse(input)?;
        Ok((input, name))
    }

    fn parse_short_name(input: &str) -> IResult<&str, &str> {
        let mut parser = (
            tag("N T"),
            space1,
            rest
        );

        let (input, (_, _, short_name)) = parser.parse(input)?;
        Ok((input, short_name))
    }

    fn parse_long_name(input: &str) -> IResult<&str, &str> {
        let mut parser = (
            tag("L T"),
            space1,
            rest
        );

        let (input, (_, _, long_name)) = parser.parse(input)?;
        Ok((input, long_name))
    }

    fn parse_text_color(input: &str) -> IResult<&str, (&str, &str, &str)> {
        let mut parser = (
            tag("F"),
            space1,
            take(3usize),
            space1,
            take(3usize),
            space1,
            take(3usize),
        );

        let (input, (_, _, r, _, g, _, b)) = parser.parse(input)?;
        Ok((input, (r, g, b)))
    }

    fn parse_background_color(input: &str) -> IResult<&str, (&str, &str, &str)> {
        let mut parser = (
            tag("B"),
            space1,
            take(3usize),
            space1,
            take(3usize),
            space1,
            take(3usize),
        );

        let (input, (_, _, r, _, g, _, b)) = parser.parse(input)?;
        Ok((input, (r, g, b)))
    }
}

impl FileParser for LineParser {
    type Output = Line;

    fn parse_line<'a>(&mut self, input: &'a str) -> IResult<&'a str, Map<String, Value>> {
        let mut parser = (take(7usize), space1);
        let (input, (id, _)) = parser.parse(input)?;
        let id_string = id.to_string();

        let mut obj = match self.data.remove(&id_string) {
            Some(obj) => obj,
            None => {
                if let Ok((_, name)) = Self::parse_name(input) {
                    let mut new_obj = Map::new();
                    new_obj.insert("id".into(), json!(parse_i32(id)));
                    new_obj.insert("name".into(), json!(parse_string(name)));
                    new_obj
                } else {
                    panic!("Impossible de créer un objet : aucun nom trouvé !");
                }
            }
        };

        if let Ok((_, short_name)) = Self::parse_short_name(input) {
            obj.insert("short_name".into(), json!(parse_string(short_name)));
        } else if let Ok((_, long_name)) = Self::parse_long_name(input) {
            obj.insert("long_name".into(), json!(parse_string(long_name)));
        } else if let Ok((_, text_color)) = Self::parse_text_color(input) {
            obj.insert("text_color".into(), json!(parse_color(text_color)));
        } else if let Ok((_, background_color)) = Self::parse_background_color(input) {
            obj.insert("background_color".into(), json!(parse_color(background_color)));
        }

        self.data.insert(id_string, obj.clone());
        Ok((input, obj))

    }
}
