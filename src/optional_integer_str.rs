use serde::{
    de::{Deserializer, Visitor},
    ser::Serializer,
};
use std::marker::PhantomData;
use std::str::FromStr;

const EXPECTED: &str = "optional integer string";

pub fn deserialize<'de, T: FromStr, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<T>, D::Error> {
    struct IntegerStrOptVisitor<T> {
        _target: PhantomData<T>,
    }

    impl<'de, T: FromStr> Visitor<'de> for IntegerStrOptVisitor<T> {
        type Value = Option<T>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str(EXPECTED)
        }

        fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_some<D: Deserializer<'de>>(
            self,
            deserializer: D,
        ) -> Result<Self::Value, D::Error> {
            super::integer_str::deserialize(deserializer).map(Some)
        }
    }

    deserializer.deserialize_option(IntegerStrOptVisitor::<T> {
        _target: PhantomData,
    })
}

pub fn serialize<T: std::fmt::Display, S: Serializer>(
    value: &Option<T>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match value {
        Some(value) => serializer.serialize_str(&value.to_string()),
        None => serializer.serialize_none(),
    }
}
