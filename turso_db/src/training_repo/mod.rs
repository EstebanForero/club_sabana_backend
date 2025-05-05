use async_trait::async_trait;
use chrono::NaiveDateTime;
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
        self.execute_with_error(
            "INSERT INTO 
training (id_training, name, start_datetime, end_datetime, minimum_payment, id_category) 
VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
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

    async fn get_training_by_id(&self, id: Uuid) -> Result<Option<Training>> {
        self.query_one_with_error(
            "SELECT id_training, name, start_datetime, end_datetime, minimum_payment, id_category
FROM training WHERE id_training = ?1",
            params![id.to_string()],
            Error::UnknownDatabaseError,
        )
        .await
    }

    async fn update_training(&self, training: &Training) -> Result<()> {
        self.execute_with_error(
            "UPDATE training SET name = ?1, start_datetime = ?2, end_datetime = ?3,
minimum_payment = ?4, id_category = ?5 WHERE id_training = ?6",
            params![
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
                training.id_category.to_string(),
                training.id_training.to_string(),
            ],
            Error::UnknownDatabaseError,
        )
        .await
    }

    async fn delete_training(&self, id: Uuid) -> Result<()> {
        self.execute_with_error(
            "UPDATE training SET deleted = 1 WHERE id_training = ?1",
            params![id.to_string()],
            Error::UnknownDatabaseError,
        )
        .await
    }

    async fn list_trainings(&self) -> Result<Vec<Training>> {
        self.query_many_with_error("SELECT id_training, name, start_datetime, end_datetime, minimum_payment, id_category FROM
training WHERE deleted = 0", params![], Error::UnknownDatabaseError).await
    }
}

#[async_trait]
impl TrainingRegistrationRepository for TursoDb {
    async fn get_user_training_registrations(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<TrainingRegistration>> {
        self.query_many_with_error(
            "SELECT id_user, registration_datetime, attended, attendance_datetime, id_training
             FROM training_registration WHERE id_user = ?1",
            params![user_id.to_string()],
            Error::UnknownDatabaseError,
        )
        .await
    }

    async fn delete_training_registration(&self, training_id: Uuid, user_id: Uuid) -> Result<()> {
        self.execute_with_error(
            "DELETE FROM training_registration WHERE id_training = ?1 AND id_user = ?2",
            params![training_id.to_string(), user_id.to_string()],
            Error::UnknownDatabaseError,
        )
        .await
    }

    async fn register_user_for_training(&self, registration: &TrainingRegistration) -> Result<()> {
        self.execute_with_error(
        "INSERT INTO training_registration (id_user, registration_datetime, attended, attendance_datetime, id_training)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            registration.id_user.to_string(),
            registration.registration_datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
            registration.attended,
            registration.attendance_datetime.map(|date_time| date_time.format("%Y-%m-%d %H:%M:%S").to_string()),
            registration.id_training.to_string(),
        ],
        Error::UnknownDatabaseError,
    ).await
    }

    async fn get_training_registrations(
        &self,
        training_id: Uuid,
    ) -> Result<Vec<TrainingRegistration>> {
        self.query_many_with_error(
            "SELECT id_user, registration_datetime, attended, attendance_datetime, id_training
FROM training_registration WHERE id_training = ?1",
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
        attendance_date: NaiveDateTime,
    ) -> Result<()> {
        self.execute_with_error(
        "UPDATE training_registration SET attended = ?1, attendance_datetime = ?4 WHERE id_training = ?2 AND id_user = ?3",
        params![attended, training_id.to_string(), user_id.to_string(), attendance_date.format("%Y-%m-%d %H:%M:%S")
                    .to_string()],
        Error::UnknownDatabaseError,
    )
    .await
    }
}
