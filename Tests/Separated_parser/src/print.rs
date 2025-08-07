use serde::Serialize;

pub enum Output {
    Empty,
    Partial(usize),
    Full,
}

pub const OUTPUT: Output = Output::Partial(12);
//pub const OUTPUT: Output = Output::Full;

pub fn print_data<T: Serialize>(results: Vec<T>) {
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
