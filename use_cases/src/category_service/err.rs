use thiserror::Error;

use crate::user_service;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    UnknownDatabaseError(String),

    #[error("Category not found")]
    CategoryNotFound,

    #[error("Category already exists")]
    CategoryAlreadyExists,

    #[error("Invalid age range: min_age must be less than max_age")]
    InvalidAgeRange,

    #[error("Category name is required")]
    MissingName,

    #[error("Category requirement not found")]
    RequirementNotFound,

    #[error("User already has this category")]
    UserAlreadyHasCategory,

    #[error("User does not meet category requirements")]
    UserDoesNotMeetRequirements,

    #[error("Level not found")]
    LevelNotFound,

    #[error("Invalid age")]
    InvalidUserAge,

    #[error("Presequisite category level, is less than the required")]
    InvalidRequirementLevel,

    #[error("Error in user service")]
    UserServiceError(#[from] user_service::err::Error),
}
