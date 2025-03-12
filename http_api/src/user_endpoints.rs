use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};

use super::err::ToErrResponse;

use entities::user::{URol, UserCreation, UserInfo, UserLogInInfo};
use serde::{Deserialize, Serialize};
use tracing::error;
use use_cases::user_service::{
    err::{self, Error},
    UserService,
};
use uuid::Uuid;

use crate::{
    auth::generate_jwt,
    err::{HttpError, HttpResult},
};

pub fn user_router(user_service: UserService, token_key: &str) -> Router {
    Router::new()
        .route("/health-user", get(alive))
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
        .http_err("get all users")?;

    Ok(Json(users))
}

async fn get_user_by_id(
    State((user_service, _)): State<(UserService, String)>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserInfo>, Response> {
    let user = user_service
        .get_user_by_id(user_id)
        .await
        .http_err("get user by id")?;

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
        .http_err("log in user")?;

    let token = generate_jwt(&log_in_response, &token_key).http_err("log in user")?;

    Ok(Json(LogInResponse {
        token,
        user_rol: log_in_response.user_rol,
    }))
}

async fn register_user(
    State((user_service, _)): State<(UserService, String)>,
    Json(user_creation): Json<UserCreation>,
) -> HttpResult<impl IntoResponse> {
    user_service
        .register_user(user_creation)
        .await
        .http_err("register user")?;

    Ok((StatusCode::OK, "User added succesfully"))
}

impl<T> HttpError<T> for Result<T, jsonwebtoken::errors::Error> {
    fn http_err(self, endpoint: &str) -> HttpResult<T> {
        self.map_err(|err| {
            format!("{endpoint}: Error with json web token: {err}").to_err_response()
        })
    }
}

impl<T> HttpError<T> for err::Result<T> {
    fn http_err(self, endpoint_name: &str) -> crate::err::HttpResult<T> {
        self.map_err(|err| {
            error!("Error in: {endpoint_name}");
            match err {
                Error::UnknownDatabaseError(error) => {
                    error!("{error}");
                    "We are having problems in the server, try again"
                }
                Error::UserIdDontExist => "Unable to find user with the provided id",
                Error::ErrorHashing(error) => {
                    error!("{error}");
                    "We are having problems in the server, try again"
                }
                Error::ErrorVerificationHash(error) => {
                    error!("{error}");
                    "We are having problems in the server, try again"
                }
                Error::InvalidPassword => "The password is invalid, try again",
                Error::EmailAlreadyExists => "Email is already in use, try with other email",
                Error::PhoneAlreadyExists => "Phone is already in use, try with other phone",
                Error::DocumentAlreadyExists => {
                    "Document is already in use, try with other document"
                }
                Error::InvalidIdentifier => {
                    "There is not an user registered with the provided identifier"
                }
            }
            .to_err_response()
        })
    }
}
