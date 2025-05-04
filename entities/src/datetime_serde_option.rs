use chrono::NaiveDateTime;
use serde::{de, Deserialize, Deserializer, Serializer};

use crate::datetime_serde;

pub fn serialize<S>(opt_datetime: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match opt_datetime {
        Some(datetime) => datetime_serde::serialize(datetime, serializer),
        None => serializer.serialize_none(),
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize into an Option<String> (null becomes None, string becomes Some(string))
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) => {
            let datetime = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
                .map_err(de::Error::custom)?;
            Ok(Some(datetime))
        }
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, NaiveDateTime};
    use serde::{Deserialize, Serialize};
    use serde_json;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestOptionDateTime {
        #[serde(with = "super")]
        datetime: Option<NaiveDateTime>,
    }

    #[test]
    fn test_serialize_option_some() {
        let datetime = Some(DateTime::from_timestamp(1672531199, 0).unwrap().naive_utc());
        let test_datetime = TestOptionDateTime { datetime };
        let serialized = serde_json::to_string(&test_datetime).unwrap();
        assert_eq!(serialized, r#"{"datetime":"2022-12-31 23:59:59"}"#);
    }

    #[test]
    fn test_serialize_option_none() {
        let test_datetime = TestOptionDateTime { datetime: None };
        let serialized = serde_json::to_string(&test_datetime).unwrap();
        assert_eq!(serialized, r#"{"datetime":null}"#);
    }

    #[test]
    fn test_deserialize_option_some() {
        let json_data = r#"{"datetime":"2022-12-31 23:59:59"}"#;
        let deserialized: TestOptionDateTime = serde_json::from_str(json_data).unwrap();
        let expected_datetime = Some(DateTime::from_timestamp(1672531199, 0).unwrap().naive_utc());
        assert_eq!(deserialized.datetime, expected_datetime);
    }

    #[test]
    fn test_deserialize_option_none() {
        let json_data = r#"{"datetime":null}"#;
        let deserialized: TestOptionDateTime = serde_json::from_str(json_data).unwrap();
        assert_eq!(deserialized.datetime, None);
    }

    #[test]
    fn test_deserialize_option_invalid_format() {
        let json_data = r#"{"datetime":"01-01-2022 23:59:59"}"#;
        let result: Result<TestOptionDateTime, _> = serde_json::from_str(json_data);
        assert!(result.is_err());
    }
}
