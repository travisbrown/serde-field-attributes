use num_rational::Ratio;
use num_traits::ToPrimitive;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

const EXPECTED: &str = "optional i64 ratio";

pub fn deserialize<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Ratio<i64>>, D::Error> {
    let value = Option::<f64>::deserialize(deserializer)?;

    match value {
        Some(value) => {
            let mut text = value.to_string();
            let decimal_point_index = text.find('.');
            let decimal_places = decimal_point_index.map_or(0, |index| text.len() - index - 1);
            let decimal_places = u32::try_from(decimal_places).map_err(|_| {
                serde::de::Error::invalid_value(serde::de::Unexpected::Float(value), &EXPECTED)
            })?;

            if let Some(index) = decimal_point_index {
                text.remove(index);
            }

            let numerator = text.parse::<i64>().map_err(|_| {
                serde::de::Error::invalid_value(serde::de::Unexpected::Float(value), &EXPECTED)
            })?;

            let denominator = 10i64.pow(decimal_places);

            Ok(Some(Ratio::new(numerator, denominator)))
        }
        None => Ok(None),
    }
}

pub fn serialize<S: Serializer>(
    value: &Option<Ratio<i64>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match value {
        Some(value) => {
            let value = value.to_f64().ok_or_else(|| {
                serde::ser::Error::custom("i64 ratio cannot be represented as f64")
            })?;

            f64::serialize(&value, serializer)
        }
        None => serializer.serialize_none(),
    }
}
