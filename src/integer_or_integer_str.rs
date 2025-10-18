use serde::{
    de::{Deserializer, Unexpected, Visitor},
    ser::Serializer,
};
use std::marker::PhantomData;
use std::str::FromStr;

const EXPECTED: &str = "integer or integer string";

// TODO: For simplicity and to keep the interface consistent with similar attributes provided here,
// this implementation relies on printing non-string values to strings and then parsing them. This
// imposes a performance cost that may not be acceptable in some contexts. At some point we should
// fix this.
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

        fn visit_u8<E: serde::de::Error>(self, v: u8) -> Result<Self::Value, E> {
            self.visit_str(&v.to_string())
        }

        fn visit_u16<E: serde::de::Error>(self, v: u16) -> Result<Self::Value, E> {
            self.visit_str(&v.to_string())
        }

        fn visit_u32<E: serde::de::Error>(self, v: u32) -> Result<Self::Value, E> {
            self.visit_str(&v.to_string())
        }

        fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
            self.visit_str(&v.to_string())
        }

        fn visit_u128<E: serde::de::Error>(self, v: u128) -> Result<Self::Value, E> {
            self.visit_str(&v.to_string())
        }

        fn visit_i8<E: serde::de::Error>(self, v: i8) -> Result<Self::Value, E> {
            self.visit_str(&v.to_string())
        }

        fn visit_i16<E: serde::de::Error>(self, v: i16) -> Result<Self::Value, E> {
            self.visit_str(&v.to_string())
        }

        fn visit_i32<E: serde::de::Error>(self, v: i32) -> Result<Self::Value, E> {
            self.visit_str(&v.to_string())
        }

        fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Self::Value, E> {
            self.visit_str(&v.to_string())
        }

        fn visit_i128<E: serde::de::Error>(self, v: i128) -> Result<Self::Value, E> {
            self.visit_str(&v.to_string())
        }
    }

    deserializer.deserialize_any(IntegerStrVisitor::<T> {
        _target: PhantomData,
    })
}

pub fn serialize<T: std::fmt::Display, S: Serializer>(
    value: &T,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&value.to_string())
}
