use serde::{
    de::{Deserializer, Visitor},
    ser::Serializer,
};
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::str::FromStr;

const EXPECTED: &str = "optional integer string array";

pub fn deserialize<'de, E: FromStr, T: FromIterator<E>, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<T>, D::Error> {
    struct IntegerStrArrayOptVisitor<E, T> {
        _element: PhantomData<E>,
        _target: PhantomData<T>,
    }

    impl<'de, E: FromStr, T: FromIterator<E>> Visitor<'de> for IntegerStrArrayOptVisitor<E, T> {
        type Value = Option<T>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str(EXPECTED)
        }

        fn visit_none<EE: serde::de::Error>(self) -> Result<Self::Value, EE> {
            Ok(None)
        }

        fn visit_some<D: Deserializer<'de>>(
            self,
            deserializer: D,
        ) -> Result<Self::Value, D::Error> {
            super::integer_str_array::deserialize(deserializer).map(Some)
        }
    }

    deserializer.deserialize_option(IntegerStrArrayOptVisitor::<E, T> {
        _element: PhantomData,
        _target: PhantomData,
    })
}

pub fn serialize<'a, E: std::fmt::Display, T: 'a, S: Serializer>(
    values: &'a Option<T>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    &'a T: IntoIterator<Item = E>,
{
    match values {
        Some(values) => super::integer_str_array::serialize(values, serializer),
        None => serializer.serialize_none(),
    }
}
