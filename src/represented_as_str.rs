use serde::{
    de::{Deserializer, Visitor},
    ser::Serializer,
};
use std::fmt::Display;
use std::marker::PhantomData;
use std::str::FromStr;

const EXPECTED: &str = "string representation";

pub fn deserialize<'de, T: FromStr, D: Deserializer<'de>>(deserializer: D) -> Result<T, D::Error>
where
    T::Err: Display,
{
    struct IntegerStrVisitor<T> {
        _target: PhantomData<T>,
    }

    impl<'de, T: FromStr> Visitor<'de> for IntegerStrVisitor<T>
    where
        T::Err: Display,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str(EXPECTED)
        }

        fn visit_borrowed_str<E: serde::de::Error>(self, v: &'de str) -> Result<Self::Value, E> {
            v.parse::<Self::Value>().map_err(serde::de::Error::custom)
        }

        fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
            v.parse::<Self::Value>().map_err(serde::de::Error::custom)
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
