use async_trait::async_trait;
use entities::category::{Category, CategoryRequirement};
use entities::user::UserCategory;
use libsql::{de, params};
use use_cases::category_service::err::Result;
use use_cases::category_service::repository_trait::{
    CategoryRepository, CategoryRequirementRepository, UserCategoryRepository,
};
use uuid::Uuid;

use crate::TursoDb;

#[async_trait]
impl CategoryRepository for TursoDb {
    async fn create_category(&self, category: &Category) -> Result<()> {
        let conn = self.get_connection().await?;

        conn.execute("INSERT INTO category (id_category, name, min_age, max_age, deleted) VALUES (1?, 2?, 3?, 4?, 5?)",
            params![category.id_category.to_string(), *category.name, category.min_age, category.max_age, category.deleted]).await
                .map_err::<Box<dyn std::error::Error>, _>(|err| Box::new(err))?;

        Ok(())
    }

    async fn get_category_by_id(&self, id: Uuid) -> Result<Option<Category>> {
        let conn = self.get_connection().await?;

        let mut rows = conn
            .query(
                "SELECT id_category, name, min_age, max_age, deleted FROM category WHERE id_category = 1? AND deleted = 0",
                params![id.to_string()],
            )
            .await
            .map_err::<Box<dyn std::error::Error>, _>(|err| Box::new(err))?;

        if let Some(rows_res) = rows
            .next()
            .await
            .map_err::<Box<dyn std::error::Error>, _>(|err| Box::new(err))?
        {
            let category = de::from_row::<Category>(&rows_res)
                .map_err::<Box<dyn std::error::Error>, _>(|err| Box::new(err))?;

            return Ok(Some(category));
        }
        Ok(None)
    }

    async fn update_category(&self, category: &Category) -> Result<()> {
        let conn = self.get_connection().await?;

        conn.execute("UPDATE category SET name = 2?, min_age = 3?, max_age = 4?, deleted = 5? WHERE id_category = 1?",
            params![category.id_category.to_string(), *category.name, category.min_age, category.max_age, category.deleted]).await
            .map_err::<Box<dyn std::error::Error>, _>(|err| Box::new(err))?;

        Ok(())
    }

    async fn delete_category(&self, id: Uuid) -> Result<()> {
        let conn = self.get_connection().await?;

        conn.execute(
            "UPDATE category SET deleted = 1 WHERE id_category = 1?",
            params![id.to_string()],
        )
        .await
        .map_err::<Box<dyn std::error::Error>, _>(|err| Box::new(err))?;

        Ok(())
    }

    async fn list_categories(&self) -> Result<Vec<Category>> {
        let conn = self.get_connection().await?;

        let mut rows = conn
            .query(
                "SELECT id_category, name, min_age, max_age, deleted WHERE deleted = 0",
                params![],
            )
            .await
            .map_err::<Box<dyn std::error::Error>, _>(|err| Box::new(err))?;

        let mut res: Vec<Category> = Vec::new();

        while let Some(res_row) = rows
            .next()
            .await
            .map_err::<Box<dyn std::error::Error>, _>(|err| Box::new(err))?
        {
            res.push(
                de::from_row::<Category>(&res_row)
                    .map_err::<Box<dyn std::error::Error>, _>(|err| Box::new(err))?,
            );
        }

        Ok(res)
    }
}

#[async_trait]
impl CategoryRequirementRepository for TursoDb {
    async fn create_category_requirement(&self, requirement: &CategoryRequirement) -> Result<()> {
        let conn = self.get_connection().await?;

        conn.execute("INSERT INTO category_requirement 
(id_category_requirement, id_category, requirement_description, required_level, deleted) VALUES(1?, 2?, 3?, 4?, 5?)",
            params![requirement.id_category_requirement.to_string(), requirement.id_category.to_string(),
            *requirement.requirement_description, requirement.required_level.to_string(), requirement.deleted]).await.
            map_err::<Box<dyn std::error::Error>, _>(|err| Box::new(err))?;

        Ok(())
    }

    async fn get_category_requirements(
        &self,
        category_id: Uuid,
    ) -> Result<Vec<CategoryRequirement>> {
        let conn = self.get_connection().await?;

        let mut rows = conn.query("SELECT id_category_requirement, id_category, requirement_description, required_level, deleted-
FROM category_requirement WHERE id_category = 1?", params![category_id.to_string()]).await.
            map_err::<Box<dyn std::error::Error>, _>(|err| Box::new(err))?;

        let mut res: Vec<CategoryRequirement> = Vec::new();

        while let Some(rows_res) = rows
            .next()
            .await
            .map_err::<Box<dyn std::error::Error>, _>(|err| Box::new(err))?
        {
            res.push(
                de::from_row::<CategoryRequirement>(&rows_res)
                    .map_err::<Box<dyn std::error::Error>, _>(|err| Box::new(err))?,
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
        let conn = self.get_connection().await?;

        let mut rows = conn
            .query(
                "SELECT id_user, id_category, deleted, user_level FROM user_category
WHERE id_user = 1?, id_category = 2?",
                params![id_user.to_string(), id_category.to_string()],
            )
            .await
            .map_err::<Box<dyn std::error::Error>, _>(|err| Box::new(err))?;

        if let Some(res_rows) = rows
            .next()
            .await
            .map_err::<Box<dyn std::error::Error>, _>(|err| Box::new(err))?
        {
            return Ok(Some(
                de::from_row::<UserCategory>(&res_rows)
                    .map_err::<Box<dyn std::error::Error>, _>(|err| Box::new(err))?,
            ));
        }

        Ok(None)
    }
}
