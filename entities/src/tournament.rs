use super::datetime_serde;
use chrono::NaiveDateTime;
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Partial)]
#[partial(
    "TournamentCreation",
    derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq),
    omit(id_tournament)
)]
pub struct Tournament {
    pub id_tournament: Uuid,
    pub name: String,
    pub id_category: Uuid,
    #[serde(with = "datetime_serde")]
    pub start_datetime: NaiveDateTime,
    #[serde(with = "datetime_serde")]
    pub end_datetime: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Partial)]
#[partial(
    "TournamentRegistrationRequest",
    derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq),
    omit(id_tournament, registration_datetime)
)]
pub struct TournamentRegistration {
    pub id_tournament: Uuid,
    pub id_user: Uuid,
    #[serde(with = "datetime_serde")]
    pub registration_datetime: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Partial)]
#[partial(
    "TournamentAttendanceRequest",
    derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq),
    omit(id_tournament, attendance_datetime)
)]
pub struct TournamentAttendance {
    pub id_tournament: Uuid,
    pub id_user: Uuid,
    #[serde(with = "datetime_serde")]
    pub attendance_datetime: NaiveDateTime,
    pub position: i32,
}
