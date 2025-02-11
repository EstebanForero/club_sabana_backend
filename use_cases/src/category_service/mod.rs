use entities::{
    category::{Category, CategoryRequirement},
    user::UserCategory,
};
use err::{Error, Result};
use repository_trait::{CategoryRepository, CategoryRequirementRepository, UserCategoryRepository};
use std::sync::Arc;
use uuid::Uuid;

pub mod err;
pub mod repository_trait;

#[derive(Clone)]
pub struct CategoryService {
    category_repo: Arc<dyn CategoryRepository + Send + Sync>,
    requirement_repo: Arc<dyn CategoryRequirementRepository + Send + Sync>,
    user_category_repo: Arc<dyn UserCategoryRepository + Send + Sync>,
}

impl CategoryService {
    pub fn new(
        category_repo: Arc<dyn CategoryRepository + Send + Sync>,
        requirement_repo: Arc<dyn CategoryRequirementRepository + Send + Sync>,
        user_category_repo: Arc<dyn UserCategoryRepository + Send + Sync>,
    ) -> Self {
        Self {
            category_repo,
            requirement_repo,
            user_category_repo,
        }
    }

    //  delete_category
    pub async fn delete_category(&self, id: Uuid) -> Result<()> {
        self.category_repo.delete_category(id).await?;
        Ok(())
    }

    pub async fn update_category(&self, category: &Category) -> Result<()> {
        // Validate category exists
        if self
            .category_repo
            .get_category_by_id(category.id_category)
            .await?
            .is_none()
        {
            return Err(Error::CategoryNotFound);
        }

        // Validate category name
        if category.name.trim().is_empty() {
            return Err(Error::MissingName);
        }

        // Validate age range
        if category.min_age >= category.max_age {
            return Err(Error::InvalidAgeRange);
        }

        self.category_repo.update_category(category).await?;
        Ok(())
    }

    pub async fn get_category_by_id(&self, id: Uuid) -> Result<Category> {
        self.category_repo
            .get_category_by_id(id)
            .await?
            .ok_or(Error::CategoryNotFound)
    }

    pub async fn get_all_categories(&self) -> Result<Vec<Category>> {
        self.category_repo.list_categories().await
    }

    pub async fn add_category_requirement(&self, category_req: &CategoryRequirement) -> Result<()> {
        self.requirement_repo
            .create_category_requirement(category_req)
            .await
    }

    pub async fn get_category_requirements(
        &self,
        category_id: Uuid,
    ) -> Result<Vec<CategoryRequirement>> {
        self.requirement_repo
            .get_category_requirements(category_id)
            .await
    }

    pub async fn get_user_category(
        &self,
        user_id: Uuid,
        category_id: Uuid,
    ) -> Result<Option<UserCategory>> {
        self.user_category_repo
            .get_user_category(user_id, category_id)
            .await
    }

    // get user categories it is elegible to
    //pub async fn get_elegible_categories(
    //    &self,
    //    category_id: Uuid,
    //    user_id: _,
    //) -> Result<Vec<Category>> {
    //    todo!()
    //}
}
