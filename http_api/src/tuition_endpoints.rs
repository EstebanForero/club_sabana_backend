use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use entities::tuition::Tuition;
use tracing::error;
use use_cases::tuition_service::{err::Error, TuitionService};
use uuid::Uuid;

use super::err::{HttpError, ToErrResponse};
use crate::{
    auth::{auth_middleware, UserInfoAuth},
    err::HttpResult,
};

pub fn tuition_router(tuition_service: TuitionService, jwt_key: String) -> Router {
    Router::new()
        .route("/health-tuition", get(alive))
        .route("/tuitions/pay/{amount}", post(pay_tuition))
        .route("/tuitions", get(list_tuitions))
        .route("/tuitions/{user_id}", get(list_user_tuitions))
        .route("/tuitions/active/{user_id}", get(has_active_tuition))
        .layer(middleware::from_fn_with_state(jwt_key, auth_middleware))
        .with_state(tuition_service)
}

async fn alive() -> &'static str {
    "Tuition service is alive"
}
async fn pay_tuition(
    State(tuition_service): State<TuitionService>,
    Path(amount): Path<f64>,
    Extension(user_info): Extension<UserInfoAuth>,
) -> HttpResult<impl IntoResponse> {
    tuition_service
        .pay_tuition(user_info.user_id, amount)
        .await
        .http_err("pay tuition")?;

    Ok((StatusCode::CREATED, "Tuition payment recorded successfully"))
}

async fn list_tuitions(
    State(tuition_service): State<TuitionService>,
) -> HttpResult<Json<Vec<Tuition>>> {
    let tuitions = tuition_service
        .get_all_tuitions()
        .await
        .http_err("list tuitions")?;

    Ok(Json(tuitions))
}

async fn list_user_tuitions(
    State(tuition_service): State<TuitionService>,
    Path(user_id): Path<Uuid>,
) -> HttpResult<Json<Vec<Tuition>>> {
    let tuitions = tuition_service
        .get_user_tuitions(user_id)
        .await
        .http_err("list user tuitions")?;

    Ok(Json(tuitions))
}

async fn has_active_tuition(
    State(tuition_service): State<TuitionService>,
    Path(user_id): Path<Uuid>,
) -> HttpResult<Json<bool>> {
    let has_active = tuition_service
        .has_active_tuition(user_id)
        .await
        .http_err("check active tuition")?;

    Ok(Json(has_active))
}

impl<T> HttpError<T> for use_cases::tuition_service::err::Result<T> {
    fn http_err(self, endpoint_name: &str) -> crate::err::HttpResult<T> {
        self.map_err(|err| {
            error!("Error in: {endpoint_name}");
            match err {
                Error::UnknownDatabaseError(error) => {
                    error!("{error}");
                    "We are having problems in the server, try again"
                }
                Error::ActiveTuitionExists => "Active tuition already exists",
                Error::InvalidAmount => "Invalid payment amount",
                Error::TuitionNotFound => "Tuition not found",
            }
            .to_err_response()
        })
    }
}
