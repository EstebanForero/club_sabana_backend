use thiserror::Error;

use crate::category_service;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    UnknownDatabaseError(String),
    #[error("Tournament not found")]
    TournamentNotFound,
    #[error("User not registered for tournament")]
    UserNotRegistered,
    #[error("User already registered")]
    UserAlreadyRegistered,
    #[error("Invalid tournament dates")]
    InvalidDates,
    #[error("Invalid category")]
    InvalidCategory,
    #[error("Invalid position, the position must be positive")]
    NegativePosition,
    #[error("Invalid position, already taken")]
    PositionAlreadyTaken,
    #[error("User did not attend tournament")]
    UserDidNotAttend,
    #[error("User does not meet tournament category requirements")]
    UserDoesNotMeetCategoryRequirements,
    #[error("Category Service Error")]
    CategoryServiceError(#[from] category_service::err::Error),
}
