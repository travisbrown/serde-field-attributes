use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, SerializeTuple, Serializer},
};
use std::ops::Range;

pub fn deserialize<'de, T: Deserialize<'de>, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Range<T>, D::Error> {
    let (start, end) = Deserialize::deserialize(deserializer)?;

    Ok(start..end)
}

pub fn serialize<T: Serialize, S: Serializer>(
    value: &Range<T>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut tuple = serializer.serialize_tuple(2)?;
    tuple.serialize_element(&value.start)?;
    tuple.serialize_element(&value.end)?;
    tuple.end()
}
