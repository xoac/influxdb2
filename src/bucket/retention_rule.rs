use serde::{de::Deserializer, Deserialize, Serialize, Serializer};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
enum RetentionType {
    Expire,
}

mod seconds {
    use super::*;
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let seconds = u64::deserialize(deserializer)?;
        Ok(Duration::from_secs(seconds))
    }

    pub fn serialize<S>(seconds: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let seconds = seconds.as_secs();
        serializer.serialize_u64(seconds)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RetentionRule {
    r#type: RetentionType,
    #[serde(with = "seconds")]
    every_seconds: Duration,
}
