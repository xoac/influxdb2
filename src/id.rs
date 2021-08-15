use serde::{
    de::{Deserialize, Error},
    Serialize,
};
/// Represent ID
use std::str::FromStr;

use thiserror::Error;

const ID_LEN: usize = 16;

/// Error represent Id Parse problems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
pub enum IdErr {
    #[error("couldn't parse string as correct ID")]
    Parse,
    #[error("ID = 0 is incorrect")]
    ZeroId,
}

/// Represent Influx ID
///
/// This is orgID and ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(u64);

impl FromStr for Id {
    type Err = IdErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != ID_LEN {
            Err(IdErr::Parse)
        } else {
            let v = u64::from_str_radix(s, 16).map_err(|_| IdErr::Parse)?;
            if v == 0 {
                Err(IdErr::ZeroId)
            } else {
                Ok(Id(v))
            }
        }
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let as_string = String::deserialize(deserializer)?;
        Id::from_str(&as_string).map_err(D::Error::custom)
    }
}

impl ToString for Id {
    fn to_string(&self) -> String {
        format!("{:016X}", self.0)
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_zero_id() {
        let id_res = "0000000000000000".parse::<Id>();
        assert_eq!(id_res.unwrap_err(), IdErr::ZeroId);
    }

    #[test]
    fn parse_max_id() {
        let _id = "ffffffffffffffff"
            .parse::<Id>()
            .expect("Should be correct ID");
    }

    #[test]
    fn parse_too_long_string() {
        let id_res = "abcdabcdabcdabcd0".parse::<Id>();
        assert_eq!(id_res.unwrap_err(), IdErr::Parse);
    }

    #[test]
    fn parse_too_short_string() {
        let id_res = "abc".parse::<Id>();
        assert_eq!(id_res.unwrap_err(), IdErr::Parse);
    }
}
