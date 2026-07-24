use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};
use std::cell::OnceCell;

/// Deserializes an `Option<T>` treating an empty string (`""`) as `None` and otherwise attempting
/// to deserialize as `T`.
///
/// # Examples
///
/// ```
/// #[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
/// struct Example {
///     #[serde(with = "serde_field_attributes::empty_string_or")]
///     value: Option<String>,
/// }
///
/// let none: Example = serde_json::from_str(r#"{"value":""}"#).unwrap();
/// assert_eq!(none.value, None);
///
/// let some: Example = serde_json::from_str(r#"{"value":"hello"}"#).unwrap();
/// assert_eq!(some.value, Some("hello".to_owned()));
/// ```
pub fn deserialize<'de, T: Deserialize<'de>, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<T>, D::Error> {
    let is_hit = OnceCell::new();

    let wrapper = super::StringCheckDeserializer {
        inner: deserializer,
        predicate: |input| input.is_empty(),
        is_hit: &is_hit,
    };

    let target = T::deserialize(wrapper);

    match is_hit.get() {
        Some(true) => Ok(None),
        _ => target.map(Some),
    }
}

/// Serializes an `Option<T>`, writing an empty string (`""`) for `None` and delegating to `T`'s
/// serializer for `Some(value)`.
pub fn serialize<T: Serialize, S: Serializer>(
    value: &Option<T>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match value {
        Some(value) => value.serialize(serializer),
        None => serializer.serialize_str(""),
    }
}

#[cfg(test)]
#[allow(clippy::option_option)]
mod tests {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct StringData {
        #[serde(with = "super")]
        value: Option<String>,
    }

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct OptionalStringData {
        #[serde(with = "super")]
        value: Option<Option<String>>,
    }

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct OptionalUsizeData {
        #[serde(with = "super")]
        value: Option<Option<usize>>,
    }

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct U64Data {
        #[serde(with = "super")]
        value: Option<u64>,
    }

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct StructData {
        #[serde(with = "super")]
        value: Option<Inner>,
    }

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct Inner {
        x: u32,
    }

    #[test]
    fn deserialize_empty_string_as_none() {
        let json = r#"{"value":""}"#;
        let result: StringData = serde_json::from_str(json).unwrap();
        assert_eq!(result, StringData { value: None });
    }

    #[test]
    fn deserialize_nonempty_string_as_some() {
        let json = r#"{"value":"hello"}"#;
        let result: StringData = serde_json::from_str(json).unwrap();
        assert_eq!(
            result,
            StringData {
                value: Some("hello".to_owned())
            }
        );
    }

    #[test]
    fn deserialize_null_as_inner_none() {
        let json = r#"{"value":null}"#;
        let result: OptionalStringData = serde_json::from_str(json).unwrap();
        assert_eq!(result, OptionalStringData { value: Some(None) });
    }

    #[test]
    fn deserialize_empty_string_as_outer_none() {
        let json = r#"{"value":""}"#;
        let result: OptionalStringData = serde_json::from_str(json).unwrap();
        assert_eq!(result, OptionalStringData { value: None });
    }

    #[test]
    fn deserialize_optional_string_as_double_some() {
        let json = r#"{"value":"abc"}"#;
        let result: OptionalStringData = serde_json::from_str(json).unwrap();
        assert_eq!(
            result,
            OptionalStringData {
                value: Some(Some("abc".to_string()))
            }
        );
    }

    #[test]
    fn deserialize_optional_usize_as_double_some() {
        let json = r#"{"value":123}"#;
        let result: OptionalUsizeData = serde_json::from_str(json).unwrap();
        assert_eq!(
            result,
            OptionalUsizeData {
                value: Some(Some(123))
            }
        );
    }

    #[test]
    fn deserialize_integer_as_some() {
        let json = r#"{"value":42}"#;
        let result: U64Data = serde_json::from_str(json).unwrap();
        assert_eq!(result, U64Data { value: Some(42) });
    }

    #[test]
    fn deserialize_empty_string_as_none_for_integer_type() {
        let json = r#"{"value":""}"#;
        let result: U64Data = serde_json::from_str(json).unwrap();
        assert_eq!(result, U64Data { value: None });
    }

    #[test]
    fn deserialize_struct_as_some() {
        let json = r#"{"value":{"x":7}}"#;
        let result: StructData = serde_json::from_str(json).unwrap();
        assert_eq!(
            result,
            StructData {
                value: Some(Inner { x: 7 })
            }
        );
    }

    #[test]
    fn deserialize_empty_string_as_none_for_struct_type() {
        let json = r#"{"value":""}"#;
        let result: StructData = serde_json::from_str(json).unwrap();
        assert_eq!(result, StructData { value: None });
    }

    #[test]
    fn serialize_none_as_empty_string() {
        let data = StringData { value: None };
        let json = serde_json::json!(data).to_string();
        assert_eq!(json, r#"{"value":""}"#);
    }

    #[test]
    fn serialize_some_string() {
        let data = StringData {
            value: Some("hello".to_owned()),
        };
        let json = serde_json::json!(data).to_string();
        assert_eq!(json, r#"{"value":"hello"}"#);
    }

    #[test]
    fn serialize_some_integer() {
        let data = U64Data { value: Some(42) };
        let json = serde_json::json!(data).to_string();
        assert_eq!(json, r#"{"value":42}"#);
    }

    #[test]
    fn serialize_none_integer_as_empty_string() {
        let data = U64Data { value: None };
        let json = serde_json::json!(data).to_string();
        assert_eq!(json, r#"{"value":""}"#);
    }

    #[test]
    fn roundtrip_some_string() {
        let original = StringData {
            value: Some("world".to_owned()),
        };
        let json = serde_json::json!(original).to_string();
        let roundtripped: StringData = serde_json::from_str(&json).unwrap();
        assert_eq!(original, roundtripped);
    }

    #[test]
    fn roundtrip_none() {
        let original = StringData { value: None };
        let json = serde_json::json!(original).to_string();
        let roundtripped: StringData = serde_json::from_str(&json).unwrap();
        assert_eq!(original, roundtripped);
    }

    #[test]
    fn deserialize_invalid_string_for_integer_type_fails() {
        let json = r#"{"value":"not-a-number"}"#;
        assert!(serde_json::from_str::<U64Data>(json).is_err());
    }
}
