use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router,
};
use entities::tournament::{Tournament, TournamentAttendance, TournamentRegistration};
use serde::{Deserialize, Serialize};
use tracing::error;
use use_cases::tournament_service::{err::Error, TournamentService};
use uuid::Uuid;

fn internal_error_response(message: &str) -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, message.to_string()).into_response()
}

pub fn tournament_router(tournament_service: TournamentService) -> Router {
    Router::new()
        .route("/health", get(alive))
        .route(
            "/tournaments",
            post(create_tournament).get(list_tournaments),
        )
        .route(
            "/tournaments/:id",
            get(get_tournament)
                .put(update_tournament)
                .delete(delete_tournament),
        )
        .route("/tournaments/:id/register", post(register_user))
        .route("/tournaments/:id/attendance", post(record_attendance))
        .route("/tournaments/:id/position", put(update_position))
        .with_state(tournament_service)
}

async fn alive() -> Result<Json<String>, Response> {
    Ok(Json("Tournament router is alive".to_string()))
}

async fn create_tournament(
    State(tournament_service): State<TournamentService>,
    Json(tournament): Json<Tournament>,
) -> Result<Json<Tournament>, Response> {
    tournament_service
        .create_tournament(tournament)
        .await
        .map_err(|err| internal_error_response(&message_from_err(err, "create tournament")))?;

    Ok(Json(tournament))
}

async fn get_tournament(
    State(tournament_service): State<TournamentService>,
    Path(id): Path<Uuid>,
) -> Result<Json<Tournament>, Response> {
    let tournament = tournament_service
        .get_tournament(id)
        .await
        .map_err(|err| internal_error_response(&message_from_err(err, "get tournament")))?;

    Ok(Json(tournament))
}

async fn update_tournament(
    State(tournament_service): State<TournamentService>,
    Path(id): Path<Uuid>,
    Json(tournament): Json<Tournament>,
) -> Result<Json<Tournament>, Response> {
    tournament_service
        .update_tournament(tournament)
        .await
        .map_err(|err| internal_error_response(&message_from_err(err, "update tournament")))?;

    Ok(Json(tournament))
}

async fn delete_tournament(
    State(tournament_service): State<TournamentService>,
    Path(id): Path<Uuid>,
) -> Result<Json<String>, Response> {
    tournament_service
        .delete_tournament(id)
        .await
        .map_err(|err| internal_error_response(&message_from_err(err, "delete tournament")))?;

    Ok(Json("Tournament deleted successfully".to_string()))
}

async fn list_tournaments(
    State(tournament_service): State<TournamentService>,
) -> Result<Json<Vec<Tournament>>, Response> {
    let tournaments = tournament_service
        .list_tournaments()
        .await
        .map_err(|err| internal_error_response(&message_from_err(err, "list tournaments")))?;

    Ok(Json(tournaments))
}

async fn register_user(
    State(tournament_service): State<TournamentService>,
    Path(tournament_id): Path<Uuid>,
    Json(registration): Json<TournamentRegistration>,
) -> Result<Json<String>, Response> {
    tournament_service
        .register_user(registration)
        .await
        .map_err(|err| internal_error_response(&message_from_err(err, "register user")))?;

    Ok(Json("User registered successfully".to_string()))
}

async fn record_attendance(
    State(tournament_service): State<TournamentService>,
    Path(tournament_id): Path<Uuid>,
    Json(attendance): Json<TournamentAttendance>,
) -> Result<Json<String>, Response> {
    tournament_service
        .record_attendance(attendance)
        .await
        .map_err(|err| internal_error_response(&message_from_err(err, "record attendance")))?;

    Ok(Json("Attendance recorded successfully".to_string()))
}

async fn update_position(
    State(tournament_service): State<TournamentService>,
    Path((tournament_id, user_id)): Path<(Uuid, Uuid)>,
    Json(position): Json<i32>,
) -> Result<Json<String>, Response> {
    tournament_service
        .update_position(tournament_id, user_id, position)
        .await
        .map_err(|err| internal_error_response(&message_from_err(err, "update position")))?;

    Ok(Json("Position updated successfully".to_string()))
}

fn message_from_err(err: Error, endpoint_name: &str) -> String {
    let error_msg = match err {
        Error::DatabaseError(error) => {
            error!("{endpoint_name}: {error}");
            "We are having problems in the server, try again"
        }
        Error::TournamentNotFound => "Tournament not found",
        Error::UserNotRegistered => "User not registered for tournament",
        Error::UserAlreadyRegistered => "User already registered",
        Error::InvalidDates => "Invalid tournament dates",
        Error::InvalidCategory => "Invalid category",
        Error::NegativePosition => "Position must be positive",
        Error::PositionAlreadyTaken => "Position already taken",
        Error::UserDidNotAttend => "User did not attend tournament",
    };

    error!("error in {endpoint_name}: {error_msg}");

    error_msg.to_string()
}
