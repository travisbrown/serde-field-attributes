use chrono::{DateTime, TimeZone, Utc};
use serde::{
    de::{Deserialize, Deserializer, Unexpected},
    ser::Serializer,
};
use std::borrow::Cow;

const EXPECTED: &str = "epoch millisecond string";

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<DateTime<Utc>, D::Error> {
    let timestamp_millis_str: Cow<'_, str> = Deserialize::deserialize(deserializer)?;

    let timestamp_millis = timestamp_millis_str.parse::<i64>().map_err(|_| {
        serde::de::Error::invalid_value(Unexpected::Str(&timestamp_millis_str), &EXPECTED)
    })?;

    let timestamp = Utc
        .timestamp_millis_opt(timestamp_millis)
        .single()
        .ok_or_else(|| {
            serde::de::Error::invalid_value(Unexpected::Str(&timestamp_millis_str), &EXPECTED)
        })?;

    Ok(timestamp)
}

pub fn serialize<S: Serializer>(value: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&value.timestamp_millis().to_string())
}
