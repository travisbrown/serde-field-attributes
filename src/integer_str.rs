use serde::{
    de::{Deserializer, Unexpected, Visitor},
    ser::Serializer,
};
use std::marker::PhantomData;
use std::str::FromStr;

const EXPECTED: &str = "integer string";

pub fn deserialize<'de, T: FromStr, D: Deserializer<'de>>(deserializer: D) -> Result<T, D::Error> {
    struct IntegerStrVisitor<T> {
        _target: PhantomData<T>,
    }

    impl<'de, T: FromStr> Visitor<'de> for IntegerStrVisitor<T> {
        type Value = T;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str(EXPECTED)
        }

        fn visit_borrowed_str<E: serde::de::Error>(self, v: &'de str) -> Result<Self::Value, E> {
            v.parse::<Self::Value>()
                .map_err(|_| serde::de::Error::invalid_value(Unexpected::Str(v), &EXPECTED))
        }

        fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
            v.parse::<Self::Value>()
                .map_err(|_| serde::de::Error::invalid_value(Unexpected::Str(v), &EXPECTED))
        }
    }

    deserializer.deserialize_str(IntegerStrVisitor::<T> {
        _target: PhantomData,
    })
}

pub fn serialize<T: std::fmt::Display, S: Serializer>(
    value: &T,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&value.to_string())
}
