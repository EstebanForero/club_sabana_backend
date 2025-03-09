use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post, put},
    Json, Router,
};
use entities::tournament::{
    Tournament, TournamentAttendance, TournamentCreation, TournamentRegistration,
};
use tracing::error;
use use_cases::tournament_service::{err::Error, TournamentService};
use uuid::Uuid;

use crate::err::{HttpError, HttpResult, ToErrResponse};

pub fn tournament_router(tournament_service: TournamentService) -> Router {
    Router::new()
        .route("/health-tournament", get(alive))
        .route(
            "/tournaments",
            post(create_tournament).get(list_tournaments),
        )
        .route(
            "/tournaments/{id}",
            get(get_tournament)
                .put(update_tournament)
                .delete(delete_tournament),
        )
        .route("/tournaments/{id}/register", post(register_user))
        .route("/tournaments/{id}/attendance", post(record_attendance))
        .route("/tournaments/{id}/position", put(update_position))
        .route(
            "/users/{id}/eligible-tournaments",
            get(get_eligible_tournaments),
        )
        .with_state(tournament_service)
}

async fn alive() -> &'static str {
    "Tournament service is alive"
}

async fn create_tournament(
    State(tournament_service): State<TournamentService>,
    Json(tournament): Json<TournamentCreation>,
) -> HttpResult<impl IntoResponse> {
    tournament_service
        .create_tournament(tournament)
        .await
        .http_err("create tournament")?;

    Ok((StatusCode::OK, "Tournament created successfully"))
}

async fn get_tournament(
    State(tournament_service): State<TournamentService>,
    Path(id): Path<Uuid>,
) -> Result<Json<Tournament>, Response> {
    let tournament = tournament_service
        .get_tournament(id)
        .await
        .http_err("get tournament")?;

    Ok(Json(tournament))
}

async fn update_tournament(
    State(tournament_service): State<TournamentService>,
    Json(tournament): Json<Tournament>,
) -> Result<(), Response> {
    tournament_service
        .update_tournament(tournament)
        .await
        .http_err("update tournament")?;

    Ok(())
}

async fn delete_tournament(
    State(tournament_service): State<TournamentService>,
    Path(id): Path<Uuid>,
) -> Result<Json<String>, Response> {
    tournament_service
        .delete_tournament(id)
        .await
        .http_err("delete tournament")?;

    Ok(Json("Tournament deleted successfully".to_string()))
}

async fn list_tournaments(
    State(tournament_service): State<TournamentService>,
) -> Result<Json<Vec<Tournament>>, Response> {
    let tournaments = tournament_service
        .list_tournaments()
        .await
        .http_err("list tournaments")?;

    let tournaments_dto = tournaments.into_iter().collect();
    Ok(Json(tournaments_dto))
}

async fn register_user(
    State(tournament_service): State<TournamentService>,
    Json(registration): Json<TournamentRegistration>,
) -> Result<Json<String>, Response> {
    tournament_service
        .register_user(registration)
        .await
        .http_err("register user")?;

    Ok(Json("User registered successfully".to_string()))
}

async fn record_attendance(
    State(tournament_service): State<TournamentService>,
    Json(attendance): Json<TournamentAttendance>,
) -> Result<Json<String>, Response> {
    tournament_service
        .record_attendance(attendance)
        .await
        .http_err("record attendance")?;

    Ok(Json("Attendance recorded successfully".to_string()))
}

async fn get_eligible_tournaments(
    State(tournament_service): State<TournamentService>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<Tournament>>, Response> {
    let tournaments = tournament_service
        .get_eligible_tournaments(user_id)
        .await
        .http_err("get eligible tournaments")?;

    Ok(Json(tournaments))
}

async fn update_position(
    State(tournament_service): State<TournamentService>,
    Path((tournament_id, user_id)): Path<(Uuid, Uuid)>,
    Json(position): Json<i32>,
) -> Result<Json<String>, Response> {
    tournament_service
        .update_position(tournament_id, user_id, position)
        .await
        .http_err("update position")?;

    Ok(Json("Position updated successfully".to_string()))
}

impl<T> HttpError<T> for use_cases::tournament_service::err::Result<T> {
    fn http_err(self, endpoint_name: &str) -> crate::err::HttpResult<T> {
        self.map_err(|err| {
            error!("Error in: {endpoint_name}");
            match err {
                Error::UnknownDatabaseError(error) => {
                    error!("{error}");
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
                Error::UserDoesNotMeetCategoryRequirements => {
                    "User is not part of the category required to join the tournament"
                }
                Error::CategoryServiceError(error) => "Error in the category service",
            }
            .to_err_response()
        })
    }
}
