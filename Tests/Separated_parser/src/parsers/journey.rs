use rustc_hash::FxHashMap;
use nom::bytes::complete::{tag, take};
use nom::character::complete::space1;
use nom::combinator::rest;
use nom::{IResult, Parser};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use crate::parser::FileParser;
use crate::structures::Line;

// region original code

/*
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Journey {
    id: i32,
    legacy_id: i32,
    administration: String,
    metadata: FxHashMap<JourneyMetadataType, Vec<JourneyMetadataEntry>>,
    route: Vec<JourneyRouteEntry>,
}

impl Journey {
    pub fn new(id: i32, legacy_id: i32, administration: String) -> Self {
        Self {
            id,
            legacy_id,
            administration,
            metadata: FxHashMap::default(),
            route: Vec::new(),
        }
    }

    // Getters/Setters

    pub fn administration(&self) -> &str {
        &self.administration
    }

    pub fn legacy_id(&self) -> i32 {
        self.legacy_id
    }

    fn metadata(&self) -> &FxHashMap<JourneyMetadataType, Vec<JourneyMetadataEntry>> {
        &self.metadata
    }

    pub fn route(&self) -> &Vec<JourneyRouteEntry> {
        &self.route
    }

    // Functions

    pub fn add_metadata_entry(&mut self, k: JourneyMetadataType, v: JourneyMetadataEntry) {
        self.metadata.entry(k).or_default().push(v);
    }

    pub fn add_route_entry(&mut self, entry: JourneyRouteEntry) {
        self.route.push(entry);
    }

    pub fn bit_field_id(&self) -> Option<i32> {
        // unwrap: There will always be a BitField entry.
        let entry = &self.metadata().get(&JourneyMetadataType::BitField).unwrap()[0];
        entry.bit_field_id
    }

    pub fn transport_type_id(&self) -> i32 {
        // unwrap: There will always be a TransportType entry.
        let entry = &self
            .metadata()
            .get(&JourneyMetadataType::TransportType)
            .unwrap()[0];
        // unwrap: It's guaranteed to have value here.
        entry.resource_id.unwrap()
    }

    pub fn transport_type<'a>(&'a self, data_storage: &'a DataStorage) -> &'a TransportType {
        data_storage
            .transport_types()
            .find(self.transport_type_id())
            .unwrap_or_else(|| panic!("Transport type {:?} not found.", self.transport_type_id()))
    }

    pub fn first_stop_id(&self) -> i32 {
        // unwrap: The route always contains at least 2 entries.
        self.route.first().unwrap().stop_id()
    }

    pub fn last_stop_id(&self) -> i32 {
        // unwrap: The route always contains at least 2 entries.
        self.route.last().unwrap().stop_id()
    }

    pub fn is_last_stop(&self, stop_id: i32, ignore_loop: bool) -> bool {
        if ignore_loop && self.first_stop_id() == self.last_stop_id() {
            false
        } else {
            stop_id == self.last_stop_id()
        }
    }

    pub fn count_stops(&self, departure_stop_id: i32, arrival_stop_id: i32) -> usize {
        self.route()
            .iter()
            .skip_while(|stop| stop.stop_id() != departure_stop_id)
            .take_while(|stop| stop.stop_id() != arrival_stop_id)
            .count()
            + 1
    }

    pub fn hash_route(&self, departure_stop_id: i32) -> Option<u64> {
        let index = self
            .route
            .iter()
            .position(|route_entry| route_entry.stop_id() == departure_stop_id)?;

        let mut hasher = DefaultHasher::new();
        self.route
            .iter()
            .skip(index)
            .map(|route_entry| route_entry.stop_id())
            .collect::<BTreeSet<_>>()
            .hash(&mut hasher);
        Some(hasher.finish())
    }

    /// unwrap: Do not call this function if the stop is not part of the route.
    /// unwrap: Do not call this function if the stop has no departure time (only the last stop has no departure time).
    pub fn departure_time_of(&self, stop_id: i32) -> (NaiveTime, bool) {
        let route = self.route();
        let index = route
            .iter()
            .position(|route_entry| route_entry.stop_id() == stop_id)
            .unwrap();
        let departure_time = route[index].departure_time().unwrap();

        (
            departure_time,
            // The departure time is on the next day if this evaluates to true.
            departure_time < route.first().unwrap().departure_time().unwrap(),
        )
    }

    /// The date must correspond to the route's first entry.
    /// Do not call this function if the stop is not part of the route.
    /// Do not call this function if the stop has no departure time (only the last stop has no departure time).
    pub fn departure_at_of(&self, stop_id: i32, date: NaiveDate) -> NaiveDateTime {
        match self.departure_time_of(stop_id) {
            (departure_time, false) => NaiveDateTime::new(date, departure_time),
            (departure_time, true) => NaiveDateTime::new(add_1_day(date), departure_time),
        }
    }

    /// The date must be associated with the origin_stop_id.
    /// Do not call this function if the stop is not part of the route.
    pub fn departure_at_of_with_origin(
        &self,
        stop_id: i32,
        date: NaiveDate,
        // If it's not a departure date, it's an arrival date.
        is_departure_date: bool,
        origin_stop_id: i32,
    ) -> NaiveDateTime {
        let (departure_time, is_next_day) = self.departure_time_of(stop_id);
        let (_, origin_is_next_day) = if is_departure_date {
            self.departure_time_of(origin_stop_id)
        } else {
            self.arrival_time_of(origin_stop_id)
        };

        match (is_next_day, origin_is_next_day) {
            (true, false) => NaiveDateTime::new(add_1_day(date), departure_time),
            (false, true) => NaiveDateTime::new(sub_1_day(date), departure_time),
            _ => NaiveDateTime::new(date, departure_time),
        }
    }

    /// unwrap: Do not call this function if the stop is not part of the route.
    /// unwrap: Do not call this function if the stop has no arrival time (only the first stop has no arrival time).
    pub fn arrival_time_of(&self, stop_id: i32) -> (NaiveTime, bool) {
        let route = self.route();
        let index = route
            .iter()
            // The first route entry has no arrival time.
            .skip(1)
            .position(|route_entry| route_entry.stop_id() == stop_id)
            .map(|i| i + 1)
            .unwrap();
        let arrival_time = route[index].arrival_time().unwrap();

        (
            arrival_time,
            // The arrival time is on the next day if this evaluates to true.
            arrival_time < route.first().unwrap().departure_time().unwrap(),
        )
    }

    /// The date must be associated with the origin_stop_id.
    pub fn arrival_at_of_with_origin(
        &self,
        stop_id: i32,
        date: NaiveDate,
        // If it's not a departure date, it's an arrival date.
        is_departure_date: bool,
        origin_stop_id: i32,
    ) -> NaiveDateTime {
        let (arrival_time, is_next_day) = self.arrival_time_of(stop_id);
        let (_, origin_is_next_day) = if is_departure_date {
            self.departure_time_of(origin_stop_id)
        } else {
            self.arrival_time_of(origin_stop_id)
        };

        match (is_next_day, origin_is_next_day) {
            (true, false) => NaiveDateTime::new(add_1_day(date), arrival_time),
            (false, true) => NaiveDateTime::new(sub_1_day(date), arrival_time),
            _ => NaiveDateTime::new(date, arrival_time),
        }
    }

    /// Excluding departure stop.
    pub fn route_section(
        &self,
        departure_stop_id: i32,
        arrival_stop_id: i32,
    ) -> Vec<&JourneyRouteEntry> {
        let mut route_iter = self.route().iter();

        for route_entry in route_iter.by_ref() {
            if route_entry.stop_id() == departure_stop_id {
                break;
            }
        }

        let mut result = Vec::new();

        for route_entry in route_iter {
            result.push(route_entry);

            if route_entry.stop_id() == arrival_stop_id {
                break;
            }
        }

        result
    }
}

fn journey_row_parser() -> RowParser {
    RowParser::new(vec![
        // This row is used to create a Journey instance.
        RowDefinition::new(
            RowType::RowA as i32,
            Box::new(FastRowMatcher::new(1, 2, "*Z", true)),
            vec![
                ColumnDefinition::new(4, 9, ExpectedType::Integer32),
                ColumnDefinition::new(11, 16, ExpectedType::String),
            ],
        ),
        RowDefinition::new(
            RowType::RowB as i32,
            Box::new(FastRowMatcher::new(1, 2, "*G", true)),
            vec![
                ColumnDefinition::new(4, 6, ExpectedType::String),
                ColumnDefinition::new(8, 14, ExpectedType::OptionInteger32),
                ColumnDefinition::new(16, 22, ExpectedType::OptionInteger32),
            ],
        ),
        RowDefinition::new(
            RowType::RowC as i32,
            Box::new(FastRowMatcher::new(1, 5, "*A VE", true)),
            vec![
                ColumnDefinition::new(7, 13, ExpectedType::OptionInteger32),
                ColumnDefinition::new(15, 21, ExpectedType::OptionInteger32),
                ColumnDefinition::new(23, 28, ExpectedType::OptionInteger32),
            ],
        ),
        RowDefinition::new(
            RowType::RowD as i32,
            Box::new(FastRowMatcher::new(1, 2, "*A", true)),
            vec![
                ColumnDefinition::new(4, 5, ExpectedType::String),
                ColumnDefinition::new(7, 13, ExpectedType::OptionInteger32),
                ColumnDefinition::new(15, 21, ExpectedType::OptionInteger32),
            ],
        ),
        RowDefinition::new(
            RowType::RowE as i32,
            Box::new(FastRowMatcher::new(1, 2, "*I", true)),
            vec![
                ColumnDefinition::new(4, 5, ExpectedType::String),
                ColumnDefinition::new(7, 13, ExpectedType::OptionInteger32),
                ColumnDefinition::new(15, 21, ExpectedType::OptionInteger32),
                ColumnDefinition::new(23, 28, ExpectedType::OptionInteger32),
                ColumnDefinition::new(30, 38, ExpectedType::Integer32),
                ColumnDefinition::new(40, 45, ExpectedType::OptionInteger32),
                ColumnDefinition::new(47, 52, ExpectedType::OptionInteger32),
            ],
        ),
        RowDefinition::new(
            RowType::RowF as i32,
            Box::new(FastRowMatcher::new(1, 2, "*L", true)),
            vec![
                ColumnDefinition::new(4, 11, ExpectedType::String),
                ColumnDefinition::new(13, 19, ExpectedType::OptionInteger32),
                ColumnDefinition::new(21, 27, ExpectedType::OptionInteger32),
                ColumnDefinition::new(29, 34, ExpectedType::OptionInteger32),
                ColumnDefinition::new(36, 41, ExpectedType::OptionInteger32),
            ],
        ),
        RowDefinition::new(
            RowType::RowG as i32,
            Box::new(FastRowMatcher::new(1, 2, "*R", true)),
            vec![
                ColumnDefinition::new(4, 4, ExpectedType::String),
                ColumnDefinition::new(6, 12, ExpectedType::String),
                ColumnDefinition::new(14, 20, ExpectedType::OptionInteger32),
                ColumnDefinition::new(22, 28, ExpectedType::OptionInteger32),
                ColumnDefinition::new(30, 35, ExpectedType::OptionInteger32),
                ColumnDefinition::new(37, 42, ExpectedType::OptionInteger32),
            ],
        ),
        // *CI
        RowDefinition::new(
            RowType::RowH as i32,
            Box::new(FastRowMatcher::new(1, 3, "*CI", true)),
            vec![
                ColumnDefinition::new(1, 3, ExpectedType::String),
                ColumnDefinition::new(5, 8, ExpectedType::Integer32),
                ColumnDefinition::new(10, 16, ExpectedType::OptionInteger32),
                ColumnDefinition::new(18, 24, ExpectedType::OptionInteger32),
            ],
        ),
        // *CO
        RowDefinition::new(
            RowType::RowH as i32,
            Box::new(FastRowMatcher::new(1, 3, "*CO", true)),
            vec![
                ColumnDefinition::new(1, 3, ExpectedType::String),
                ColumnDefinition::new(5, 8, ExpectedType::Integer32),
                ColumnDefinition::new(10, 16, ExpectedType::OptionInteger32),
                ColumnDefinition::new(18, 24, ExpectedType::OptionInteger32),
            ],
        ),
        RowDefinition::new(
            RowType::RowI as i32,
            Box::new(FastRowMatcher::new(1, 0, "", true)),
            vec![
                ColumnDefinition::new(1, 7, ExpectedType::Integer32),
                ColumnDefinition::new(30, 35, ExpectedType::OptionInteger32),
                ColumnDefinition::new(37, 42, ExpectedType::OptionInteger32),
            ],
        ),
    ])
}

*/

// endregion original code

type HMap = FxHashMap<String, Map<String, Value>>;
#[derive(Default)]
pub struct FPlanParser {
    data: HMap,
}


impl FPlanParser {
    // TODO: Create enough line types
    fn parse_fplan_1/*TODO : name*/(input: &str) -> IResult<&str, ()> {
        let mut parser = (
            tag("*Z"),
            space1,
            take(6usize), // i32
            space1,
            take(6usize), // string
        );

        let (input, _) = parser.parse(input)?;
        Ok((input, ()))
    }
    fn parse_fplan_2/*TODO : name*/(input: &str) -> IResult<&str, ()> {
        let mut parser = (
            tag("*G"),
            space1,
            take(3usize), // string
            space1,
            take(7usize), // option i32
            space1,
            take(7usize), // option i32
        );

        let (input, _) = parser.parse(input)?;
        Ok((input, ()))
    }
    fn parse_fplan_3/*TODO : name*/(input: &str) -> IResult<&str, ()> {
        let mut parser = (
            tag("*A VE"),
            space1,
            take(7usize), // option i32
            space1,
            take(7usize), // option i32
            space1,
            take(6usize), // option i32
        );

        let (input, _) = parser.parse(input)?;
        Ok((input, ()))
    }
    fn parse_fplan_4/*TODO : name*/(input: &str) -> IResult<&str, ()> {
        let mut parser = (
            tag("*A"),
            space1,
            take(2usize), // string
            space1,
            take(7usize), // option i32
            space1,
            take(7usize), // option i32
        );

        let (input, _) = parser.parse(input)?;
        Ok((input, ()))
    }
    fn parse_fplan_5/*TODO : name*/(input: &str) -> IResult<&str, ()> {
        let mut parser = (
            tag("*I"),
            space1,
            take(2usize), // string
            space1,
            take(7usize), // option i32
            space1,
            take(7usize), // option i32
            space1,
            take(6usize), // option i32
            space1,
            take(9usize), // i32
            space1,
            take(6usize), // option i32
            space1,
            take(6usize), // option i32
        );

        let (input, _) = parser.parse(input)?;
        Ok((input, ()))
    }
    fn parse_fplan_6/*TODO : name*/(input: &str) -> IResult<&str, ()> {
        let mut parser = (
            tag("*L"),
            space1,
            take(8usize), // string
            space1,
            take(7usize), // option i32
            space1,
            take(7usize), // option i32
            space1,
            take(6usize), // option i32
            space1,
            take(6usize), // option i32
        );

        let (input, _) = parser.parse(input)?;
        Ok((input, ()))
    }
    fn parse_fplan_7/*TODO : name*/(input: &str) -> IResult<&str, ()> {
        let mut parser = (
            tag("*R"),
            space1,
            take(1usize), // string
            space1,
            take(7usize), // string
            space1,
            take(7usize), // option i32
            space1,
            take(7usize), // option i32
            space1,
            take(6usize), // option i32
            space1,
            take(6usize), // option i32
        );

        let (input, _) = parser.parse(input)?;
        Ok((input, ()))
    }
    fn parse_fplan_8/*TODO : name*/(input: &str) -> IResult<&str, ()> {
        let mut parser = (
            tag("*CI"), // see if need to be changed
            space1,
            take(4usize), // i32
            space1,
            take(7usize), // option i32
            space1,
            take(7usize), // option i32
        );

        let (input, _) = parser.parse(input)?;
        Ok((input, ()))
    }
    fn parse_fplan_9/*TODO : name*/(input: &str) -> IResult<&str, ()> {
        let mut parser = (
            tag("*CO"), // see if need to be changed
            space1,
            take(4usize), // i32
            space1,
            take(7usize), // option i32
            space1,
            take(7usize), // option i32
        );

        let (input, _) = parser.parse(input)?;
        Ok((input, ()))
    }
    fn parse_fplan_10/*TODO : name*/(input: &str) -> IResult<&str, ()> {
        let mut parser = (
            take(7usize), // i32
            space1,
            take(6usize), // option i32
            space1,
            take(6usize), // option i32
        );

        let (input, _) = parser.parse(input)?;
        Ok((input, ()))
    }
}

impl FileParser for FPlanParser {
    type Output = Line;

    fn parse_line<'a>(&mut self, input: &'a str) -> IResult<&'a str, Map<String, Value>> {
        // TODO: fplan obj code
        Ok((input, Map::default()))
    }
}
