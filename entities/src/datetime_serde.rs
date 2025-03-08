use chrono::NaiveDateTime;
use serde::{de, Deserialize, Deserializer, Serializer};

pub fn serialize<S>(datetime: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&datetime.format("%Y-%m-%d %H:%M:%S").to_string())
}

/// Deserializes a string "YYYY-MM-DD HH:MM:SS" into a NaiveDateTime
pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").map_err(de::Error::custom)
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, NaiveDateTime};
    use serde::{Deserialize, Serialize};
    use serde_json;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestDateTime {
        #[serde(with = "super")]
        datetime: NaiveDateTime,
    }

    #[test]
    fn test_serialize() {
        let datetime = DateTime::from_timestamp(1672531199, 0).unwrap().naive_utc(); // 2023-01-01 23:59:59
        let test_datetime = TestDateTime { datetime };

        let serialized = serde_json::to_string(&test_datetime).unwrap();
        assert_eq!(serialized, r#"{"datetime":"2022-12-31 23:59:59"}"#);
    }

    #[test]
    fn test_deserialize() {
        let json_data = r#"{"datetime":"2022-12-31 23:59:59"}"#;
        let deserialized: TestDateTime = serde_json::from_str(json_data).unwrap();

        let expected_datetime = DateTime::from_timestamp(1672531199, 0).unwrap().naive_utc();
        assert_eq!(deserialized.datetime, expected_datetime);
    }

    #[test]
    fn test_deserialize_invalid_format() {
        let json_data = r#"{"datetime":"01-01-2022 23:59:59"}"#; // Wrong date format
        let result: Result<TestDateTime, _> = serde_json::from_str(json_data);

        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_invalid_datetime() {
        let json_data = r#"{"datetime":"2022-02-30 25:61:61"}"#; // Invalid date and time
        let result: Result<TestDateTime, _> = serde_json::from_str(json_data);

        assert!(result.is_err());
    }

    #[test]
    fn test_round_trip() {
        let original_datetime = DateTime::from_timestamp(1672531199, 0).unwrap().naive_utc();
        let test_datetime = TestDateTime {
            datetime: original_datetime,
        };

        // Serialize
        let serialized = serde_json::to_string(&test_datetime).unwrap();

        // Deserialize
        let deserialized: TestDateTime = serde_json::from_str(&serialized).unwrap();

        assert_eq!(test_datetime, deserialized);
    }
}
