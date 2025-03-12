use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unknow error in the database: {0}")]
    UnknownDatabaseError(String),
    #[error("User do not exists")]
    UserIdDontExist,
    #[error("Error hashing")]
    ErrorHashing(String),
    #[error("Error verifying hash")]
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
}
