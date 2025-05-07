use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("Database error: {0}")]
    UnknownDatabaseError(String),
    #[error("Court not found")]
    CourtNotFound,
    #[error("Court name already exists")]
    CourtNameExists,
    #[error("Court is already reserved for the given time")]
    CourtUnavailable,
    #[error("Invalid reservation time: end time must be after start time")]
    InvalidReservationTime,
    #[error("Reservation must be linked to a training or a tournament")]
    ReservationPurposeMissing,
    #[error("Reservation not found")]
    ReservationNotFound,
    #[error("Cannot link a reservation to both a training and a tournament")]
    ReservationPurposeConflict,
}
