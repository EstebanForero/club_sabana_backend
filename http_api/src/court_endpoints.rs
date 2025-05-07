use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use entities::court::{Court, CourtCreation};
use tracing::error;
use use_cases::court_service::{err::Error as CourtServiceError, CourtService};
use uuid::Uuid;

use super::err::HttpError;
use crate::{
    auth::auth_middleware, // Assuming some endpoints might need auth
    err::HttpResult,
};

pub fn court_router(court_service: CourtService, jwt_key: String) -> Router {
    Router::new()
        .route("/health-court", get(alive))
        .route("/courts", post(create_court).get(list_courts))
        .route("/courts/{id_court}", get(get_court).delete(delete_court))
        // .route("/courts/{id_court}/reservations", post(create_reservation_for_court))
        // .route("/courts/reservations/{id_reservation}", get(get_reservation).delete(delete_reservation))
        // For now, court reservation is handled internally by training/tournament services.
        .route_layer(middleware::from_fn_with_state(jwt_key, auth_middleware)) // Apply auth to all court routes for now
        .with_state(court_service)
}

async fn alive() -> &'static str {
    "Court service is alive"
}

async fn create_court(
    State(court_service): State<CourtService>,
    Json(court_creation): Json<CourtCreation>,
) -> HttpResult<Json<Court>> {
    let court = court_service
        .create_court(court_creation)
        .await
        .http_err("create court")?;
    Ok(Json(court))
}

async fn get_court(
    State(court_service): State<CourtService>,
    Path(id_court): Path<Uuid>,
) -> HttpResult<Json<Court>> {
    let court = court_service
        .get_court(id_court)
        .await
        .http_err("get court")?;
    Ok(Json(court))
}

async fn list_courts(State(court_service): State<CourtService>) -> HttpResult<Json<Vec<Court>>> {
    let courts = court_service.list_courts().await.http_err("list courts")?;
    Ok(Json(courts))
}

async fn delete_court(
    State(court_service): State<CourtService>,
    Path(id_court): Path<Uuid>,
) -> HttpResult<impl IntoResponse> {
    court_service
        .delete_court(id_court)
        .await
        .http_err("delete court")?;
    Ok((StatusCode::OK, "Court deleted successfully"))
}

impl<T> HttpError<T> for Result<T, CourtServiceError> {
    fn http_err(self, endpoint_name: &str) -> crate::err::HttpResult<T> {
        self.map_err(|err| {
            error!("Error in court endpoint ({}): {}", endpoint_name, err);
            let (status, msg) = match err {
                CourtServiceError::UnknownDatabaseError(e) => {
                    error!("Court DB error: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Database error processing court request.",
                    )
                }
                CourtServiceError::CourtNotFound => (StatusCode::NOT_FOUND, "Court not found."),
                CourtServiceError::CourtNameExists => {
                    (StatusCode::CONFLICT, "Court name already exists.")
                }
                CourtServiceError::CourtUnavailable => (
                    StatusCode::CONFLICT,
                    "Court is unavailable for the selected time.",
                ),
                CourtServiceError::InvalidReservationTime => {
                    (StatusCode::BAD_REQUEST, "Invalid reservation time range.")
                }
                CourtServiceError::ReservationPurposeMissing => (
                    StatusCode::BAD_REQUEST,
                    "Reservation must be for a training or tournament.",
                ),
                CourtServiceError::ReservationNotFound => {
                    (StatusCode::NOT_FOUND, "Court reservation not found.")
                }
                CourtServiceError::ReservationPurposeConflict => (
                    StatusCode::BAD_REQUEST,
                    "Reservation cannot be for both training and tournament.",
                ),
            };
            (status, msg.to_string()).into_response()
        })
    }
}
