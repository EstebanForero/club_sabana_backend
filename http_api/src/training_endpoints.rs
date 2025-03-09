use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post, put},
    Json, Router,
};
use entities::training::{Training, TrainingRegistration};
use tracing::error;
use use_cases::training_service::{err::Error, TrainingService};
use uuid::Uuid;

use crate::err::{HttpError, HttpResult, ToErrResponse};

pub fn training_router(training_service: TrainingService) -> Router {
    Router::new()
        .route("/health-training", get(alive))
        .route("/trainings", post(create_training).get(list_trainings))
        .route(
            "/trainings/:id",
            get(get_training)
                .put(update_training)
                .delete(delete_training),
        )
        .route("/trainings/:id/register", post(register_user))
        .route("/trainings/:id/attendance", post(mark_attendance))
        .route("/users/:id/eligible-trainings", get(get_eligible_trainings))
        .with_state(training_service)
}

async fn alive() -> &'static str {
    "Training service is alive"
}

async fn create_training(
    State(training_service): State<TrainingService>,
    Json(training): Json<Training>,
) -> HttpResult<impl IntoResponse> {
    training_service
        .create_training(&training)
        .await
        .http_err("create training")?;

    Ok((StatusCode::OK, "Training created successfully"))
}

async fn get_training(
    State(training_service): State<TrainingService>,
    Path(id): Path<Uuid>,
) -> Result<Json<Training>, Response> {
    let training = training_service
        .get_training(id)
        .await
        .http_err("get training")?;

    Ok(Json(training))
}

async fn update_training(
    State(training_service): State<TrainingService>,
    Json(training): Json<Training>,
) -> Result<(), Response> {
    training_service
        .update_training(&training)
        .await
        .http_err("update training")?;

    Ok(())
}

async fn delete_training(
    State(training_service): State<TrainingService>,
    Path(id): Path<Uuid>,
) -> Result<Json<String>, Response> {
    training_service
        .delete_training(id)
        .await
        .http_err("delete training")?;

    Ok(Json("Training deleted successfully".to_string()))
}

async fn list_trainings(
    State(training_service): State<TrainingService>,
) -> Result<Json<Vec<Training>>, Response> {
    let trainings = training_service
        .list_trainings()
        .await
        .http_err("list trainings")?;

    Ok(Json(trainings))
}

async fn register_user(
    State(training_service): State<TrainingService>,
    Json(registration): Json<TrainingRegistration>,
) -> Result<Json<String>, Response> {
    training_service
        .register_user(registration)
        .await
        .http_err("register user")?;

    Ok(Json("User registered successfully".to_string()))
}

async fn mark_attendance(
    State(training_service): State<TrainingService>,
    Path((training_id, user_id)): Path<(Uuid, Uuid)>,
    Json(attended): Json<bool>,
) -> Result<Json<String>, Response> {
    training_service
        .mark_attendance(training_id, user_id, attended)
        .await
        .http_err("mark attendance")?;

    Ok(Json("Attendance marked successfully".to_string()))
}

async fn get_eligible_trainings(
    State(training_service): State<TrainingService>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<Training>>, Response> {
    let trainings = training_service
        .get_eligible_trainings(user_id)
        .await
        .http_err("get eligible trainings")?;

    Ok(Json(trainings))
}

impl<T> HttpError<T> for use_cases::training_service::err::Result<T> {
    fn http_err(self, endpoint_name: &str) -> crate::err::HttpResult<T> {
        self.map_err(|err| {
            error!("Error in: {endpoint_name}");
            match err {
                Error::UnknownDatabaseError(error) => {
                    error!("{error}");
                    "We are having problems in the server, try again"
                }
                Error::TrainingNotFound => "Training not found",
                Error::UserAlreadyRegistered => "User already registered for this training",
                Error::UserDoesNotMeetCategoryRequirements => {
                    "User does not meet category requirements for this training"
                }
                Error::InvalidDates => "Invalid training dates",
                Error::UserNotRegistered => "User not registered for this training",
                Error::RegistrationNotFound => "Training registration not found",
                Error::CategoryServiceError(_) => "Error in the category service",
            }
            .to_err_response()
        })
    }
}
