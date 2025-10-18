use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, SerializeTuple, Serializer},
};
use std::ops::Range;

pub fn deserialize<'de, T: Deserialize<'de>, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Range<T>>, D::Error> {
    let range: Option<(T, T)> = Deserialize::deserialize(deserializer)?;

    Ok(range.map(|(start, end)| start..end))
}

pub fn serialize<T: Serialize, S: Serializer>(
    value: &Option<Range<T>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match value {
        Some(value) => {
            let mut tuple = serializer.serialize_tuple(2)?;
            tuple.serialize_element(&value.start)?;
            tuple.serialize_element(&value.end)?;
            tuple.end()
        }
        None => serializer.serialize_none(),
    }
}
