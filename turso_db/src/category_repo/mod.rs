use async_trait::async_trait;
use entities::category::Category;
use libsql::{de, params};
use use_cases::category_service::err::{Error, Result};
use use_cases::category_service::repository_trait::CategoryRepository;

use uuid::Uuid;

use crate::TursoDb;

#[async_trait]
impl CategoryRepository for TursoDb {
    async fn get_category_by_name(&self, name: &str) -> Result<Option<Category>> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        let mut rows = conn
            .query(
                "SELECT id_category, name, min_age, max_age, deleted FROM category WHERE name = 1? AND deleted = 0",
                params![name],
            )
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        if let Some(rows_res) = rows
            .next()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?
        {
            let category = de::from_row::<Category>(&rows_res)
                .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;
            return Ok(Some(category));
        }
        Ok(None)
    }

    async fn create_category(&self, category: &Category) -> Result<()> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        conn.execute("INSERT INTO category (id_category, name, min_age, max_age, deleted) VALUES (1?, 2?, 3?, 4?, 5?)",
            params![category.id_category.to_string(), *category.name, category.min_age, category.max_age, category.deleted]).await
                .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        Ok(())
    }

    async fn get_category_by_id(&self, id: Uuid) -> Result<Option<Category>> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        let mut rows = conn
            .query(
                "SELECT id_category, name, min_age, max_age, deleted FROM category WHERE id_category = 1? AND deleted = 0",
                params![id.to_string()],
            )
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        if let Some(rows_res) = rows
            .next()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?
        {
            let category = de::from_row::<Category>(&rows_res)
                .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

            return Ok(Some(category));
        }
        Ok(None)
    }

    async fn update_category(&self, category: &Category) -> Result<()> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        conn.execute("UPDATE category SET name = 2?, min_age = 3?, max_age = 4?, deleted = 5? WHERE id_category = 1?",
            params![category.id_category.to_string(), *category.name, category.min_age, category.max_age, category.deleted]).await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        Ok(())
    }

    async fn delete_category(&self, id: Uuid) -> Result<()> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        conn.execute(
            "UPDATE category SET deleted = 1 WHERE id_category = 1?",
            params![id.to_string()],
        )
        .await
        .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        Ok(())
    }

    async fn list_categories(&self) -> Result<Vec<Category>> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        let mut rows = conn
            .query(
                "SELECT id_category, name, min_age, max_age, deleted WHERE deleted = 0",
                params![],
            )
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        let mut res: Vec<Category> = Vec::new();

        while let Some(res_row) = rows
            .next()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?
        {
            res.push(
                de::from_row::<Category>(&res_row)
                    .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?,
            );
        }

        Ok(res)
    }
}
