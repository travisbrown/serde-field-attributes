//! An optional unsigned integer representation where `-1` indicates absence.
use serde::{
    de::{Deserializer, Unexpected, Visitor},
    ser::Serializer,
};

const EXPECTED: &str = "optional unsigned integer";

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<usize>, D::Error> {
    struct UsizeOptVisitor;

    impl Visitor<'_> for UsizeOptVisitor {
        type Value = Option<usize>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str(EXPECTED)
        }

        fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
            v.try_into()
                .map_err(|_| E::invalid_value(Unexpected::Unsigned(v), &EXPECTED))
                .map(Some)
        }

        fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Self::Value, E> {
            if v == -1 {
                Ok(None)
            } else {
                Err(E::invalid_value(Unexpected::Signed(v), &EXPECTED))
            }
        }
    }

    deserializer.deserialize_any(UsizeOptVisitor)
}

pub fn serialize<S: Serializer>(value: &Option<usize>, serializer: S) -> Result<S::Ok, S::Error> {
    match value {
        Some(value) => serializer.serialize_u64(*value as u64),
        None => serializer.serialize_i64(-1),
    }
}
