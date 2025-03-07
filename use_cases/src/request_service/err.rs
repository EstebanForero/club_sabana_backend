use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    UnknownDatabaseError(String),
    #[error("Request not found")]
    RequestNotFound,
    #[error("Request already completed")]
    RequestAlreadyCompleted,
    #[error("Cannot approve/reject your own request")]
    SelfApprovalNotAllowed,
    #[error("Invalid approver ID")]
    InvalidApprover,
}
