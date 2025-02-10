use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};

fn internal_error_response(message: &str) -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, message.to_string()).into_response()
}

use entities::user::{URol, UserCreation, UserInfo, UserLogInInfo};
use serde::{Deserialize, Serialize};
use tracing::error;
use use_cases::user_service::{err::Error, UserService};
use uuid::Uuid;

use crate::auth::generate_jwt;

pub fn user_router(user_service: UserService, token_key: &str) -> Router {
    Router::new()
        .route("/health", get(alive))
        .route("/register", post(register_user))
        .route("/logIn", post(log_in_user))
        .route("/users", get(get_all_users))
        .route("/users/{id}", get(get_user_by_id))
        .with_state((user_service, token_key.to_string()))
}

async fn get_all_users(
    State((user_service, _)): State<(UserService, String)>,
) -> Result<Json<Vec<UserInfo>>, Response> {
    let users = user_service
        .get_all_users()
        .await
        .map_err(|err| internal_error_response(&message_from_err(err, "get all users")))?;

    Ok(Json(users))
}

async fn get_user_by_id(
    State((user_service, _)): State<(UserService, String)>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserInfo>, Response> {
    let user = user_service
        .get_user_by_id(user_id)
        .await
        .map_err(|err| internal_error_response(&message_from_err(err, "get user by id")))?;

    Ok(Json(user))
}

async fn alive() -> Result<Json<String>, Response> {
    "The user router is alive";

    Ok(Json("I am alive".to_string()))

    // Example Error Err((StatusCode::INTERNAL_SERVER_ERROR, error_message))
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct LogInResponse {
    token: String,
    user_rol: URol,
}

async fn log_in_user(
    State((user_service, token_key)): State<(UserService, String)>,
    Json(user_log_in_info): Json<UserLogInInfo>,
) -> Result<Json<LogInResponse>, Response> {
    let log_in_response = user_service
        .log_in_user(&user_log_in_info)
        .await
        .map_err(|err| internal_error_response(&message_from_err(err, "log in user")))?;

    let token = generate_jwt(&log_in_response, &token_key).map_err(|err| {
        error!("Error log in user, generating jwt: {}", err.to_string());
        internal_error_response("Internal error generating token")
    })?;

    Ok(Json(LogInResponse {
        token,
        user_rol: log_in_response.user_rol,
    }))
}

async fn register_user(
    State((user_service, _)): State<(UserService, String)>,
    Json(user_creation): Json<UserCreation>,
) -> impl IntoResponse {
    if let Err(err) = user_service.register_user(user_creation).await {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            message_from_err(err, "register user"),
        )
    } else {
        (StatusCode::OK, "User added succesfully".into())
    }
}

fn message_from_err(err: Error, endpoint_name: &str) -> String {
    let error_msg = match err {
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
        Error::InvalidIdentifier => "There is not an user registered with the provided identifier",
    };

    error!("error in {endpoint_name}: {error_msg}");

    error_msg.to_string()
}
