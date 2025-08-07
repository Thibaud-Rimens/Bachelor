use std::fs::read_to_string;
use std::path::Path;
use std::io;

use nom::{IResult};
use serde::{Serialize};
use serde::de::DeserializeOwned;
use serde_json::{from_value, Map, Value};
use crate::print::print_data;

pub trait FileParser {
    type Output: Serialize + DeserializeOwned;

    fn parse_line<'a>(&mut self, input: &'a str) -> IResult<&'a str, Map<String, Value>>;

    fn convert_to_struct(&mut self, parsed: Map<String, Value>) -> Self::Output {
        from_value(Value::Object(parsed)).unwrap()
    }

    fn process_input(&mut self, input: &str) -> Result<Self::Output, String> {
        match self.parse_line(input) {
            Ok((_rest, parsed)) => Ok(self.convert_to_struct(parsed)),
            Err(e) => Err(format!("Parse error: {}", e)),
        }
    }

    // Good for test, not good for actual use
    fn read_file<'a>(&mut self, path: &Path, file: String) -> io::Result<()> {
        let mut results: Vec<Self::Output> = Vec::new();
        let filepath = path.join(file);

        for line in read_to_string(filepath)?.lines() {
            match self.process_input(line) {
                Ok(output) => results.push(output),
                Err(e) => {
                    eprintln!("Erreur lors du parsing de la ligne '{}': {}", line, e);
                    return Err(io::Error::new(io::ErrorKind::InvalidData, e));
                }
            }
        }

        print_data(results);
        Ok(())
    }
}
