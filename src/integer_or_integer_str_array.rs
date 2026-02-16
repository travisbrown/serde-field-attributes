//! Deserialize an array of integers or integer strings into a collection of integers (and the reverse).

use serde::{
    de::{Deserializer, Visitor},
    ser::Serializer,
};
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::str::FromStr;

const EXPECTED: &str = "integer or integer string array";

pub fn deserialize<'de, E: FromStr, T: FromIterator<E>, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<T, D::Error> {
    struct ArrayVisitor<E, T> {
        _element: PhantomData<E>,
        _target: PhantomData<T>,
    }

    impl<'de, E: FromStr, T: FromIterator<E>> Visitor<'de> for ArrayVisitor<E, T> {
        type Value = T;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str(EXPECTED)
        }

        fn visit_seq<A: serde::de::SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
            let mut error = std::cell::OnceCell::new();

            let wrapper: SeqAccessWrapper<'de, '_, A, E> = SeqAccessWrapper {
                underlying: seq,
                error: &mut error,
                _element: PhantomData,
            };

            let result = T::from_iter(wrapper);

            error.take().map_or_else(|| Ok(result), |error| Err(error))
        }
    }

    deserializer.deserialize_seq(ArrayVisitor::<E, T> {
        _element: PhantomData,
        _target: PhantomData,
    })
}

pub fn serialize<'a, E: std::fmt::Display, T: 'a, S: Serializer>(
    values: &'a T,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    &'a T: IntoIterator<Item = E>,
{
    use serde::ser::SerializeSeq;

    let mut seq = serializer.serialize_seq(None)?;

    for value in values {
        seq.serialize_element(&value.to_string())?;
    }

    seq.end()
}

struct SeqAccessWrapper<'de, 'a, A: serde::de::SeqAccess<'de>, E> {
    underlying: A,
    error: &'a mut std::cell::OnceCell<A::Error>,
    _element: std::marker::PhantomData<E>,
}

impl<'de, 'a, A: serde::de::SeqAccess<'de>, E: FromStr> IntoIterator
    for SeqAccessWrapper<'de, 'a, A, E>
{
    type Item = E;
    type IntoIter = SeqAccessIterator<'de, 'a, A, E>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { source: self }
    }
}

struct SeqAccessIterator<'de, 'a, A: serde::de::SeqAccess<'de>, E> {
    source: SeqAccessWrapper<'de, 'a, A, E>,
}

impl<'de, A: serde::de::SeqAccess<'de>, E: FromStr> Iterator for SeqAccessIterator<'de, '_, A, E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        if self.source.error.get().is_some() {
            return None;
        }

        match self
            .source
            .underlying
            .next_element_seed(ElementSeed::<E>(PhantomData))
        {
            Ok(Some(value)) => Some(value),
            Ok(None) => None,
            Err(error) => {
                // We've just checked whether the cell is initialized.
                self.source.error.set(error).unwrap();
                None
            }
        }
    }
}

struct ElementSeed<T>(PhantomData<T>);

impl<'de, T: FromStr> serde::de::DeserializeSeed<'de> for ElementSeed<T> {
    type Value = T;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        super::integer_or_integer_str::deserialize(deserializer)
    }
}
