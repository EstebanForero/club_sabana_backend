use async_trait::async_trait;
use chrono::NaiveDateTime;
use entities::training::Training;
use libsql::{de, params};
use use_cases::training_service::{err::Result, repository_trait::TrainingRepository};
use uuid::Uuid;

use crate::TursoDb;

#[async_trait]
impl TrainingRepository for TursoDb {
    async fn create_training(&self, training: &Training) -> Result<()> {
        let conn = self.get_connection().await?;

        conn.execute(
            "INSERT INTO 
training (id_training, name, start_datetime, end_datetime, minimum_payment, deleted, id_category)",
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
        .map_err::<Box<dyn std::error::Error>, _>(|err| Box::new(err))?;

        Ok(())
    }

    async fn get_training_by_id(&self, id: Uuid) -> Result<Option<Training>> {
        let conn = self.get_connection().await?;

        let mut rows = conn.query("SELECT id_training, name, start_datetime, end_datetime, minimum_payment, deleted, id_category
FROM training WHERE id_training = 1?", params![id.to_string()]).await.map_err::<Box<dyn std::error::Error>, _>(|err| Box::new(err))?;

        if let Some(res_rows) = rows
            .next()
            .await
            .map_err::<Box<dyn std::error::Error>, _>(|err| Box::new(err))?
        {
            return Ok(Some(de::from_row::<Training>(&res_rows).map_err::<Box<
                dyn std::error::Error,
            >, _>(
                |err| Box::new(err),
            )?));
        }

        Ok(None)
    }

    async fn update_training(&self, training: &Training) -> Result<()> {
        let conn = self.get_connection().await?;

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
        .map_err::<Box<dyn std::error::Error>, _>(|err| Box::new(err))?;

        Ok(())
    }

    async fn delete_training(&self, id: Uuid) -> Result<()> {
        let conn = self.get_connection().await?;

        conn.execute(
            "UPDATE training SET deleted = 1 WHERE id_training = 1?",
            params![id.to_string()],
        )
        .await
        .map_err::<Box<dyn std::error::Error>, _>(|err| Box::new(err))?;

        Ok(())
    }

    async fn list_trainings(&self) -> Result<Vec<Training>> {
        let conn = self.get_connection().await?;

        let mut rows = conn.query("SELECT id_training, name, start_datetime, end_datetime, minimum_payment, deleted, id_category FROM
training", params![]).await.map_err::<Box<dyn std::error::Error>, _>(|err| Box::new(err))?;

        let mut res: Vec<Training> = Vec::new();

        while let Some(res_row) = rows
            .next()
            .await
            .map_err::<Box<dyn std::error::Error>, _>(|err| Box::new(err))?
        {
            res.push(
                de::from_row::<Training>(&res_row)
                    .map_err::<Box<dyn std::error::Error>, _>(|err| Box::new(err))?,
            );
        }

        Ok(res)
    }
}
