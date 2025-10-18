use num_rational::Ratio;
use num_traits::ToPrimitive;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

const EXPECTED: &str = "i64 ratio";

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Ratio<i64>, D::Error> {
    let value = f64::deserialize(deserializer)?;
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

    Ok(Ratio::new(numerator, denominator))
}

pub fn serialize<S: Serializer>(value: &Ratio<i64>, serializer: S) -> Result<S::Ok, S::Error> {
    let value = value
        .to_f64()
        .ok_or_else(|| serde::ser::Error::custom("i64 ratio cannot be represented as f64"))?;

    f64::serialize(&value, serializer)
}

#[cfg(test)]
mod tests {
    use crate::ratio_i64;
    use num_rational::Ratio;

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct Test {
        #[serde(with = "ratio_i64")]
        foo: Ratio<i64>,
        #[serde(with = "ratio_i64")]
        bar: Ratio<i64>,
    }

    #[test]
    fn parse_json() {
        let test =
            serde_json::from_str::<Test>(r#"{ "foo": -0.1372897, "bar": 132791823 }"#).unwrap();

        assert_eq!(test.foo, Ratio::new(-1372897, 10000000));
        assert_eq!(test.bar, Ratio::new(132791823, 1));
    }

    #[test]
    fn round_trip_json() {
        let value = Test {
            foo: Ratio::new(-1372897, 10000000),
            bar: Ratio::new(132791823, 1),
        };

        let serialized = serde_json::json!(value).to_string();
        let deserialized = serde_json::from_str::<Test>(&serialized).unwrap();

        assert_eq!(deserialized, value);
    }
}
