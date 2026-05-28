use serde::{
    de::{Deserialize, Deserializer, Visitor},
    ser::{Serialize, Serializer},
};
use std::fmt;
use std::marker::PhantomData;

/// Returns `true` when every byte is a space (`0x20`) or tab (`0x09`).
#[inline]
fn is_whitespace_only(input: &str) -> bool {
    input.bytes().all(|byte| byte == b' ' || byte == b'\t')
}

/// Deserializes an `Option<T>` treating any string composed entirely of spaces or tabs (including
/// the empty string) as `None`, and otherwise attempting to deserialize as `T`.
///
/// # Examples
///
/// ```
/// #[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
/// struct Example {
///     #[serde(with = "serde_field_attributes::whitespace_string_or")]
///     value: Option<String>,
/// }
///
/// let none: Example = serde_json::from_str(r#"{"value":"   \t"}"#).unwrap();
/// assert_eq!(none.value, None);
///
/// let some: Example = serde_json::from_str(r#"{"value":"hello"}"#).unwrap();
/// assert_eq!(some.value, Some("hello".to_owned()));
/// ```
pub fn deserialize<'de, T: Deserialize<'de>, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<T>, D::Error> {
    struct WhitespaceStringOrVisitor<T>(PhantomData<T>);

    impl<'de, T: Deserialize<'de>> Visitor<'de> for WhitespaceStringOrVisitor<T> {
        type Value = Option<T>;

        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("a whitespace-only string or a deserializable value")
        }

        fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
            T::deserialize(serde::de::value::UnitDeserializer::new()).map(Some)
        }

        fn visit_some<D: Deserializer<'de>>(
            self,
            deserializer: D,
        ) -> Result<Self::Value, D::Error> {
            T::deserialize(deserializer).map(Some)
        }

        fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<Self::Value, E> {
            T::deserialize(serde::de::value::BoolDeserializer::new(v)).map(Some)
        }

        fn visit_i8<E: serde::de::Error>(self, v: i8) -> Result<Self::Value, E> {
            T::deserialize(serde::de::value::I8Deserializer::new(v)).map(Some)
        }

        fn visit_i16<E: serde::de::Error>(self, v: i16) -> Result<Self::Value, E> {
            T::deserialize(serde::de::value::I16Deserializer::new(v)).map(Some)
        }

        fn visit_i32<E: serde::de::Error>(self, v: i32) -> Result<Self::Value, E> {
            T::deserialize(serde::de::value::I32Deserializer::new(v)).map(Some)
        }

        fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Self::Value, E> {
            T::deserialize(serde::de::value::I64Deserializer::new(v)).map(Some)
        }

        fn visit_i128<E: serde::de::Error>(self, v: i128) -> Result<Self::Value, E> {
            T::deserialize(serde::de::value::I128Deserializer::new(v)).map(Some)
        }

        fn visit_u8<E: serde::de::Error>(self, v: u8) -> Result<Self::Value, E> {
            T::deserialize(serde::de::value::U8Deserializer::new(v)).map(Some)
        }

        fn visit_u16<E: serde::de::Error>(self, v: u16) -> Result<Self::Value, E> {
            T::deserialize(serde::de::value::U16Deserializer::new(v)).map(Some)
        }

        fn visit_u32<E: serde::de::Error>(self, v: u32) -> Result<Self::Value, E> {
            T::deserialize(serde::de::value::U32Deserializer::new(v)).map(Some)
        }

        fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
            T::deserialize(serde::de::value::U64Deserializer::new(v)).map(Some)
        }

        fn visit_u128<E: serde::de::Error>(self, v: u128) -> Result<Self::Value, E> {
            T::deserialize(serde::de::value::U128Deserializer::new(v)).map(Some)
        }

        fn visit_f32<E: serde::de::Error>(self, v: f32) -> Result<Self::Value, E> {
            T::deserialize(serde::de::value::F32Deserializer::new(v)).map(Some)
        }

        fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<Self::Value, E> {
            T::deserialize(serde::de::value::F64Deserializer::new(v)).map(Some)
        }

        fn visit_char<E: serde::de::Error>(self, v: char) -> Result<Self::Value, E> {
            T::deserialize(serde::de::value::CharDeserializer::new(v)).map(Some)
        }

        fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
            if is_whitespace_only(v) {
                Ok(None)
            } else {
                T::deserialize(serde::de::value::StrDeserializer::new(v)).map(Some)
            }
        }

        fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
            if is_whitespace_only(&v) {
                Ok(None)
            } else {
                T::deserialize(serde::de::value::StringDeserializer::new(v)).map(Some)
            }
        }

        fn visit_seq<A: serde::de::SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
            T::deserialize(serde::de::value::SeqAccessDeserializer::new(seq)).map(Some)
        }

        fn visit_map<A: serde::de::MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
            T::deserialize(serde::de::value::MapAccessDeserializer::new(map)).map(Some)
        }
    }

    deserializer.deserialize_any(WhitespaceStringOrVisitor::<T>(PhantomData))
}

/// Serializes an `Option<T>`, writing an empty string `""` for `None` and
/// delegating to `T`'s serializer for `Some(value)`.
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
mod tests {
    use super::is_whitespace_only;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct StringData {
        #[serde(with = "super")]
        value: Option<String>,
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

    // --- is_whitespace_only predicate ---

    #[test]
    fn whitespace_only_empty() {
        assert!(is_whitespace_only(""));
    }

    #[test]
    fn whitespace_only_spaces() {
        assert!(is_whitespace_only("   "));
    }

    #[test]
    fn whitespace_only_tabs() {
        assert!(is_whitespace_only("\t\t"));
    }

    #[test]
    fn whitespace_only_mixed_space_tab() {
        assert!(is_whitespace_only(" \t \t"));
    }

    #[test]
    fn whitespace_only_rejects_newline() {
        assert!(!is_whitespace_only("\n"));
        assert!(!is_whitespace_only(" \n "));
    }

    #[test]
    fn whitespace_only_rejects_text() {
        assert!(!is_whitespace_only("hello"));
        assert!(!is_whitespace_only(" hello "));
    }

    // --- deserialize ---

    #[test]
    fn deserialize_empty_string_as_none() {
        let json = r#"{"value":""}"#;
        let result: StringData = serde_json::from_str(json).unwrap();
        assert_eq!(result, StringData { value: None });
    }

    #[test]
    fn deserialize_spaces_as_none() {
        let json = r#"{"value":"   "}"#;
        let result: StringData = serde_json::from_str(json).unwrap();
        assert_eq!(result, StringData { value: None });
    }

    #[test]
    fn deserialize_tabs_as_none() {
        // JSON \t escape sequences decode to tab bytes, which are whitespace-only.
        let json = r#"{"value":"\t\t"}"#;
        let result: StringData = serde_json::from_str(json).unwrap();
        assert_eq!(result, StringData { value: None });
    }

    #[test]
    fn deserialize_mixed_whitespace_as_none() {
        let json = r#"{"value":" \t "}"#;
        let result: StringData = serde_json::from_str(json).unwrap();
        assert_eq!(result, StringData { value: None });
    }

    #[test]
    fn deserialize_nonempty_nonwhitespace_string_as_some() {
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
    fn deserialize_string_with_leading_space_as_some() {
        let json = r#"{"value":" hello"}"#;
        let result: StringData = serde_json::from_str(json).unwrap();
        assert_eq!(
            result,
            StringData {
                value: Some(" hello".to_owned())
            }
        );
    }

    #[test]
    fn deserialize_newline_string_as_some() {
        // Newline is not space or tab, so it is not treated as None.
        let json = r#"{"value":"\n"}"#;
        let result: StringData = serde_json::from_str(json).unwrap();
        assert_eq!(
            result,
            StringData {
                value: Some("\n".to_owned())
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
    fn deserialize_whitespace_as_none_for_integer_type() {
        let json = r#"{"value":"  "}"#;
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
    fn deserialize_whitespace_as_none_for_struct_type() {
        let json = r#"{"value":"\t"}"#;
        let result: StructData = serde_json::from_str(json).unwrap();
        assert_eq!(result, StructData { value: None });
    }

    // --- serialize ---

    #[test]
    fn serialize_none_as_empty_string() {
        let data = StringData { value: None };
        assert_eq!(serde_json::json!(data).to_string(), r#"{"value":""}"#);
    }

    #[test]
    fn serialize_some_string() {
        let data = StringData {
            value: Some("hello".to_owned()),
        };
        assert_eq!(serde_json::json!(data).to_string(), r#"{"value":"hello"}"#);
    }

    #[test]
    fn serialize_some_integer() {
        let data = U64Data { value: Some(42) };
        assert_eq!(serde_json::json!(data).to_string(), r#"{"value":42}"#);
    }

    #[test]
    fn serialize_none_integer_as_empty_string() {
        let data = U64Data { value: None };
        assert_eq!(serde_json::json!(data).to_string(), r#"{"value":""}"#);
    }

    // --- roundtrips ---

    #[test]
    fn roundtrip_some_string() {
        let original = StringData {
            value: Some("world".to_owned()),
        };
        let roundtripped: StringData =
            serde_json::from_str(&serde_json::json!(original).to_string()).unwrap();
        assert_eq!(original, roundtripped);
    }

    #[test]
    fn roundtrip_none() {
        let original = StringData { value: None };
        let roundtripped: StringData =
            serde_json::from_str(&serde_json::json!(original).to_string()).unwrap();
        assert_eq!(original, roundtripped);
    }

    #[test]
    fn deserialize_invalid_string_for_integer_type_fails() {
        let json = r#"{"value":"not-a-number"}"#;
        assert!(serde_json::from_str::<U64Data>(json).is_err());
    }
}
