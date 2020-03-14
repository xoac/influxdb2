// https://v2.docs.influxdata.com/v2.0/reference/key-concepts/data-elements/#timestamp
// https://github.com/influxdata/influxdb/blob/master/models/time.go
use chrono::{
    format::{parse, Fixed, Item, ParseError, Parsed},
    NaiveDateTime,
};
use serde::{
    de::{Deserialize, Error},
    Serialize,
};
use std::str::FromStr;

const TIMESTAMP_FORMAT: &'static [Item<'static>] = &[Item::Fixed(Fixed::RFC3339)];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timestamp {
    //On disk, timestamps are stored in epoch nanosecond format.
    // inner: i64, This could be represented as i64 but NaiveDateTime give us extra functionality
    inner: NaiveDateTime,
}

impl Timestamp {
    #[inline]
    pub fn timestamp_nanos(&self) -> i64 {
        self.inner.timestamp_nanos()
    }
}

impl FromStr for Timestamp {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parsed = Parsed::new();
        parse(&mut parsed, s, TIMESTAMP_FORMAT.iter())?;
        parsed
            .to_naive_datetime_with_offset(0)
            .map(|inner| Timestamp { inner })

        //TODO check for max / min values!
    }
}

impl ToString for Timestamp {
    fn to_string(&self) -> String {
        format!("{}", self.inner.format_with_items(TIMESTAMP_FORMAT.iter()))
    }
}

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let as_str = self.to_string();
        serializer.serialize_str(&as_str)
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let str_time = String::deserialize(deserializer)?;
        Timestamp::from_str(&str_time).map_err(Error::custom)
    }
}
