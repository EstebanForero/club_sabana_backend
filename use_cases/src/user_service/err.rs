use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unknow error in the database: {0}")]
    UnknownDatabaseError(#[from] Box<dyn std::error::Error>),
    #[error("User do not exists")]
    UserIdDontExist,
}
