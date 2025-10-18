//! Deserialize an array of integer strings into a collection of integers (and the reverse).

use serde::{
    de::{Deserializer, Visitor},
    ser::Serializer,
};
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::str::FromStr;

const EXPECTED: &str = "integer string array";

pub fn deserialize<'de, E: FromStr, T: FromIterator<E>, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<T, D::Error> {
    struct IntegerStrArrayVisitor<E, T> {
        _element: PhantomData<E>,
        _target: PhantomData<T>,
    }

    impl<'de, E: FromStr, T: FromIterator<E>> Visitor<'de> for IntegerStrArrayVisitor<E, T> {
        type Value = T;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str(EXPECTED)
        }

        fn visit_seq<A: serde::de::SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
            let mut error = std::cell::OnceCell::new();

            let wrapper: IntegerStrArraySeqAccessWrapper<'de, '_, A, E> =
                IntegerStrArraySeqAccessWrapper {
                    underlying: seq,
                    error: &mut error,
                    _element: PhantomData,
                };

            let result = T::from_iter(wrapper);

            error.take().map_or_else(|| Ok(result), |error| Err(error))
        }
    }

    deserializer.deserialize_seq(IntegerStrArrayVisitor::<E, T> {
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

const INTEGER_STR_ARRAY_ELEMENT_EXPECTED: &str = "integer string";

struct IntegerStrArraySeqAccessWrapper<'de, 'a, A: serde::de::SeqAccess<'de>, E> {
    underlying: A,
    error: &'a mut std::cell::OnceCell<A::Error>,
    _element: std::marker::PhantomData<E>,
}

impl<'de, 'a, A: serde::de::SeqAccess<'de>, E: std::str::FromStr> IntoIterator
    for IntegerStrArraySeqAccessWrapper<'de, 'a, A, E>
{
    type Item = E;
    type IntoIter = IntegerStrArraySeqAccessIterator<'de, 'a, A, E>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { source: self }
    }
}

struct IntegerStrArraySeqAccessIterator<'de, 'a, A: serde::de::SeqAccess<'de>, E> {
    source: IntegerStrArraySeqAccessWrapper<'de, 'a, A, E>,
}

impl<'de, A: serde::de::SeqAccess<'de>, E: std::str::FromStr> Iterator
    for IntegerStrArraySeqAccessIterator<'de, '_, A, E>
{
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        if self.source.error.get().is_some() {
            None
        } else {
            match self
                .source
                .underlying
                .next_element::<std::borrow::Cow<'_, str>>()
            {
                Ok(Some(value)) => {
                    if let Ok(value) = value.parse() {
                        Some(value)
                    } else {
                        // We've just checked whether the cell is initialized.
                        self.source
                            .error
                            .set(serde::de::Error::invalid_value(
                                serde::de::Unexpected::Str(&value),
                                &INTEGER_STR_ARRAY_ELEMENT_EXPECTED,
                            ))
                            .unwrap();
                        None
                    }
                }
                Ok(None) => None,
                Err(error) => {
                    // We've just checked whether the cell is initialized.
                    self.source.error.set(error).unwrap();
                    None
                }
            }
        }
    }
}
