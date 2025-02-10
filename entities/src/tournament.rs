use super::datetime_serde;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Tournament {
    pub id_tournament: Uuid,
    pub name: String,
    pub id_category: Uuid,
    #[serde(with = "datetime_serde")]
    pub start_datetime: NaiveDateTime,
    #[serde(with = "datetime_serde")]
    pub end_datetime: NaiveDateTime,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TournamentRegistration {
    pub id_tournament: Uuid,
    pub id_user: Uuid,
    #[serde(with = "datetime_serde")]
    pub registration_datetime: NaiveDateTime,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TournamentAttendance {
    pub id_tournament: Uuid,
    pub id_user: Uuid,
    #[serde(with = "datetime_serde")]
    pub attendance_datetime: NaiveDateTime,
    pub position: i32,
    pub deleted: bool,
}
