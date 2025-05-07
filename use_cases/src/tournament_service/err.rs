use thiserror::Error;

use crate::category_service;
use crate::court_service;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("Database error: {0}")]
    UnknownDatabaseError(String),
    #[error("Tournament not found")]
    TournamentNotFound,
    #[error("User not registered for tournament")]
    UserNotRegistered,
    #[error("User already registered")]
    UserAlreadyRegistered,
    #[error("Invalid tournament dates or duration (10min-5hr)")]
    InvalidDates,
    #[error("Invalid category")]
    InvalidCategory,
    #[error("Invalid position, the position must be positive and not zero")]
    NegativePosition,
    #[error("Invalid position, already taken")]
    PositionAlreadyTaken,
    #[error("User did not attend tournament")]
    UserDidNotAttend,
    #[error("User does not meet tournament category requirements")]
    UserDoesNotMeetCategoryRequirements,
    #[error("Category Service Error: {0}")]
    CategoryServiceError(#[from] category_service::err::Error),
    #[error("Court Service Error: {0}")]
    CourtServiceError(#[from] court_service::err::Error),
}
