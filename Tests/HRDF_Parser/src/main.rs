use std::env;
use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;
use serde::Serialize;
use nom::{IResult, bytes::complete::{take}, character::complete::space1, Parser};
use nom::bytes::complete::tag;
use nom::combinator::{opt, rest};
use rustc_hash::FxHashMap;

enum Output {
    Empty,
    Partial(usize),
    Full
}

const OUTPUT: Output = Output::Partial(12);

fn print_data<T: Serialize>(results: Vec<T>) {
    match OUTPUT {
        Output::Partial(n) => {
            let result = &results[n];
            let string = serde_json::to_string_pretty(result).unwrap();
            println!("\n{}", string);
        }
        Output::Full => {
            for result in &results {
                let string = serde_json::to_string_pretty(result).unwrap();
                println!("\n{}", string);
            }
        }
        _ => {}
    }
}

#[derive(Serialize)]
struct Direction {
    id: String,
    name: String,
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let mut parser = (take(7usize), space1, rest);
    let (input, (id, _, name)) = parser.parse(input)?;

    Ok((input, Direction {
        id: id.to_string(),
        name: name.trim().to_string(),
    }))
}

fn read_direction(f: &Path) -> std::io::Result<()> {
    let mut results : Vec<Direction> = Vec::new();

    let filepath = f.join("RICHTUNG");
    for line in read_to_string(filepath)?.lines() {
        results.push(parse_direction(line).unwrap().1);
    }

    print_data(results);
    Ok(())
}

#[derive(Serialize)]
pub struct BitField {
    id: i32,
    bits: Vec<u8>,
}

fn parse_bitfield(input: &str) -> IResult<&str, BitField> {
    let mut parser = (take(6usize), space1, take(96usize));
    let (input, (id, _, bits)) = parser.parse(input)?;

    // ID
    let id = i32::from_str(id).unwrap();

    // Conversion hex → Vec<u8>
    let bits = bits
        .as_bytes()
        .chunks(2)
        .map(|chunk| {
            let s = std::str::from_utf8(chunk).unwrap();
            u8::from_str_radix(s, 16).unwrap()
        })
        .collect::<Vec<u8>>();

    Ok((input, BitField { id, bits }))
}

fn read_bitfield(f: &Path) -> std::io::Result<()>{
    let mut results : Vec<BitField> = Vec::new();

    let filepath = f.join("BITFELD");
    for line in read_to_string(filepath)?.lines() {
        results.push(parse_bitfield(line).unwrap().1);
    }

    print_data(results);
    Ok(())
}

#[derive(Serialize)]
struct ExchangeTimeJourney {
    stop_id: i32,
    journey_legacy_id_1: i32,
    administration_1: String,
    journey_legacy_id_2: i32,
    administration_2: String,
    duration: i16,
    // is_guaranteed: bool, // Removed for convenience
    bit_field_id: Option<i32>,
}

fn parse_exchange_journey(input: &str) -> IResult<&str, ExchangeTimeJourney> {
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
        bit_field_id,
    )) = parser.parse(input)?;

    Ok((input, ExchangeTimeJourney {
        stop_id: stop_id.parse().unwrap(),
        journey_legacy_id_1: journey_legacy_id_1.parse().unwrap(),
        administration_1: administration_1.to_string(),
        journey_legacy_id_2: journey_legacy_id_2.parse().unwrap(),
        administration_2: administration_2.to_string(),
        duration: duration.parse().unwrap(),
        bit_field_id: bit_field_id
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .and_then(|s| s.parse::<i32>().ok()),
    }))
}

fn read_exchange_journey(f: &Path) -> std::io::Result<()>{
    let mut results : Vec<ExchangeTimeJourney> = Vec::new();

    let filepath = f.join("UMSTEIGZ");
    for line in read_to_string(filepath)?.lines() {
        results.push(parse_exchange_journey(line).unwrap().1);
    }

    print_data(results);
    Ok(())
}

#[derive(Default, Serialize, Clone)]
struct Color {
    r: i16,
    g: i16,
    b: i16
}

#[derive(Serialize, Clone)]
struct Line {
    id: i32,
    name: String,
    short_name: String,
    long_name: String,
    text_color: Color,
    background_color: Color,
}

impl Line {
    pub fn new(id: i32) -> Self {
        Line {
            id,
            name: String::default(),
            short_name: String::default(),
            long_name: String::default(),
            text_color: Color::default(),
            background_color: Color::default(),
        }
    }
}

fn parse_line_name(input: &str) -> IResult<&str, String> {
    let mut parser = (
        tag("K"),
        space1,
        rest
    );

    let (input, (_, _, name)) = parser.parse(input)?;
    Ok((input, name.trim().to_string()))
}

fn parse_line_short_name(input: &str) -> IResult<&str, String> {
    let mut parser = (
        tag("N T"),
        space1,
        rest
    );

    let (input, (_, _, short_name)) = parser.parse(input)?;
    Ok((input, short_name.trim().to_string()))
}

fn parse_line_long_name(input: &str) -> IResult<&str, String> {
    let mut parser = (
        tag("L T"),
        space1,
        rest
    );

    let (input, (_, _, long_name)) = parser.parse(input)?;
    Ok((input, long_name.trim().to_string()))
}

fn parse_line_text_color(input: &str) -> IResult<&str, Color> {
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
    Ok((input, Color {
        r: r.parse().unwrap(),
        g: g.parse().unwrap(),
        b: b.parse().unwrap()
    }))
}

fn parse_line_background_color(input: &str) -> IResult<&str, Color> {
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
    Ok((input, Color {
        r: r.parse().unwrap(),
        g: g.parse().unwrap(),
        b: b.parse().unwrap()
    }))
}


fn parse_line<'a>(input: &'a str, lines: &mut FxHashMap<i32, Line>) -> IResult<&'a str, Line> {
    let mut parser = (take(7usize), space1);
    let (input, (id, _)) = parser.parse(input)?;
    let id = i32::from_str(id).unwrap();
    let line = lines.entry(id).or_insert_with(|| Line::new(id));

    if let Ok((_, name)) = parse_line_name(input) {
        line.name = name.to_string();
    } else if let Ok((_, short_name)) = parse_line_short_name(input) {
        line.short_name = short_name.to_string();
    } else if let Ok((_, long_name)) = parse_line_long_name(input) {
        line.long_name = long_name.to_string();
    } else if let Ok((_, text_color)) = parse_line_text_color(input) {
        line.text_color = text_color;
    } else if let Ok((_, background_color)) = parse_line_background_color(input) {
        line.background_color = background_color;
    }

    Ok((input, line.clone()))
}

fn read_line(f: &Path) -> std::io::Result<()>{
    let mut lines: FxHashMap<i32, Line> = FxHashMap::default();

    let filepath = f.join("LINIE");
    for line in read_to_string(filepath)?.lines() {
        _ = parse_line(line, &mut lines);
    }

    let results: Vec<Line> = lines.into_values().collect();
    print_data(results);
    Ok(())
}

fn main() -> std::io::Result<()>  {
    let args: Vec<String> = env::args().collect();

    println!("argc: {}", args.len());
    if args.len() <= 1 {
        panic!("Argument missing")
    }

    // Test for accessing elements in a vector
    /*
    for i in 0..10 {
        result.push(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs().to_string());
        sleep(Duration::new(1,0));
        println!("{}", result[result.len() - 1])
    }
    */

    let filepath = Path::new(&args[1]);
    read_direction(filepath)?;
    read_bitfield(filepath)?;
    read_exchange_journey(filepath)?;
    read_line(filepath)?;
    
    Ok(())
}

