use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post, put},
    Json, Router, ServiceExt,
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
            "/tournaments/registrations/user/{user_id}",
            get(get_user_registrations),
        )
        .route(
            "/tournaments/registrations/tournament/{tournament_id}",
            get(get_tournament_registrations),
        )
        .route(
            "/tournaments/users/{id}/eligible-tournaments",
            get(get_eligible_tournaments),
        )
        .route(
            "/tournaments/{id}/attendance",
            get(get_tournament_attendance),
        )
        .route(
            "/tournaments/users/{id}/attendance",
            get(get_user_attendance),
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

async fn get_tournament_attendance(
    State(tournament_service): State<TournamentService>,
    Path(tournament_id): Path<Uuid>,
) -> Result<Json<Vec<TournamentAttendance>>, Response> {
    let attendance = tournament_service
        .get_tournament_attendance(tournament_id)
        .await
        .http_err("get tournament attendance")?;

    Ok(Json(attendance))
}

async fn get_user_attendance(
    State(tournament_service): State<TournamentService>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<TournamentAttendance>>, Response> {
    let attendance = tournament_service
        .get_user_attendance(user_id)
        .await
        .http_err("get user attendance")?;

    Ok(Json(attendance))
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
    Path(tournament_id): Path<Uuid>,
) -> HttpResult<Json<Vec<TournamentRegistration>>> {
    let registrations = tournament_service
        .get_tournament_registrations(tournament_id)
        .await
        .http_err("get tournament registrations")?;
    Ok(Json(registrations))
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
                Error::CategoryServiceError(_) => "Error in the category service",
            }
            .to_err_response()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module
    use chrono::NaiveDateTime;
    use serde_json;
    use uuid::Uuid;

    // Define TournamentCreation if not directly accessible
    #[test]
    fn test_deserialize_tournament_creation_success() {
        let json = r#"
        {
            "name": "Summer championship",
            "id_category": "346a12fb-914d-412a-a39e-e14d00a69ac9",
            "start_datetime": "2006-02-19 08:00:00",
            "end_datetime": "2006-02-19 10:00:00"
        }
        "#;

        let result: Result<TournamentCreation, _> = serde_json::from_str(json);
        assert!(result.is_ok(), "Deserialization failed: {:?}", result.err());

        let tournament = result.unwrap();
        assert_eq!(tournament.name, "Summer championship");
        assert_eq!(
            tournament.id_category,
            Uuid::parse_str("346a12fb-914d-412a-a39e-e14d00a69ac9").unwrap()
        );
        assert_eq!(
            tournament.start_datetime,
            NaiveDateTime::parse_from_str("2006-02-19 08:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
        );
        assert_eq!(
            tournament.end_datetime,
            NaiveDateTime::parse_from_str("2006-02-19 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
        );
    }

    #[test]
    fn test_deserialize_tournament_creation_success_should() {
        let id_category = Uuid::new_v4();

        let tournament_creation = TournamentCreation {
            name: "Summer championship".to_string(),
            id_category,
            start_datetime: NaiveDateTime::parse_from_str(
                "2006-02-19 08:00:00",
                "%Y-%m-%d %H:%M:%S",
            )
            .expect("Error parsing date from string"),
            end_datetime: NaiveDateTime::parse_from_str("2006-02-19 10:00:00", "%Y-%m-%d %H:%M:%S")
                .expect("Error parsing from string"),
        };

        let json = serde_json::to_string(&tournament_creation)
            .expect("Failed to serialize tournament data");

        println!("The json file is: {json}");

        let result: Result<TournamentCreation, _> = serde_json::from_str(&json);
        assert!(result.is_ok(), "Deserialization failed: {:?}", result.err());

        let tournament = result.unwrap();
        assert_eq!(tournament.name, "Summer championship");
        assert_eq!(tournament.id_category, id_category);
        assert_eq!(
            tournament.start_datetime,
            NaiveDateTime::parse_from_str("2006-02-19 08:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
        );
        assert_eq!(
            tournament.end_datetime,
            NaiveDateTime::parse_from_str("2006-02-19 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
        );
    }

    #[test]
    fn test_deserialize_tournament_creation_invalid_datetime() {
        // Simulate a common invalid format (ISO 8601 with 'T')
        let json = r#"
        {
            "name": "Summer championship",
            "id_category": "346a12fb-914d-412a-a39e-e14d00a69ac9",
            "start_datetime": "2006-02-19T08:00:00",
            "end_datetime": "2006-02-19 10:00:00"
        }
        "#;

        let result: Result<TournamentCreation, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Deserialization should have failed but succeeded"
        );
        let error = result.unwrap_err();
        let error_msg = error.to_string();
        assert!(
            error_msg.contains("start_datetime"),
            "Error should mention 'start_datetime', got: {}",
            error_msg
        );
        assert!(
            error_msg.contains("invalid characters"),
            "Error should mention 'invalid characters', got: {}",
            error_msg
        );
    }
}
