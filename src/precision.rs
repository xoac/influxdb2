use serde::{Serialize, Serializer};
use std::str::FromStr;

pub struct ParsePrecisionErr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Precision {
    Milli,
    Secs,
    Micro,
    Nano,
}

impl Serialize for Precision {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let as_str = self.to_string();
        serializer.serialize_str(&as_str)
    }
}

impl Default for Precision {
    fn default() -> Self {
        Self::Nano
    }
}

impl ToString for Precision {
    fn to_string(&self) -> String {
        match self {
            Precision::Milli => "ms",
            Precision::Secs => "s",
            Precision::Micro => "us",
            Precision::Nano => "ns",
        }
        .to_string()
    }
}

impl FromStr for Precision {
    type Err = ParsePrecisionErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "ms" => Precision::Milli,
            "s" => Precision::Secs,
            "us" => Precision::Micro,
            "ns" => Precision::Nano,
            _ => return Err(ParsePrecisionErr),
        })
    }
}
