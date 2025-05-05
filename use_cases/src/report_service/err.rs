use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReportError {
    #[error("User service error: {0}")]
    UserServiceError(#[from] crate::user_service::err::Error),

    #[error("Category service error: {0}")]
    CategoryServiceError(#[from] crate::category_service::err::Error),

    #[error("Training service error: {0}")]
    TrainingServiceError(#[from] crate::training_service::err::Error),

    #[error("Tournament service error: {0}")]
    TournamentServiceError(#[from] crate::tournament_service::err::Error),

    #[error("Tuition service error: {0}")]
    TuitionServiceError(#[from] crate::tuition_service::err::Error),

    #[error("Request service error: {0}")]
    RequestServiceError(#[from] crate::request_service::err::Error),

    #[error("Report service error: {0}")]
    ReportServiceError(String),
}
