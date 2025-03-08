use super::datetime_serde;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Tuition {
    pub id_tuition: Uuid,
    pub id_user: Uuid,
    pub amount: f64,
    #[serde(with = "datetime_serde")]
    pub payment_date: NaiveDateTime,
}
