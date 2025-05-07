use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug, PartialEq, Eq)] // Added PartialEq for easier testing
pub enum Error {
    #[error("Unknow error in the database: {0}")]
    UnknownDatabaseError(String),
    #[error("User do not exists")]
    UserIdDontExist,
    #[error("Error hashing: {0}")] // Added {0}
    ErrorHashing(String),
    #[error("Error verifying hash: {0}")] // Added {0}
    ErrorVerificationHash(String),
    #[error("Error in password")]
    InvalidPassword,
    #[error("Email already exists")]
    EmailAlreadyExists,
    #[error("Phone number already exists")]
    PhoneAlreadyExists,
    #[error("Document already exists")]
    DocumentAlreadyExists,
    #[error("Invalid identifier")]
    InvalidIdentifier,
    #[error("Invalid birth date: {0}")] // New
    InvalidBirthDate(String),
}
