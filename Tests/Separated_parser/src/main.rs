#![feature(associated_type_defaults)]

mod print;
mod parser;
mod parsers;
mod utils;
mod structures;

use std::env;
use std::path::Path;
use parsers::{DirectionParser, BitFieldParser, ExchangeTimeJourneyParser, LineParser};
use crate::parser::FileParser;

fn main() -> std::io::Result<()>  {
    let args: Vec<String> = env::args().collect();

    println!("argc: {}", args.len());
    if args.len() <= 1 {
        panic!("Argument missing")
    }

    let filepath = Path::new(&args[1]);

    let mut direction_parser = DirectionParser;
    let mut bitfield_parser = BitFieldParser;
    let mut exchange_time_journey_parser = ExchangeTimeJourneyParser;
    let mut line_parser = LineParser::default(); // Necessary because of hashmap

    direction_parser.read_file(&filepath, String::from("RICHTUNG"))?;
    bitfield_parser.read_file(&filepath, String::from("BITFELD"))?;
    exchange_time_journey_parser.read_file(&filepath, String::from("UMSTEIGZ"))?;
    line_parser.read_file(&filepath, String::from("LINIE"))?;
    Ok(())
}
