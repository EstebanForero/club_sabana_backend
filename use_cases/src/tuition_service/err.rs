use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    UnknownDatabaseError(String),
    #[error("Active tuition already exists")]
    ActiveTuitionExists,
    #[error("Invalid payment amount")]
    InvalidAmount,
    #[error("Tuition not found")]
    TuitionNotFound,
}
