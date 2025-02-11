use async_trait::async_trait;
use entities::training::{Training, TrainingRegistration};
use libsql::{de, params};
use use_cases::training_service::{
    err::{Error, Result},
    repository_trait::{TrainingRegistrationRepository, TrainingRepository},
};
use uuid::Uuid;

use crate::TursoDb;

#[async_trait]
impl TrainingRepository for TursoDb {
    async fn create_training(&self, training: &Training) -> Result<()> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        conn.execute(
            "INSERT INTO 
training (id_training, name, start_datetime, end_datetime, minimum_payment, deleted, id_category) 
VALUES (id_training = 1?, name = 2?, start_datetime = 3?, end_datetime = 4?, minimum_payment = 5?, deleted = 6?, id_category = 7?)",
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
                training.deleted,
                training.id_category.to_string()
            ],
        )
        .await
        .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        Ok(())
    }

    async fn get_training_by_id(&self, id: Uuid) -> Result<Option<Training>> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        let mut rows = conn.query("SELECT id_training, name, start_datetime, end_datetime, minimum_payment, deleted, id_category
FROM training WHERE id_training = 1?", params![id.to_string()]).await.map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        if let Some(res_rows) = rows
            .next()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?
        {
            return Ok(Some(
                de::from_row::<Training>(&res_rows)
                    .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?,
            ));
        }
        Ok(None)
    }

    async fn update_training(&self, training: &Training) -> Result<()> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        conn.execute(
            "UPDATE training SET name = 2?, start_datetime = 3?, end_datetime = 4?,
minimum_payment = 5?, deleted = 6?, id_category = 7? WHERE id_training = 1?",
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
                training.deleted,
                training.id_category.to_string()
            ],
        )
        .await
        .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        Ok(())
    }

    async fn delete_training(&self, id: Uuid) -> Result<()> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        conn.execute(
            "UPDATE training SET deleted = 1 WHERE id_training = 1?",
            params![id.to_string()],
        )
        .await
        .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        Ok(())
    }

    async fn list_trainings(&self) -> Result<Vec<Training>> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        let mut rows = conn.query("SELECT id_training, name, start_datetime, end_datetime, minimum_payment, deleted, id_category FROM
training", params![]).await.map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        let mut res: Vec<Training> = Vec::new();

        while let Some(res_row) = rows
            .next()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?
        {
            res.push(
                de::from_row::<Training>(&res_row)
                    .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?,
            );
        }

        Ok(res)
    }
}

#[async_trait]
impl TrainingRegistrationRepository for TursoDb {
    async fn register_user_for_training(&self, registration: &TrainingRegistration) -> Result<()> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        conn.execute("INSERT INTO training_registration (id_user, registration_datetime, attended, attendance_time, id_training, deleted)
VALUES (id_user = 1?, registration_datetime = 2?, attended = 3?, attendance_time = 4?, id_training = 5?, deleted = 6?)", params![
    registration.id_user.to_string(),
    registration.registration_datetime
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
    registration.attended,
    registration.attendance_datetime
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
    registration.id_training.to_string(),
    registration.deleted
]).await.map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        Ok(())
    }

    async fn get_training_registrations(
        &self,
        training_id: Uuid,
    ) -> Result<Vec<TrainingRegistration>> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        let mut rows = conn.query(
            "SELECT id_user, registration_datetime, attended, attendance_time, id_training, deleted
FROM training_registration WHERE id_training = 1?",
            params![training_id.to_string()],
        )
        .await
        .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        let mut res: Vec<TrainingRegistration> = Vec::new();

        while let Some(res_rows) = rows
            .next()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?
        {
            res.push(
                de::from_row::<TrainingRegistration>(&res_rows)
                    .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?,
            );
        }

        Ok(res)
    }

    async fn mark_training_attendance(
        &self,
        training_id: Uuid,
        user_id: Uuid,
        attended: bool,
    ) -> Result<()> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        conn.execute(
            "UPDATE training_registration SET attended = 1? WHERE training_id: 2?, user_id: 3?",
            params![attended, training_id.to_string(), user_id.to_string()],
        )
        .await
        .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        Ok(())
    }
}
