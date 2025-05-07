use chrono::NaiveDateTime;
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::datetime_serde;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Partial)]
#[partial(
    "CourtCreation",
    derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq),
    omit(id_court)
)]
pub struct Court {
    pub id_court: Uuid,
    pub court_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Partial)]
#[partial(
    "CourtReservationCreation",
    derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq),
    omit(id_court_reservation)
)]
pub struct CourtReservation {
    pub id_court_reservation: Uuid,
    pub id_court: Uuid,
    #[serde(with = "datetime_serde")]
    pub start_reservation_datetime: NaiveDateTime,
    #[serde(with = "datetime_serde")]
    pub end_reservation_datetime: NaiveDateTime,
    pub id_training: Option<Uuid>,
    pub id_tournament: Option<Uuid>,
}
