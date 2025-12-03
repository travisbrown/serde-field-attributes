use chrono::{DateTime, TimeZone, Utc};
use serde::{
    de::{Deserialize, Deserializer, Unexpected},
    ser::Serializer,
};
use std::borrow::Cow;

const EXPECTED: &str = "optional epoch millisecond string";

pub fn deserialize<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error> {
    let timestamp_millis_str: Option<Cow<'_, str>> = Deserialize::deserialize(deserializer)?;

    timestamp_millis_str
        .map(|timestamp_millis_str| {
            let timestamp_millis = timestamp_millis_str.parse::<i64>().map_err(|_| {
                serde::de::Error::invalid_value(Unexpected::Str(&timestamp_millis_str), &EXPECTED)
            })?;

            let timestamp = Utc
                .timestamp_millis_opt(timestamp_millis)
                .single()
                .ok_or_else(|| {
                    serde::de::Error::invalid_value(
                        Unexpected::Str(&timestamp_millis_str),
                        &EXPECTED,
                    )
                })?;

            Ok(timestamp)
        })
        .map_or(Ok(None), |result| result.map(Some))
}

pub fn serialize<S: Serializer>(
    value: &Option<DateTime<Utc>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match value {
        Some(value) => serializer.serialize_str(&value.timestamp_millis().to_string()),
        None => serializer.serialize_none(),
    }
}
