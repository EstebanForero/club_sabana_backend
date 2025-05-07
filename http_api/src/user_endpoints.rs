use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};

// Removed super::err::ToErrResponse as HttpError is used directly
use entities::user::{URol, UserCreation, UserInfo, UserLogInInfo};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::error;
use use_cases::user_service::{
    err::Error as UserServiceError, // Renamed Error to UserServiceError
    UserService,
};
use uuid::Uuid; // Added for Arc<UserService>

use crate::{
    auth::generate_jwt,
    err::{HttpError, HttpResult}, // HttpResult might need adjustment if ToErrResponse was its only user
};

// Router function now takes Arc<UserService>
pub fn user_router(user_service: Arc<UserService>, token_key: &str) -> Router {
    Router::new()
        .route("/health-user", get(alive))
        .route("/users/register", post(register_user)) // Changed route
        .route("/users/login", post(log_in_user)) // Changed route
        .route("/users", get(get_all_users))
        .route("/users/{id_user}", get(get_user_by_id).put(update_user)) // Combined get and put, changed path
        .route("/users/{id_user}/role", put(update_user_role)) // Changed path for role update
        .route("/users/{id_user}/verify-email", post(verify_email)) // New endpoint for email verification
        .with_state((user_service, token_key.to_string()))
}

async fn get_all_users(
    State((user_service, _)): State<(Arc<UserService>, String)>,
) -> HttpResult<Json<Vec<UserInfo>>> {
    // Return HttpResult
    let users = user_service
        .get_all_users()
        .await
        .http_err("get all users")?;
    Ok(Json(users))
}

async fn get_user_by_id(
    State((user_service, _)): State<(Arc<UserService>, String)>,
    Path(id_user): Path<Uuid>, // Changed path variable name
) -> HttpResult<Json<UserInfo>> {
    // Return HttpResult
    let user = user_service
        .get_user_by_id(id_user)
        .await
        .http_err("get user by id")?;
    Ok(Json(user))
}

#[derive(Deserialize)]
struct UpdateUserRolePayload {
    user_rol: URol,
}

async fn update_user_role(
    State((user_service, _)): State<(Arc<UserService>, String)>,
    Path(id_user): Path<Uuid>, // Changed path variable name
    Json(payload): Json<UpdateUserRolePayload>,
) -> HttpResult<Json<UserInfo>> {
    // Return updated UserInfo
    let updated_user = user_service
        .update_user_role(id_user, payload.user_rol)
        .await
        .http_err("update user role")?;
    Ok(Json(updated_user))
}

async fn update_user(
    State((user_service, _)): State<(Arc<UserService>, String)>,
    Path(id_user): Path<Uuid>, // Changed path variable name
    Json(user_update_payload): Json<UserCreation>, // Use UserCreation as DTO for update
) -> HttpResult<Json<UserInfo>> {
    // Return updated UserInfo
    let updated_user = user_service
        .update_user(id_user, user_update_payload)
        .await
        .http_err("update user")?;
    Ok(Json(updated_user))
}

async fn alive() -> Json<String> {
    // Simpler return for alive
    Json("User service is alive".to_string())
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct ApiLogInResponse {
    // Renamed to avoid conflict with service layer response
    token: String,
    user_id: Uuid,
    user_rol: URol,
}

async fn log_in_user(
    State((user_service, token_key)): State<(Arc<UserService>, String)>,
    Json(user_log_in_info): Json<UserLogInInfo>,
) -> HttpResult<Json<ApiLogInResponse>> {
    // Return HttpResult
    let log_in_service_response = user_service
        .log_in_user(&user_log_in_info)
        .await
        .http_err("log in user")?;

    let token = generate_jwt(&log_in_service_response, &token_key).map_err(|jwt_err| {
        error!("JWT generation error: {}", jwt_err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to generate token",
        )
            .into_response()
    })?;

    Ok(Json(ApiLogInResponse {
        token,
        user_id: log_in_service_response.user_id,
        user_rol: log_in_service_response.user_rol,
    }))
}

async fn register_user(
    State((user_service, _)): State<(Arc<UserService>, String)>,
    Json(user_creation): Json<UserCreation>,
) -> HttpResult<Json<UserInfo>> {
    // Return created UserInfo and 201 status
    let new_user_info = user_service
        .register_user(user_creation)
        .await
        .http_err("register user")?;
    Ok(Json(new_user_info)) // Axum will default to 200 OK, change to (StatusCode::CREATED, Json(...)) if needed
}

#[derive(Deserialize)]
struct VerifyEmailPayload {
    code: String,
}

async fn verify_email(
    State((user_service, _)): State<(Arc<UserService>, String)>,
    Path(id_user): Path<Uuid>,
    Json(payload): Json<VerifyEmailPayload>,
) -> HttpResult<impl IntoResponse> {
    user_service
        .verify_email_with_code(id_user, &payload.code)
        .await
        .http_err("verify email")?;
    Ok((StatusCode::OK, "Email verified successfully"))
}

// HttpError implementation for jsonwebtoken::Error
impl<T> HttpError<T> for Result<T, jsonwebtoken::errors::Error> {
    fn http_err(self, endpoint_name: &str) -> crate::err::HttpResult<T> {
        self.map_err(|err| {
            error!("Error in {} (jsonwebtoken): {}", endpoint_name, err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Token processing error".to_string(),
            )
                .into_response()
        })
    }
}

// HttpError implementation for UserServiceError
impl<T> HttpError<T> for Result<T, UserServiceError> {
    fn http_err(self, endpoint_name: &str) -> crate::err::HttpResult<T> {
        self.map_err(|err| {
            error!("Error in user endpoint ({}): {}", endpoint_name, err);
            let (status, msg) = match err {
                UserServiceError::UnknownDatabaseError(e) => {
                    error!("User DB error: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Database error processing user request.",
                    )
                }
                UserServiceError::UserIdDontExist => (StatusCode::NOT_FOUND, "User not found."),
                UserServiceError::ErrorHashing(_) | UserServiceError::ErrorVerificationHash(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error processing password.",
                ),
                UserServiceError::InvalidPassword => {
                    (StatusCode::UNAUTHORIZED, "Invalid credentials.")
                }
                UserServiceError::EmailAlreadyExists => {
                    (StatusCode::CONFLICT, "Email already in use.")
                }
                UserServiceError::PhoneAlreadyExists => {
                    (StatusCode::CONFLICT, "Phone number already in use.")
                }
                UserServiceError::DocumentAlreadyExists => {
                    (StatusCode::CONFLICT, "Document already in use.")
                }
                UserServiceError::InvalidIdentifier => {
                    (StatusCode::BAD_REQUEST, "Invalid identifier for login.")
                }
                UserServiceError::InvalidBirthDate(_) => {
                    (StatusCode::BAD_REQUEST, "Invalid birth date")
                }
            };
            (status, msg.to_string()).into_response()
        })
    }
}
