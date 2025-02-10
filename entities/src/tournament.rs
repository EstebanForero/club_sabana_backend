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

// DTOs without deleted field
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TournamentDTO {
    pub id_tournament: Uuid,
    pub name: String,
    pub id_category: Uuid,
    #[serde(with = "datetime_serde")]
    pub start_datetime: NaiveDateTime,
    #[serde(with = "datetime_serde")]
    pub end_datetime: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TournamentRegistrationDTO {
    pub id_tournament: Uuid,
    pub id_user: Uuid,
    #[serde(with = "datetime_serde")]
    pub registration_datetime: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TournamentAttendanceDTO {
    pub id_tournament: Uuid,
    pub id_user: Uuid,
    #[serde(with = "datetime_serde")]
    pub attendance_datetime: NaiveDateTime,
    pub position: i32,
}

// Conversion implementations
impl From<Tournament> for TournamentDTO {
    fn from(t: Tournament) -> Self {
        TournamentDTO {
            id_tournament: t.id_tournament,
            name: t.name,
            id_category: t.id_category,
            start_datetime: t.start_datetime,
            end_datetime: t.end_datetime,
        }
    }
}

impl From<TournamentRegistration> for TournamentRegistrationDTO {
    fn from(r: TournamentRegistration) -> Self {
        TournamentRegistrationDTO {
            id_tournament: r.id_tournament,
            id_user: r.id_user,
            registration_datetime: r.registration_datetime,
        }
    }
}

impl From<TournamentAttendance> for TournamentAttendanceDTO {
    fn from(a: TournamentAttendance) -> Self {
        TournamentAttendanceDTO {
            id_tournament: a.id_tournament,
            id_user: a.id_user,
            attendance_datetime: a.attendance_datetime,
            position: a.position,
        }
    }
}
