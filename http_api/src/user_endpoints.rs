use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use entities::user::UserCreation;
use tracing::error;
use use_cases::user_service::err::Error;
use use_cases::user_service::UserService;

pub fn user_router(user_service: UserService) -> Router {
    Router::new()
        .route("/health", get(alive))
        .route("/register", post(register_user))
        .with_state(user_service)
}

async fn alive() -> &'static str {
    "The user router is alive"
}

async fn register_user(
    State(user_service): State<UserService>,
    Json(user_creation): Json<UserCreation>,
) -> impl IntoResponse {
    if let Err(err) = user_service.register_user(user_creation).await {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            message_from_err(err, "register user"),
        )
    } else {
        (StatusCode::OK, "User added succesfully")
    }
}

fn message_from_err(err: Error, endpoint_name: &str) -> &'static str {
    match err {
        Error::UnknownDatabaseError(error) => {
            error!("{endpoint_name}: {error}");
            "We are having problems in the server, try again"
        }
        Error::UserIdDontExist => "Unable to find user with the provided id",
        Error::ErrorHashing(error) => {
            error!("{endpoint_name}: {error}");
            "We are having problems in the server, try again"
        }
        Error::ErrorVerificationHash(error) => {
            error!("{endpoint_name}: {error}");
            "We are having problems in the server, try again"
        }
        Error::InvalidPassword => "The password is invalid, try again",
        Error::EmailAlreadyExists => "Email is already in use, try with other email",
        Error::PhoneAlreadyExists => "Phone is already in use, try with other phone",
        Error::DocumentAlreadyExists => "Document is already in use, try with other document",
    }
}
