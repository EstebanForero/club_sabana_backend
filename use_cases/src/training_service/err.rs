use thiserror::Error;

use crate::court_service;
use crate::tuition_service;
use crate::user_service;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unknown error in the database: {0}")]
    UnknownDatabaseError(String),

    #[error("Training not found")]
    TrainingNotFound,

    #[error("User already registered for this training")]
    UserAlreadyRegistered,

    #[error("User does not meet category requirements")]
    UserDoesNotMeetCategoryRequirements,

    #[error("Invalid dates: start date must be before end date, or duration is out of bounds (10min-5hr)")]
    InvalidDates,

    #[error("User not registered for this training")]
    UserNotRegistered,

    #[error("Training registration not found")]
    RegistrationNotFound,

    #[error("Error in category service: {0}")]
    CategoryServiceError(#[from] crate::category_service::err::Error),

    #[error("Error in court service: {0}")]
    CourtServiceError(#[from] court_service::err::Error),

    #[error("Error in user service: {0}")]
    UserServiceError(#[from] user_service::err::Error),

    #[error("Error in tuition service: {0}")]
    TuitionServiceError(#[from] tuition_service::err::Error),

    #[error("The event hasn't started, wait until the event starts to register assistances")]
    InvalidAssistanceDate,

    #[error("Users can register only before an event starts")]
    InvalidRegistrationDate,
}
