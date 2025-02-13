use async_trait::async_trait;
use entities::training::{Training, TrainingRegistration};
use libsql::params;
use use_cases::training_service::{
    err::{Error, Result},
    repository_trait::{TrainingRegistrationRepository, TrainingRepository},
};
use uuid::Uuid;

use crate::TursoDb;

#[async_trait]
impl TrainingRepository for TursoDb {
    async fn create_training(&self, training: &Training) -> Result<()> {
        self.execute_with_error("INSERT INTO 
training (id_training, name, start_datetime, end_datetime, minimum_payment, id_category) 
VALUES (id_training = 1?, name = 2?, start_datetime = 3?, end_datetime = 4?, minimum_payment = 5?, id_category = 6?)",
            params![
                training.id_training.to_string(),
                *training.name,
                training
                    .start_datetime
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
                training
                    .end_datetime
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
                training.minimum_payment,
                training.id_category.to_string()
            ],
 Error::UnknownDatabaseError).await
    }

    async fn get_training_by_id(&self, id: Uuid) -> Result<Option<Training>> {
        self.query_one_with_error(
            "SELECT id_training, name, start_datetime, end_datetime, minimum_payment, id_category
FROM training WHERE id_training = 1?",
            params![id.to_string()],
            Error::UnknownDatabaseError,
        )
        .await
    }

    async fn update_training(&self, training: &Training) -> Result<()> {
        self.execute_with_error(
            "UPDATE training SET name = 2?, start_datetime = 3?, end_datetime = 4?,
minimum_payment = 5? = 6?, id_category = 7? WHERE id_training = 1?",
            params![
                training.id_training.to_string(),
                *training.name,
                training
                    .start_datetime
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
                training
                    .end_datetime
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
                training.minimum_payment,
                training.id_category.to_string()
            ],
            Error::UnknownDatabaseError,
        )
        .await
    }

    async fn delete_training(&self, id: Uuid) -> Result<()> {
        self.execute_with_error(
            "UPDATE training SET deleted = 1 WHERE id_training = 1?",
            params![id.to_string()],
            Error::UnknownDatabaseError,
        )
        .await
    }

    async fn list_trainings(&self) -> Result<Vec<Training>> {
        self.query_many_with_error("SELECT id_training, name, start_datetime, end_datetime, minimum_payment, id_category FROM
training", params![], Error::UnknownDatabaseError).await
    }
}

#[async_trait]
impl TrainingRegistrationRepository for TursoDb {
    async fn register_user_for_training(&self, registration: &TrainingRegistration) -> Result<()> {
        self.execute_with_error("INSERT INTO training_registration (id_user, registration_datetime, attended, attendance_time, id_training)
VALUES (id_user = 1?, registration_datetime = 2?, attended = 3?, attendance_time = 4?, id_training = 5? = 6?)", params![
    registration.id_user.to_string(),
    registration.registration_datetime
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
    registration.attended,
    registration.attendance_datetime
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
    registration.id_training.to_string(),
], Error::UnknownDatabaseError).await
    }

    async fn get_training_registrations(
        &self,
        training_id: Uuid,
    ) -> Result<Vec<TrainingRegistration>> {
        self.query_many_with_error(
            "SELECT id_user, registration_datetime, attended, attendance_time, id_training
FROM training_registration WHERE id_training = 1?",
            params![training_id.to_string()],
            Error::UnknownDatabaseError,
        )
        .await
    }

    async fn mark_training_attendance(
        &self,
        training_id: Uuid,
        user_id: Uuid,
        attended: bool,
    ) -> Result<()> {
        self.execute_with_error(
            "UPDATE training_registration SET attended = 1? WHERE training_id: 2?, user_id: 3?",
            params![attended, training_id.to_string(), user_id.to_string()],
            Error::UnknownDatabaseError,
        )
        .await
    }
}
