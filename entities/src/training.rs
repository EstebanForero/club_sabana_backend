use super::datetime_serde;
use chrono::NaiveDateTime;
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Partial)]
#[partial(
    "TrainingCreation",
    derive(Debug, Serialize, Deserialize),
    omit(id_training)
)]
pub struct Training {
    pub id_training: Uuid,
    pub name: String,
    pub id_category: Uuid,
    #[serde(with = "datetime_serde")]
    pub start_datetime: NaiveDateTime,
    #[serde(with = "datetime_serde")]
    pub end_datetime: NaiveDateTime,
    pub minimum_payment: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingRegistration {
    pub id_training: Uuid,
    pub id_user: Uuid,
    #[serde(with = "datetime_serde")]
    pub registration_datetime: NaiveDateTime,
    pub attended: bool,
    #[serde(with = "datetime_serde")]
    pub attendance_datetime: NaiveDateTime,
}
