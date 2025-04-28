use async_trait::async_trait;
use entities::category::{Category, CategoryRequirement, Level};
use entities::user::UserCategory;
use libsql::{de, params};
use use_cases::category_service::err::{Error, Result};
use use_cases::category_service::repository_trait::{
    CategoryRepository, CategoryRequirementRepository, LevelRepository, UserCategoryRepository,
};

use uuid::Uuid;

use crate::TursoDb;

#[async_trait]
impl CategoryRequirementRepository for TursoDb {
    async fn create_category_requirement(&self, requirement: &CategoryRequirement) -> Result<()> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        conn.execute("INSERT INTO category_requirement (id_category_requirement, id_category, requirement_description,
required_level, deleted) VALUES (?1, ?2, ?3, ?4, 0)",
            params![requirement.id_category_requirement.to_string(), requirement.id_category.to_string(),
                requirement.requirement_description.to_string(),
            requirement.required_level.to_string()]).await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        Ok(())
    }

    async fn get_category_requirements(
        &self,
        category_id: Uuid,
    ) -> Result<Vec<CategoryRequirement>> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        let mut rows = conn
            .query(
                "SELECT id_category_requirement, id_category, requirement_description, required_level, deleted 
FROM category_requirement
WHERE deleted = 0 AND id_category = ?1",
                params![category_id.to_string()],
            )
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        let mut res: Vec<CategoryRequirement> = Vec::new();

        while let Some(res_row) = rows
            .next()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?
        {
            res.push(
                de::from_row::<CategoryRequirement>(&res_row)
                    .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?,
            );
        }

        Ok(res)
    }
}

#[async_trait]
impl UserCategoryRepository for TursoDb {
    async fn get_user_category(
        &self,
        id_user: Uuid,
        id_category: Uuid,
    ) -> Result<Option<UserCategory>> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        let mut rows = conn
            .query(
                "SELECT id_category, id_user, user_level FROM user_category WHERE id_category = ?1 AND id_user = ?2 AND deleted = 0",
                params![id_user.to_string(), id_category.to_string()],
            )
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        if let Some(rows_res) = rows
            .next()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?
        {
            let category = de::from_row::<UserCategory>(&rows_res)
                .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;
            return Ok(Some(category));
        }
        Ok(None)
    }

    async fn user_has_category(&self, id_user: Uuid, id_category: Uuid) -> Result<bool> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        let mut rows = conn
            .query(
                "SELECT 1 FROM user_category WHERE id_category = ?1 AND id_user = ?2 AND deleted = 0",
                params![id_user.to_string(), id_category.to_string()],
            )
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        Ok(rows
            .next()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?
            .is_some())
    }

    async fn create_user_category(&self, user_category: &UserCategory) -> Result<()> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        conn.execute(
            "INSERT INTO user_category (id_user, id_category, user_level, deleted) VALUES (?1, ?2, ?3, 0)",
            params![
                user_category.id_user.to_string(),
                user_category.id_category.to_string(),
                user_category.user_level.to_string()
            ],
        )
        .await
        .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        Ok(())
    }

    async fn get_user_categories(&self, user_id: Uuid) -> Result<Vec<UserCategory>> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        let mut rows = conn
            .query(
                "SELECT id_user, id_category, user_level 
                 FROM user_category 
                 WHERE id_user = ?1 AND deleted = 0",
                params![user_id.to_string()],
            )
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        let mut categories = Vec::new();

        while let Some(row) = rows
            .next()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?
        {
            let category = de::from_row::<UserCategory>(&row)
                .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;
            categories.push(category);
        }

        Ok(categories)
    }
}

#[async_trait]
impl LevelRepository for TursoDb {
    async fn create_level(&self, level: &Level) -> Result<()> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;
        conn.execute(
            "INSERT INTO level (level_name) VALUES (?1)",
            params![level.level_name.to_string()],
        )
        .await
        .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        Ok(())
    }

    async fn get_level_by_id(&self, id: Uuid) -> Result<Option<Level>> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        let mut rows = conn
            .query(
                "SELECT level_name FROM level WHERE level_name = ?1",
                params![id.to_string()],
            )
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        if let Some(rows_res) = rows
            .next()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?
        {
            let level = de::from_row::<Level>(&rows_res)
                .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;
            return Ok(Some(level));
        }
        Ok(None)
    }

    async fn list_levels(&self) -> Result<Vec<Level>> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        let mut rows = conn
            .query("SELECT id_level, name FROM level", params![])
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        let mut res: Vec<Level> = Vec::new();

        while let Some(res_row) = rows
            .next()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?
        {
            res.push(
                de::from_row::<Level>(&res_row)
                    .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?,
            );
        }

        Ok(res)
    }
}

#[async_trait]
impl CategoryRepository for TursoDb {
    async fn get_category_by_name(&self, name: &str) -> Result<Option<Category>> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        let mut rows = conn
            .query(
                "SELECT id_category, name, min_age, max_age, deleted FROM category WHERE name = ?1 AND deleted = 0",
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

        conn.execute(
            "INSERT INTO category (id_category, name, min_age, max_age) VALUES (?1, ?2, ?3, ?4)",
            params![
                category.id_category.to_string(),
                *category.name,
                category.min_age,
                category.max_age
            ],
        )
        .await
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
                "SELECT id_category, name, min_age, max_age, deleted FROM category WHERE id_category = ?1 AND deleted = 0",
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

        conn.execute(
            "UPDATE category SET name = ?2, min_age = ?3, max_age = ?4 WHERE id_category = ?1",
            params![
                category.id_category.to_string(),
                *category.name,
                category.min_age,
                category.max_age
            ],
        )
        .await
        .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        Ok(())
    }

    async fn delete_category(&self, id: Uuid) -> Result<()> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(err.to_string()))?;

        conn.execute(
            "UPDATE category SET deleted = 1 WHERE id_category = ?1",
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
                "SELECT id_category, name, min_age, max_age, deleted FROM category WHERE deleted = 0",
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
