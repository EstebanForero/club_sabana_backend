use axum::{
    extract::{Path, State}, // Added Query
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post}, // Added put
    Json,
    Router,
};
use entities::training::{Training, TrainingCreation, TrainingRegistration};
use serde::Deserialize; // Added
use tracing::error;
use use_cases::training_service::{err::Error, TrainingService};
use uuid::Uuid;

use crate::err::{HttpError, HttpResult};

// DTO for training creation that includes optional court ID
#[derive(Debug, Deserialize)]
pub struct TrainingCreationPayload {
    #[serde(flatten)]
    pub training_data: TrainingCreation,
    pub id_court: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct TrainingUpdatePayload {
    // Similar for updates
    #[serde(flatten)]
    pub training_data: TrainingCreation, // Re-use TrainingCreation for update fields
    pub id_court: Option<Uuid>,
}

pub fn training_router(training_service: TrainingService) -> Router {
    Router::new()
        .route("/health-training", get(alive))
        .route("/trainings", post(create_training).get(list_trainings))
        .route(
            "/trainings/{id}",
            get(get_training)
                .put(update_training) // Added PUT
                .delete(delete_training),
        )
        .route(
            "/trainings/{id}/register/{user_id}",
            post(register_user_for_training),
        ) // Renamed for clarity
        .route(
            "/trainings/{training_id}/attendance/{user_id}",
            post(mark_attendance),
        )
        .route(
            "/users/{id}/eligible-trainings",
            get(get_eligible_trainings),
        )
        .route(
            "/users/{id}/training-registrations",
            get(get_user_training_registrations),
        )
        .route(
            "/trainings/{id}/registrations",
            get(get_training_registrations),
        )
        .route(
            "/trainings/{training_id}/registrations/{user_id}",
            delete(delete_training_registration),
        )
        .route(
            // New route for trainings by trainer
            "/trainers/{trainer_id}/trainings",
            get(get_trainings_by_trainer),
        )
        .with_state(training_service)
}

async fn alive() -> &'static str {
    "Training service is alive"
}

async fn get_trainings_by_trainer(
    State(training_service): State<TrainingService>,
    Path(trainer_id): Path<Uuid>,
) -> HttpResult<Json<Vec<Training>>> {
    let trainings = training_service
        .get_trainings_by_trainer(trainer_id)
        .await
        .http_err("get trainings by trainer")?;
    Ok(Json(trainings))
}

async fn get_user_training_registrations(
    State(training_service): State<TrainingService>,
    Path(user_id): Path<Uuid>,
) -> HttpResult<Json<Vec<TrainingRegistration>>> {
    // Return type changed to HttpResult
    let registrations = training_service
        .get_user_training_registrations(user_id)
        .await
        .http_err("get user training registrations")?;
    Ok(Json(registrations))
}

async fn get_training_registrations(
    State(training_service): State<TrainingService>,
    Path(training_id): Path<Uuid>,
) -> HttpResult<Json<Vec<TrainingRegistration>>> {
    // Return type changed
    let registrations = training_service
        .get_training_registrations(training_id)
        .await
        .http_err("get training registrations")?;
    Ok(Json(registrations))
}

async fn delete_training_registration(
    State(training_service): State<TrainingService>,
    Path((training_id, user_id)): Path<(Uuid, Uuid)>,
) -> HttpResult<impl IntoResponse> {
    // Return type changed
    training_service
        .delete_training_registration(training_id, user_id)
        .await
        .http_err("delete training registration")?;
    Ok((
        StatusCode::OK, // Use OK for successful deletion
        "Training registration deleted successfully",
    ))
}

async fn create_training(
    State(training_service): State<TrainingService>,
    Json(payload): Json<TrainingCreationPayload>, // Use new payload
) -> HttpResult<Json<Training>> {
    // Return created training
    let created_training = training_service
        .create_training(payload.training_data, payload.id_court)
        .await
        .http_err("create training")?;
    Ok(Json(created_training))
}

async fn get_training(
    State(training_service): State<TrainingService>,
    Path(id): Path<Uuid>,
) -> HttpResult<Json<Training>> {
    // Return type changed
    let training = training_service
        .get_training(id)
        .await
        .http_err("get training")?;
    Ok(Json(training))
}

async fn update_training(
    State(training_service): State<TrainingService>,
    Path(id_training): Path<Uuid>,
    Json(payload): Json<TrainingUpdatePayload>, // Use new payload for update
) -> HttpResult<Json<Training>> {
    // Return updated training
    let updated_training = training_service
        .update_training(id_training, payload.training_data, payload.id_court)
        .await
        .http_err("update training")?;
    Ok(Json(updated_training))
}

async fn delete_training(
    State(training_service): State<TrainingService>,
    Path(id): Path<Uuid>,
) -> HttpResult<impl IntoResponse> {
    // Return type changed
    training_service
        .delete_training(id)
        .await
        .http_err("delete training")?;
    Ok((StatusCode::OK, "Training deleted successfully"))
}

async fn list_trainings(
    State(training_service): State<TrainingService>,
) -> HttpResult<Json<Vec<Training>>> {
    // Return type changed
    let trainings = training_service
        .list_trainings()
        .await
        .http_err("list trainings")?;
    Ok(Json(trainings))
}

async fn register_user_for_training(
    State(training_service): State<TrainingService>,
    Path((id_training, id_user)): Path<(Uuid, Uuid)>, // id_training is now in the body
) -> HttpResult<Json<TrainingRegistration>> {
    let registration = training_service
        .register_user(id_training, id_user) // Pass the whole payload
        .await
        .http_err("register user for training")?;
    Ok(Json(registration))
}

#[derive(Deserialize)]
struct MarkAttendancePayload {
    attended: bool,
}

async fn mark_attendance(
    State(training_service): State<TrainingService>,
    Path((training_id, user_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<MarkAttendancePayload>, // Use payload for attended status
) -> HttpResult<impl IntoResponse> {
    // Return type changed
    training_service
        .mark_attendance(training_id, user_id, payload.attended)
        .await
        .http_err("mark attendance")?;
    Ok((StatusCode::OK, "Attendance marked successfully"))
}

async fn get_eligible_trainings(
    State(training_service): State<TrainingService>,
    Path(user_id): Path<Uuid>,
) -> HttpResult<Json<Vec<Training>>> {
    // Return type changed
    let trainings = training_service
        .get_eligible_trainings(user_id)
        .await
        .http_err("get eligible trainings")?;
    Ok(Json(trainings))
}

// HttpError implementation for TrainingService::Error
impl<T> HttpError<T> for Result<T, Error> {
    fn http_err(self, endpoint_name: &str) -> crate::err::HttpResult<T> {
        self.map_err(|err| {
            error!("Error in training endpoint ({}): {}", endpoint_name, err);
            let (status_code, message) = match err {
                Error::UnknownDatabaseError(e) => {
                    error!("Training DB error: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Database error processing training request.",
                    )
                }
                Error::TrainingNotFound => (StatusCode::NOT_FOUND, "Training not found."),
                Error::UserAlreadyRegistered => (
                    StatusCode::CONFLICT,
                    "User already registered for this training.",
                ),
                Error::UserDoesNotMeetCategoryRequirements => (
                    StatusCode::FORBIDDEN,
                    "User does not meet category requirements for this training.",
                ),
                Error::InvalidDates => (
                    StatusCode::BAD_REQUEST,
                    "Invalid training dates or duration.",
                ),
                Error::UserNotRegistered => (
                    StatusCode::NOT_FOUND,
                    "User not registered for this training.",
                ),
                Error::RegistrationNotFound => {
                    (StatusCode::NOT_FOUND, "Training registration not found.")
                }
                Error::CategoryServiceError(e) => {
                    error!("Category service error via training: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal error with category service.",
                    )
                }
                Error::CourtServiceError(e) => {
                    // Map court service errors to appropriate HTTP responses
                    error!("Court service error via training: {}", e);
                    match e {
                        use_cases::court_service::err::Error::CourtUnavailable => (
                            StatusCode::CONFLICT,
                            "Selected court is unavailable for the training time.",
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
                Error::UserServiceError(e) => {
                    error!("User service error via training: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal error with user service.",
                    )
                }
                Error::TuitionServiceError(e) => {
                    error!("Tuition service error via training: {}", e);
                    (
                        StatusCode::FORBIDDEN,
                        "Tuition requirement not met for training.",
                    )
                }
                Error::InvalidAssistanceDate => (
                    StatusCode::BAD_REQUEST,
                    "Invalid assistance date, the training hasn't started",
                ),
                Error::InvalidRegistrationDate => (
                    StatusCode::BAD_REQUEST,
                    "Users can only register, before the training starts",
                ),
            };
            (status_code, message.to_string()).into_response()
        })
    }
}
