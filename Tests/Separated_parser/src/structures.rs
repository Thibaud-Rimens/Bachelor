use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

#[derive(Serialize, Deserialize, Debug)]
pub struct Direction {
    id: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BitField {
    id: i32,
    bits: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExchangeTimeJourney {
    stop_id: i32,
    journey_legacy_id_1: i32,
    administration_1: String,
    journey_legacy_id_2: i32,
    administration_2: String,
    duration: i16,
    // is_guaranteed: bool, // Removed for convenience
    bit_field_id: Option<i32>,
}

#[derive(Default, Deserialize, Serialize, Clone, Debug)]
pub struct Color {
    r: i16,
    g: i16,
    b: i16,
}

impl Color {
    pub fn new(r: i16, g: i16, b: i16) -> Self {
        Color {r, g, b}
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Line {
    id: i32,
    name: String,
    short_name: Option<String>,
    long_name: Option<String>,
    text_color: Option<Color>,
    background_color: Option<Color>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute {
    id: i32,
    designation: String,
    stop_scope: i16,
    main_sorting_priority: i16,
    secondary_sorting_priority: i16,
    description: FxHashMap<Language, String>,
}

#[derive(
    Clone, Copy, Debug, Default, Display, Eq, Hash, PartialEq, Serialize, Deserialize,
)]
pub enum Language {
    #[default]
    #[strum(serialize = "deu")]
    German,

    #[strum(serialize = "fra")]
    French,

    #[strum(serialize = "ita")]
    Italian,

    #[strum(serialize = "eng")]
    English,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeTimeAdministration {
    id: i32,
    stop_id: Option<i32>, // A None value means that the exchange time applies to all stops if there is no specific entry for the stop and the 2 administrations.
    administration_1: String,
    administration_2: String,
    duration: i16, // Exchange time from administration 1 to administration 2 is in minutes.
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeTimeLine {
    id: i32,
    stop_id: Option<i32>,
    line_1: LineInfo,
    line_2: LineInfo,
    duration: i16, // Exchange time from line 1 to line 2 is in minutes.
    is_guaranteed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct LineInfo {
    administration: String,
    transport_type_id: i32,
    line_id: Option<String>,
    direction: Option<DirectionType>,
}

#[derive(
    Clone, Copy, Debug, Default, Display, Eq, Hash, PartialEq, Serialize, Deserialize,
)]
pub enum DirectionType {
    #[default]
    #[strum(serialize = "R")]
    Outbound,

    #[strum(serialize = "H")]
    Return,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Holiday {
    id: i32,
    date: NaiveDate,
    name: FxHashMap<Language, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InformationText {
    id: i32,
    content: FxHashMap<Language, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JourneyPlatform {
    journey_legacy_id: i32,
    administration: String,
    platform_id: i32,
    time: Option<NaiveTime>,
    bit_field_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stop {
    id: i32,
    name: String,
    long_name: Option<String>,
    abbreviation: Option<String>,
    synonyms: Option<Vec<String>>,
    lv95_coordinates: Option<Coordinates>,
    wgs84_coordinates: Option<Coordinates>,
    exchange_priority: i16,
    exchange_flag: i16,
    exchange_time: Option<(i16, i16)>, // (InterCity exchange time, Exchange time for all other journey types)
    restrictions: i16,
    sloid: String,
    boarding_areas: Vec<String>,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct Coordinates {
    coordinate_system: CoordinateSystem,
    x: f64,
    y: f64,
}

#[derive(Clone, Copy, Debug, Default, Display, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum CoordinateSystem {
    #[default]
    LV95,
    WGS84,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StopConnection {
    id: i32,
    stop_id_1: i32,
    stop_id_2: i32,
    duration: i16, // Exchange time from stop 1 to stop 2 is in minutes.
    attribute: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThroughService {
    id: i32,
    journey_1_id: JourneyId,
    journey_1_stop_id: i32, // Last stop of journey 1.
    journey_2_id: JourneyId,
    journey_2_stop_id: i32, // First stop of journey 2.
    bit_field_id: i32,
}

pub(crate) type JourneyId = (i32, String); // (legacy_id, administration)

#[derive(Debug, Serialize, Deserialize)]
pub struct TimetableMetadataEntry {
    id: i32,
    key: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportCompany {
    id: i32,
    short_name: FxHashMap<Language, String>,
    long_name: FxHashMap<Language, String>,
    full_name: FxHashMap<Language, String>,
    administrations: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TransportType {
    id: i32,
    designation: String,
    product_class_id: i16,
    tarrif_group: String,
    output_control: i16,
    short_name: String,
    surchage: i16,
    flag: String,
    product_class_name: FxHashMap<Language, String>,
    category_name: FxHashMap<Language, String>,
}
