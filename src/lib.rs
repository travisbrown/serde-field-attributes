#![warn(clippy::all, clippy::pedantic, clippy::nursery, rust_2018_idioms)]
#![allow(clippy::missing_errors_doc)]
#![forbid(unsafe_code)]
pub mod integer_or_integer_str;
pub mod integer_str;
pub mod integer_str_array;
pub mod optional_integer_str;
pub mod optional_integer_str_array;
pub mod optional_range;
pub mod optional_ratio_i64;
pub mod optional_ratio_u64;
pub mod optional_usize;
pub mod range;
pub mod ratio_i64;
pub mod ratio_u64;
pub mod represented_as_str;
pub mod timestamp_millis_str;
pub mod timestamp_str;

#[cfg(test)]
mod tests {
    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct IntegerStrData {
        #[serde(with = "super::integer_str")]
        value: u64,
    }

    #[test]
    fn deserialize_integer_str() {
        let json = format!(r#"{{"value":"{}"}}"#, 123);
        let expected = IntegerStrData { value: 123 };

        assert_eq!(
            serde_json::from_str::<IntegerStrData>(&json).unwrap(),
            expected
        );
    }

    #[test]
    fn serialize_integer_str() {
        let value = IntegerStrData { value: 123 };
        let expected = format!(r#"{{"value":"{}"}}"#, 123);

        assert_eq!(serde_json::json!(value).to_string(), expected);
    }

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct IntegerStrOptData {
        #[serde(
            with = "super::optional_integer_str",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        value: Option<u64>,
    }

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct IntegerStrArrayData {
        #[serde(with = "super::integer_str_array")]
        values: Vec<u64>,
    }

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct IntegerStrArrayOptData {
        #[serde(
            with = "super::optional_integer_str_array",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        values: Option<Vec<u64>>,
    }

    #[test]
    fn deserialize_some_integer_str_opt() {
        let json = format!(r#"{{"value":"{}"}}"#, 123);
        let expected = IntegerStrOptData { value: Some(123) };

        assert_eq!(
            serde_json::from_str::<IntegerStrOptData>(&json).unwrap(),
            expected
        );
    }

    #[test]
    fn serialize_some_integer_str_opt() {
        let value = IntegerStrOptData { value: Some(123) };
        let expected = format!(r#"{{"value":"{}"}}"#, 123);

        assert_eq!(serde_json::json!(value).to_string(), expected);
    }

    #[test]
    fn deserialize_missing_integer_str_opt() {
        let json = "{}";
        let expected = IntegerStrOptData { value: None };

        assert_eq!(
            serde_json::from_str::<IntegerStrOptData>(&json).unwrap(),
            expected
        );
    }

    #[test]
    fn deserialize_null_integer_str_opt() {
        let json = r#"{"value":null}"#;
        let expected = IntegerStrOptData { value: None };

        assert_eq!(
            serde_json::from_str::<IntegerStrOptData>(&json).unwrap(),
            expected
        );
    }
    #[test]
    fn serialize_none_integer_str_opt() {
        let value = IntegerStrOptData { value: None };
        let expected = "{}";

        assert_eq!(serde_json::json!(value).to_string(), expected);
    }

    #[test]
    fn deserialize_integer_str_array() {
        let json = r#"{"values":["123", "456"]}"#;
        let expected = IntegerStrArrayData {
            values: vec![123, 456],
        };

        assert_eq!(
            serde_json::from_str::<IntegerStrArrayData>(&json).unwrap(),
            expected
        );
    }

    #[test]
    fn serialize_integer_str_array() {
        let value = IntegerStrArrayData {
            values: vec![123, 456],
        };
        let expected = r#"{"values":["123","456"]}"#;

        assert_eq!(serde_json::json!(value).to_string(), expected);
    }

    #[test]
    fn deserialize_invalid_integer_str_array() {
        let invalid_type_json = r#"{"values":["123", 987, "456"]}"#;
        let invalid_value_json = r#"{"values":["123", "abc", "456"]}"#;

        let invalid_type_result = serde_json::from_str::<IntegerStrArrayData>(&invalid_type_json);
        let invalid_value_result = serde_json::from_str::<IntegerStrArrayData>(&invalid_value_json);

        assert!(invalid_type_result.is_err());
        assert!(invalid_value_result.is_err());
    }

    #[test]
    fn deserialize_integer_str_array_opt() {
        let json = r#"{"values":["123", "456"]}"#;
        let expected = IntegerStrArrayOptData {
            values: Some(vec![123, 456]),
        };

        assert_eq!(
            serde_json::from_str::<IntegerStrArrayOptData>(&json).unwrap(),
            expected
        );
    }

    #[test]
    fn serialize_integer_str_array_opt() {
        let value = IntegerStrArrayOptData {
            values: Some(vec![123, 456]),
        };
        let expected = r#"{"values":["123","456"]}"#;

        assert_eq!(serde_json::json!(value).to_string(), expected);
    }

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct UsizeOptData {
        #[serde(with = "super::optional_usize")]
        value_a: Option<usize>,
        #[serde(with = "super::optional_usize")]
        value_b: Option<usize>,
    }

    #[test]
    fn deserialize_u64_opt() {
        let json = r#"{"value_a":123,"value_b":-1}"#;
        let expected = UsizeOptData {
            value_a: Some(123),
            value_b: None,
        };

        assert_eq!(
            serde_json::from_str::<UsizeOptData>(&json).unwrap(),
            expected
        );
    }

    #[test]
    fn serialize_u64_opt() {
        let value = UsizeOptData {
            value_a: Some(123),
            value_b: None,
        };
        let expected = r#"{"value_a":123,"value_b":-1}"#;

        assert_eq!(serde_json::json!(value).to_string(), expected);
    }
}
