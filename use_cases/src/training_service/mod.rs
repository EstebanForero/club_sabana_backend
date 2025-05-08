pub mod err;
pub mod repository_trait;

use chrono::{Duration, NaiveDateTime, Utc};
use err::{Error, Result};
use repository_trait::{TrainingRegistrationRepository, TrainingRepository};

use crate::{
    category_service::CategoryService,
    court_service::CourtService,
    tuition_service::TuitionService,
    user_service::{err::Error as UserError, UserService},
};
use entities::{
    court::CourtReservationCreation,
    training::{Training, TrainingCreation, TrainingRegistration},
    user::URol,
};
use std::sync::Arc;
use uuid::Uuid;

const MIN_EVENT_DURATION_MINUTES: i64 = 10;
const MAX_EVENT_DURATION_HOURS: i64 = 5;

#[derive(Clone)]
pub struct TrainingService {
    training_repo: Arc<dyn TrainingRepository>,
    registration_repo: Arc<dyn TrainingRegistrationRepository>,
    category_service: CategoryService,
    court_service: CourtService,
    user_service: UserService,
    tuition_service: TuitionService,
}

impl TrainingService {
    pub fn new(
        training_repo: Arc<dyn TrainingRepository>,
        registration_repo: Arc<dyn TrainingRegistrationRepository>,
        category_service: CategoryService,
        court_service: CourtService,
        user_service: UserService,
        tuition_service: TuitionService,
    ) -> Self {
        Self {
            training_repo,
            registration_repo,
            category_service,
            court_service,
            user_service,
            tuition_service,
        }
    }

    pub async fn get_training_registrations(
        &self,
        training_id: Uuid,
    ) -> Result<Vec<TrainingRegistration>> {
        self.registration_repo
            .get_training_registrations(training_id)
            .await
    }

    pub async fn get_user_training_registrations(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<TrainingRegistration>> {
        self.registration_repo
            .get_user_training_registrations(user_id)
            .await
    }

    pub async fn delete_training_registration(
        &self,
        training_id: Uuid,
        user_id: Uuid,
    ) -> Result<()> {
        // Check if training exists
        let _ = self.get_training(training_id).await?;
        // Check if registration exists
        self.registration_repo
            .get_training_registration(training_id, user_id)
            .await?
            .ok_or(Error::RegistrationNotFound)?;

        self.registration_repo
            .delete_training_registration(training_id, user_id)
            .await
    }

    pub async fn create_training(
        &self,
        training_creation: TrainingCreation,
        id_court_to_reserve: Option<Uuid>, // Added optional court ID for reservation
    ) -> Result<Training> {
        validate_event_duration(
            training_creation.start_datetime,
            training_creation.end_datetime,
        )?;

        // Validate trainer_id exists and is a TRAINER
        let trainer = self
            .user_service
            .get_user_by_id(training_creation.trainer_id)
            .await
            .map_err(|e| match e {
                UserError::UserIdDontExist => Error::UserServiceError(UserError::UserIdDontExist), // Map to specific service error
                _ => Error::UserServiceError(UserError::UnknownDatabaseError(
                    "Failed to validate trainer".to_string(),
                )),
            })?;
        if trainer.user_rol != URol::TRAINER {
            return Err(Error::UserServiceError(UserError::UnknownDatabaseError(
                // Be more specific or add a new error variant
                "Assigned user is not a trainer".to_string(),
            )));
        }

        // Validate category exists
        let _ = self
            .category_service
            .get_category_by_id(training_creation.id_category)
            .await?;

        let training_id = Uuid::new_v4();
        let training = training_creation.to_training(training_id);

        self.training_repo.create_training(&training).await?;

        if let Some(id_court) = id_court_to_reserve {
            let reservation_creation = CourtReservationCreation {
                id_court,
                start_reservation_datetime: training.start_datetime,
                end_reservation_datetime: training.end_datetime,
                id_training: Some(training.id_training),
                id_tournament: None,
            };
            if let Err(e) = self
                .court_service
                .create_reservation(reservation_creation)
                .await
            {
                self.training_repo.delete_training(training.id_training).await.unwrap_or_else(|del_err| {
                    tracing::error!("Failed to rollback training creation after court reservation failure: {}", del_err);
                });
                // Directly use the CourtServiceError variant from TrainingService::Error
                return Err(Error::CourtServiceError(e));
            }
        }
        Ok(training)
    }

    pub async fn get_training(&self, id: Uuid) -> Result<Training> {
        self.training_repo
            .get_training_by_id(id)
            .await?
            .ok_or(Error::TrainingNotFound)
    }

    pub async fn update_training(
        &self,
        training_id: Uuid,
        training_update_payload: TrainingCreation,
        id_court_to_reserve: Option<Uuid>,
    ) -> Result<Training> {
        // ... (validation logic as before) ...
        let mut training = self.get_training(training_id).await?;

        validate_event_duration(
            training_update_payload.start_datetime,
            training_update_payload.end_datetime,
        )?;

        if training.trainer_id != training_update_payload.trainer_id {
            let trainer = self
                .user_service
                .get_user_by_id(training_update_payload.trainer_id)
                .await
                .map_err(|e| match e {
                    UserError::UserIdDontExist => {
                        Error::UserServiceError(UserError::UserIdDontExist)
                    }
                    _ => Error::UserServiceError(UserError::UnknownDatabaseError(
                        "Failed to validate new trainer".to_string(),
                    )),
                })?;
            if trainer.user_rol != URol::TRAINER {
                return Err(Error::UserServiceError(UserError::UnknownDatabaseError(
                    "Assigned new user is not a trainer".to_string(),
                )));
            }
        }

        if training.id_category != training_update_payload.id_category {
            let _ = self
                .category_service
                .get_category_by_id(training_update_payload.id_category)
                .await?;
        }

        training.name = training_update_payload.name;
        training.id_category = training_update_payload.id_category;
        training.trainer_id = training_update_payload.trainer_id;
        training.start_datetime = training_update_payload.start_datetime;
        training.end_datetime = training_update_payload.end_datetime;
        training.minimum_payment = training_update_payload.minimum_payment;

        // Handle court reservation change
        // 1. Delete existing reservation for this training if any
        if let Some(existing_res) = self
            .court_service
            .get_reservation_for_training(training_id)
            .await // Now returns Option
            .map_err(|e| Error::CourtServiceError(e))?
        {
            // Check if the court or time is changing, or if no new court is specified
            let times_changed = existing_res.start_reservation_datetime != training.start_datetime
                || existing_res.end_reservation_datetime != training.end_datetime;
            let court_changed =
                id_court_to_reserve.is_some() && Some(existing_res.id_court) != id_court_to_reserve;

            if times_changed || court_changed || id_court_to_reserve.is_none() {
                self.court_service
                    .delete_reservation_for_event(training_id, "training")
                    .await
                    .map_err(|e| Error::CourtServiceError(e))?;
            }
        }

        // 2. Create new reservation if id_court_to_reserve is Some AND (it's a new court OR times changed for existing court)
        if let Some(id_court) = id_court_to_reserve {
            // Re-check if a reservation for this court/time already exists from previous step to avoid race if delete was slow
            // Or simply attempt to create; create_reservation should handle conflicts.
            let reservation_creation = CourtReservationCreation {
                id_court,
                start_reservation_datetime: training.start_datetime,
                end_reservation_datetime: training.end_datetime,
                id_training: Some(training.id_training),
                id_tournament: None,
            };
            // Only create if no existing reservation for this new configuration,
            // or if the existing one was for a different court/time and got deleted.
            // This logic can be tricky. The `create_reservation` should be idempotent or check availability.
            // For simplicity, we assume `create_reservation` handles `CourtUnavailable` correctly.
            if self
                .court_service
                .get_reservation_for_training(training_id)
                .await?
                .is_none()
            {
                // If no reservation exists now
                if let Err(e) = self
                    .court_service
                    .create_reservation(reservation_creation)
                    .await
                {
                    return Err(Error::CourtServiceError(e));
                }
            }
        }

        self.training_repo.update_training(&training).await?;
        Ok(training)
    }

    pub async fn delete_training(&self, id: Uuid) -> Result<()> {
        let _ = self.get_training(id).await?; // Ensures training exists before attempting delete

        // Soft delete associated court reservations
        // This might be better handled by ON DELETE CASCADE in DB if reservations are hard-deleted,
        // or explicitly here if soft-deleted.
        // For now, assuming CourtService handles this or it's done via DB constraints.
        if let Err(e) = self
            .court_service
            .delete_reservation_for_event(id, "training")
            .await
        {
            tracing::warn!(
                "Could not delete court reservation for training {}: {}",
                id,
                e
            );
            // Depending on requirements, you might want to fail here or just log.
        }

        self.training_repo.delete_training(id).await
    }

    pub async fn list_trainings(&self) -> Result<Vec<Training>> {
        self.training_repo.list_trainings().await
    }

    pub async fn get_trainings_by_trainer(&self, trainer_id: Uuid) -> Result<Vec<Training>> {
        // Validate trainer_id exists and is a TRAINER
        let trainer = self
            .user_service
            .get_user_by_id(trainer_id)
            .await
            .map_err(|e| match e {
                UserError::UserIdDontExist => {
                    Error::UnknownDatabaseError(format!("Trainer with ID {} not found", trainer_id))
                }
                _ => Error::UnknownDatabaseError("Failed to validate trainer".to_string()),
            })?;
        if trainer.user_rol != URol::TRAINER {
            return Err(Error::UnknownDatabaseError(
                "User is not a trainer".to_string(),
            ));
        }
        self.training_repo
            .get_trainings_by_trainer_id(trainer_id)
            .await
    }

    pub async fn register_user(
        &self,
        registration_payload: TrainingRegistration,
    ) -> Result<TrainingRegistration> {
        let training = self
            .training_repo
            .get_training_by_id(registration_payload.id_training)
            .await?
            .ok_or(Error::TrainingNotFound)?;

        if !self
            .category_service
            .user_has_category(registration_payload.id_user, training.id_category)
            .await?
        {
            return Err(Error::UserDoesNotMeetCategoryRequirements);
        }

        if self
            .registration_repo
            .get_training_registration(
                registration_payload.id_training,
                registration_payload.id_user,
            )
            .await?
            .is_some()
        {
            return Err(Error::UserAlreadyRegistered);
        }
        fn fun_name(e: crate::tuition_service::err::Error) -> Error {
            Error::TuitionServiceError(e)
        }

        if training.minimum_payment > 0.0
            && !self
                .tuition_service
                .has_active_tuition_with_amount(
                    registration_payload.id_user,
                    training.minimum_payment,
                )
                .await // Corrected
                .map_err(fun_name)?
        {
            return Err(Error::TuitionServiceError(crate::tuition_service::err::Error::UnknownDatabaseError( // Be more specific
                        format!("User has no active tuition or tuition amount is less than the minimum required: {}", training.minimum_payment) // Corrected
                    )));
        }

        let registration_to_create = TrainingRegistration {
            id_training: registration_payload.id_training,
            id_user: registration_payload.id_user,
            registration_datetime: Utc::now().naive_utc(),
            attended: false,
            attendance_datetime: None,
        };

        self.registration_repo
            .register_user_for_training(&registration_to_create)
            .await?;

        Ok(registration_to_create)
    }

    pub async fn mark_attendance(
        &self,
        training_id: Uuid,
        user_id: Uuid,
        attended: bool,
    ) -> Result<()> {
        let _ = self.get_training(training_id).await?;

        let _ = self
            .registration_repo
            .get_training_registration(training_id, user_id)
            .await?
            .ok_or(Error::UserNotRegistered)?;

        let attendance_datetime = if attended {
            Some(Utc::now().naive_utc())
        } else {
            None
        };

        self.registration_repo
            .mark_training_attendance(training_id, user_id, attended, attendance_datetime)
            .await
    }

    pub async fn get_eligible_trainings(&self, user_id: Uuid) -> Result<Vec<Training>> {
        let all_trainings = self.training_repo.list_trainings().await?;

        let user_categories = self.category_service.get_user_categories(user_id).await?;
        let user_category_ids: Vec<Uuid> = user_categories
            .into_iter()
            .map(|uc| uc.id_category)
            .collect();

        let eligible_trainings = all_trainings
            .into_iter()
            .filter(|training| user_category_ids.contains(&training.id_category))
            .collect();

        Ok(eligible_trainings)
    }
}

fn validate_event_duration(start_time: NaiveDateTime, end_time: NaiveDateTime) -> Result<()> {
    if start_time >= end_time {
        return Err(Error::InvalidDates);
    }

    let duration = end_time - start_time;
    if duration < Duration::minutes(MIN_EVENT_DURATION_MINUTES) {
        return Err(Error::InvalidDates);
    }
    if duration > Duration::hours(MAX_EVENT_DURATION_HOURS) {
        return Err(Error::InvalidDates);
    }
    Ok(())
}
