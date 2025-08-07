use crate::structures::Color;

pub fn parse_i32(s: &str) -> i32 {
    s.trim().parse::<i32>().unwrap()
}

pub fn parse_i16(s: &str) -> i16 {
    s.trim().parse::<i16>().unwrap()
}

pub fn parse_string(s: &str) -> String {
    s.trim().to_string()
}

pub fn parse_option_i32(s: Option<&str>) -> Option<i32> {
    s.and_then(|v| v.trim().parse::<i32>().ok())
}

pub fn parse_vec_u8(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}

pub fn parse_color((r, g, b): (&str, &str, &str)) -> Color {
    Color::new(
        r.trim().parse::<i16>().unwrap(),
        g.trim().parse::<i16>().unwrap(),
        b.trim().parse::<i16>().unwrap()
    )
}

// TODO: verifier si c'est coherent avec ce qui est necessaire
pub fn parse_bool(s: &str) -> bool {
    s != " "
}