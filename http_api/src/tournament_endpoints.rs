use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use entities::tournament::{
    Tournament, TournamentAttendance, TournamentCreation, TournamentRegistration,
};
use serde::Deserialize;
use tracing::error;
use use_cases::tournament_service::{err::Error, TournamentService};
use uuid::Uuid;

use crate::err::{HttpError, HttpResult};

#[derive(Debug, Deserialize)]
pub struct TournamentCreationPayload {
    #[serde(flatten)]
    pub tournament_data: TournamentCreation,
    pub id_court: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct TournamentUpdatePayload {
    #[serde(flatten)]
    pub tournament_data: TournamentCreation,
    pub id_court: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePositionPayload {
    pub position: i32,
}

pub fn tournament_router(tournament_service: TournamentService) -> Router {
    Router::new()
        .route("/health-tournament", get(alive))
        .route(
            "/tournaments",
            post(create_tournament).get(list_tournaments),
        )
        .route(
            "/tournaments/{id_tournament}",
            get(get_tournament)
                .put(update_tournament)
                .delete(delete_tournament),
        )
        .route(
            "/tournaments/{id_tournament}/register",
            post(register_user_for_tournament),
        )
        .route(
            "/tournaments/{id_tournament}/attendance",
            post(record_attendance),
        )
        .route(
            "/tournaments/{id_tournament}/users/{id_user}/position",
            put(update_user_position_in_tournament),
        )
        .route(
            "/tournaments/registrations/user/{user_id}",
            get(get_user_registrations),
        )
        .route(
            "/tournaments/registrations/tournament/{id_tournament}",
            get(get_tournament_registrations),
        )
        .route(
            "/users/{user_id}/eligible-tournaments",
            get(get_eligible_tournaments_for_user),
        )
        .route(
            "/tournaments/{id_tournament}/attendance",
            get(get_tournament_attendance_list),
        )
        .route(
            "/users/{user_id}/tournament-attendance",
            get(get_user_tournament_attendance_list),
        )
        .route(
            "/tournaments/{id_tournament}/attendance/{user_id}",
            delete(delete_user_attendance_from_tournament),
        )
        .route(
            "/tournaments/{id_tournament}/registrations/{user_id}",
            delete(delete_user_registration_from_tournament),
        )
        .with_state(tournament_service)
}

async fn alive() -> &'static str {
    "Tournament service is alive"
}

async fn delete_user_attendance_from_tournament(
    State(tournament_service): State<TournamentService>,
    Path((id_tournament, user_id)): Path<(Uuid, Uuid)>,
) -> HttpResult<impl IntoResponse> {
    tournament_service
        .delete_attendance(id_tournament, user_id)
        .await
        .http_err("delete attendance")?;
    Ok((StatusCode::OK, "Attendance deleted successfully"))
}

async fn delete_user_registration_from_tournament(
    State(tournament_service): State<TournamentService>,
    Path((id_tournament, user_id)): Path<(Uuid, Uuid)>,
) -> HttpResult<impl IntoResponse> {
    tournament_service
        .delete_registration(id_tournament, user_id)
        .await
        .http_err("delete registration")?;
    Ok((StatusCode::OK, "Registration deleted successfully"))
}

async fn create_tournament(
    State(tournament_service): State<TournamentService>,
    Json(payload): Json<TournamentCreationPayload>,
) -> HttpResult<Json<Tournament>> {
    let created_tournament = tournament_service
        .create_tournament(payload.tournament_data, payload.id_court)
        .await
        .http_err("create tournament")?;
    Ok(Json(created_tournament))
}

async fn get_tournament(
    State(tournament_service): State<TournamentService>,
    Path(id_tournament): Path<Uuid>,
) -> HttpResult<Json<Tournament>> {
    let tournament = tournament_service
        .get_tournament(id_tournament)
        .await
        .http_err("get tournament")?;
    Ok(Json(tournament))
}

async fn update_tournament(
    State(tournament_service): State<TournamentService>,
    Path(id_tournament): Path<Uuid>,
    Json(payload): Json<TournamentUpdatePayload>,
) -> HttpResult<Json<Tournament>> {
    let updated_tournament = tournament_service
        .update_tournament(id_tournament, payload.tournament_data, payload.id_court)
        .await
        .http_err("update tournament")?;
    Ok(Json(updated_tournament))
}

async fn delete_tournament(
    State(tournament_service): State<TournamentService>,
    Path(id_tournament): Path<Uuid>,
) -> HttpResult<impl IntoResponse> {
    tournament_service
        .delete_tournament(id_tournament)
        .await
        .http_err("delete tournament")?;
    Ok((StatusCode::OK, "Tournament deleted successfully"))
}

async fn list_tournaments(
    State(tournament_service): State<TournamentService>,
) -> HttpResult<Json<Vec<Tournament>>> {
    let tournaments = tournament_service
        .list_tournaments()
        .await
        .http_err("list tournaments")?;
    Ok(Json(tournaments))
}

async fn register_user_for_tournament(
    State(tournament_service): State<TournamentService>,
    Path(id_tournament): Path<Uuid>,
    Json(mut registration_payload): Json<TournamentRegistration>,
) -> HttpResult<Json<TournamentRegistration>> {
    registration_payload.id_tournament = id_tournament;

    let registration = tournament_service
        .register_user(registration_payload)
        .await
        .http_err("register user for tournament")?;
    Ok(Json(registration))
}

async fn record_attendance(
    State(tournament_service): State<TournamentService>,
    Path(id_tournament): Path<Uuid>,
    Json(mut attendance_payload): Json<TournamentAttendance>,
) -> HttpResult<Json<TournamentAttendance>> {
    attendance_payload.id_tournament = id_tournament;

    let attendance = tournament_service
        .record_attendance(attendance_payload)
        .await
        .http_err("record attendance")?;
    Ok(Json(attendance))
}

async fn get_eligible_tournaments_for_user(
    State(tournament_service): State<TournamentService>,
    Path(user_id): Path<Uuid>,
) -> HttpResult<Json<Vec<Tournament>>> {
    let tournaments = tournament_service
        .get_eligible_tournaments(user_id)
        .await
        .http_err("get eligible tournaments for user")?;
    Ok(Json(tournaments))
}

async fn get_tournament_attendance_list(
    State(tournament_service): State<TournamentService>,
    Path(id_tournament): Path<Uuid>,
) -> HttpResult<Json<Vec<TournamentAttendance>>> {
    let attendance_list = tournament_service
        .get_tournament_attendance(id_tournament)
        .await
        .http_err("get tournament attendance list")?;
    Ok(Json(attendance_list))
}

async fn get_user_tournament_attendance_list(
    State(tournament_service): State<TournamentService>,
    Path(user_id): Path<Uuid>,
) -> HttpResult<Json<Vec<TournamentAttendance>>> {
    let attendance_list = tournament_service
        .get_user_attendance(user_id)
        .await
        .http_err("get user tournament attendance list")?;
    Ok(Json(attendance_list))
}

async fn get_user_registrations(
    State(tournament_service): State<TournamentService>,
    Path(user_id): Path<Uuid>,
) -> HttpResult<Json<Vec<TournamentRegistration>>> {
    let registrations = tournament_service
        .get_user_registrations(user_id)
        .await
        .http_err("get user registrations")?;
    Ok(Json(registrations))
}

async fn get_tournament_registrations(
    State(tournament_service): State<TournamentService>,
    Path(id_tournament): Path<Uuid>,
) -> HttpResult<Json<Vec<TournamentRegistration>>> {
    let registrations = tournament_service
        .get_tournament_registrations(id_tournament)
        .await
        .http_err("get tournament registrations")?;
    Ok(Json(registrations))
}

async fn update_user_position_in_tournament(
    State(tournament_service): State<TournamentService>,
    Path((id_tournament, user_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdatePositionPayload>,
) -> HttpResult<impl IntoResponse> {
    tournament_service
        .update_position(id_tournament, user_id, payload.position)
        .await
        .http_err("update user position in tournament")?;
    Ok((StatusCode::OK, "Position updated successfully"))
}

impl<T> HttpError<T> for Result<T, Error> {
    fn http_err(self, endpoint_name: &str) -> crate::err::HttpResult<T> {
        self.map_err(|err| {
            error!("Error in tournament endpoint ({}): {}", endpoint_name, err);
            let (status_code, message) = match err {
                Error::UnknownDatabaseError(e) => {
                    error!("Tournament DB error: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Database error processing tournament request.",
                    )
                }
                Error::TournamentNotFound => (StatusCode::NOT_FOUND, "Tournament not found."),
                Error::UserNotRegistered => (
                    StatusCode::NOT_FOUND,
                    "User not registered for this tournament.",
                ),
                Error::UserAlreadyRegistered => (
                    StatusCode::CONFLICT,
                    "User already registered for this tournament.",
                ),
                Error::InvalidDates => (
                    StatusCode::BAD_REQUEST,
                    "Invalid tournament dates or duration.",
                ),
                Error::InvalidCategory => {
                    (StatusCode::BAD_REQUEST, "Invalid category for tournament.")
                }
                Error::NegativePosition => (
                    StatusCode::BAD_REQUEST,
                    "Position must be a positive integer.",
                ),
                Error::PositionAlreadyTaken => (
                    StatusCode::CONFLICT,
                    "This position is already taken in the tournament.",
                ),
                Error::UserDidNotAttend => (
                    StatusCode::NOT_FOUND,
                    "User did not attend this tournament.",
                ),
                Error::UserDoesNotMeetCategoryRequirements => (
                    StatusCode::FORBIDDEN,
                    "User does not meet category requirements for this tournament.",
                ),
                Error::CategoryServiceError(e) => {
                    error!("Category service error via tournament: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal error with category service.",
                    )
                }
                Error::CourtServiceError(e) => {
                    error!("Court service error via tournament: {}", e);
                    match e {
                        use_cases::court_service::err::Error::CourtUnavailable => (
                            StatusCode::CONFLICT,
                            "Selected court is unavailable for the tournament time.",
                        ),
                        use_cases::court_service::err::Error::CourtNotFound => {
                            (StatusCode::BAD_REQUEST, "Selected court not found.")
                        }
                        _ => (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Internal error with court service.",
                        ),
                    }
                }
            };
            (status_code, message.to_string()).into_response()
        })
    }
}
