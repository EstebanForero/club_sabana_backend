use chrono::NaiveDate;
use serde::{de, Deserialize, Deserializer, Serializer};

pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&date.format("%Y-%m-%d").to_string())
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(de::Error::custom)
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use serde::{Deserialize, Serialize};
    use serde_json;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestDate {
        #[serde(with = "super")]
        date: NaiveDate,
    }

    #[test]
    fn test_serialize() {
        let date = NaiveDate::from_ymd_opt(2023, 10, 15).unwrap();
        let test_date = TestDate { date };

        let serialized = serde_json::to_string(&test_date).unwrap();
        assert_eq!(serialized, r#"{"date":"2023-10-15"}"#);
    }

    #[test]
    fn test_deserialize() {
        let json_data = r#"{"date":"2023-10-15"}"#;
        let deserialized: TestDate = serde_json::from_str(json_data).unwrap();

        let expected_date = NaiveDate::from_ymd_opt(2023, 10, 15).unwrap();
        assert_eq!(deserialized.date, expected_date);
    }

    #[test]
    fn test_deserialize_invalid_format() {
        let json_data = r#"{"date":"15-10-2023"}"#;
        let result: Result<TestDate, _> = serde_json::from_str(json_data);

        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_invalid_date() {
        let json_data = r#"{"date":"2023-02-30"}"#; // February 30th doesn't exist
        let result: Result<TestDate, _> = serde_json::from_str(json_data);

        assert!(result.is_err());
    }
}
