use serde::de::{Deserializer, Visitor};
use std::cell::OnceCell;

pub mod empty_string_or;
pub mod space_string_or;
pub mod spaces_string_or;

struct StringCheckDeserializer<'a, D, P: FnOnce(&str) -> bool> {
    inner: D,
    predicate: P,
    is_hit: &'a OnceCell<bool>,
}

impl<'de, D: serde::de::Deserializer<'de>, P: FnOnce(&str) -> bool + Copy>
    serde::de::Deserializer<'de> for StringCheckDeserializer<'_, D, P>
{
    type Error = D::Error;

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let wrapper = StringCheckVisitor {
            inner: visitor,
            predicate: self.predicate,
            is_hit: self.is_hit,
        };

        self.inner.deserialize_str(wrapper)
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let wrapper = StringCheckVisitor {
            inner: visitor,
            predicate: self.predicate,
            is_hit: self.is_hit,
        };

        self.inner.deserialize_string(wrapper)
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let wrapper = StringCheckVisitor {
            inner: visitor,
            predicate: self.predicate,
            is_hit: self.is_hit,
        };

        self.inner.deserialize_option(wrapper)
    }

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let wrapper = StringCheckVisitor {
            inner: visitor,
            predicate: self.predicate,
            is_hit: self.is_hit,
        };

        self.inner.deserialize_any(wrapper)
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char
        bytes byte_buf unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

struct StringCheckVisitor<'a, V, P: FnOnce(&str) -> bool> {
    inner: V,
    predicate: P,
    is_hit: &'a OnceCell<bool>,
}

impl<'de, V: Visitor<'de>, P: FnOnce(&str) -> bool + Copy> Visitor<'de>
    for StringCheckVisitor<'_, V, P>
{
    type Value = V::Value;

    fn visit_borrowed_str<E: serde::de::Error>(self, v: &'de str) -> Result<Self::Value, E> {
        self.is_hit
            .set((self.predicate)(v))
            .map_err(|_| serde::de::Error::custom("string already checked"))?;

        self.inner.visit_borrowed_str(v)
    }

    fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        let wrapper = StringCheckDeserializer {
            inner: deserializer,
            predicate: self.predicate,
            is_hit: self.is_hit,
        };

        self.inner.visit_some(wrapper)
    }

    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
        self.is_hit
            .set((self.predicate)(v))
            .map_err(|_| serde::de::Error::custom("string already checked"))?;

        self.inner.visit_str(v)
    }

    fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
        self.is_hit
            .set((self.predicate)(&v))
            .map_err(|_| serde::de::Error::custom("string already checked"))?;

        self.inner.visit_string(v)
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.expecting(formatter)
    }

    fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<Self::Value, E> {
        self.inner.visit_bool(v)
    }

    fn visit_borrowed_bytes<E: serde::de::Error>(self, v: &'de [u8]) -> Result<Self::Value, E> {
        self.inner.visit_borrowed_bytes(v)
    }

    fn visit_byte_buf<E: serde::de::Error>(self, v: Vec<u8>) -> Result<Self::Value, E> {
        self.inner.visit_byte_buf(v)
    }

    fn visit_bytes<E: serde::de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
        self.inner.visit_bytes(v)
    }

    fn visit_char<E: serde::de::Error>(self, v: char) -> Result<Self::Value, E> {
        self.inner.visit_char(v)
    }

    fn visit_enum<A: serde::de::EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
        self.inner.visit_enum(data)
    }

    fn visit_f32<E: serde::de::Error>(self, v: f32) -> Result<Self::Value, E> {
        self.inner.visit_f32(v)
    }

    fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<Self::Value, E> {
        self.inner.visit_f64(v)
    }

    fn visit_i128<E: serde::de::Error>(self, v: i128) -> Result<Self::Value, E> {
        self.inner.visit_i128(v)
    }

    fn visit_i16<E: serde::de::Error>(self, v: i16) -> Result<Self::Value, E> {
        self.inner.visit_i16(v)
    }

    fn visit_i32<E: serde::de::Error>(self, v: i32) -> Result<Self::Value, E> {
        self.inner.visit_i32(v)
    }

    fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Self::Value, E> {
        self.inner.visit_i64(v)
    }

    fn visit_i8<E: serde::de::Error>(self, v: i8) -> Result<Self::Value, E> {
        self.inner.visit_i8(v)
    }

    fn visit_map<A: serde::de::MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
        self.inner.visit_map(map)
    }

    fn visit_newtype_struct<D: Deserializer<'de>>(
        self,
        deserializer: D,
    ) -> Result<Self::Value, D::Error> {
        self.inner.visit_newtype_struct(deserializer)
    }

    fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
        self.inner.visit_none()
    }

    fn visit_seq<A: serde::de::SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
        self.inner.visit_seq(seq)
    }

    fn visit_u128<E: serde::de::Error>(self, v: u128) -> Result<Self::Value, E> {
        self.inner.visit_u128(v)
    }

    fn visit_u16<E: serde::de::Error>(self, v: u16) -> Result<Self::Value, E> {
        self.inner.visit_u16(v)
    }

    fn visit_u32<E: serde::de::Error>(self, v: u32) -> Result<Self::Value, E> {
        self.inner.visit_u32(v)
    }

    fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
        self.inner.visit_u64(v)
    }

    fn visit_u8<E: serde::de::Error>(self, v: u8) -> Result<Self::Value, E> {
        self.inner.visit_u8(v)
    }

    fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
        self.inner.visit_unit()
    }
}
