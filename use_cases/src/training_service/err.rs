use thiserror::Error;

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

    #[error("Invalid dates: start date must be before end date")]
    InvalidDates,

    #[error("User not registered for this training")]
    UserNotRegistered,

    #[error("Training registration not found")]
    RegistrationNotFound,

    #[error("Error in category service")]
    CategoryServiceError(#[from] crate::category_service::err::Error),
}
