use super::err::Result;
use async_trait::async_trait;
use entities::{
    category::{Category, CategoryRequirement, Level, LevelName},
    user::UserCategory,
};
use uuid::Uuid;

#[async_trait]
pub trait CategoryRepository: Send + Sync {
    async fn create_category(&self, category: &Category) -> Result<()>;
    async fn get_category_by_id(&self, id: Uuid) -> Result<Option<Category>>;
    async fn update_category(&self, category: &Category) -> Result<()>;
    async fn delete_category(&self, id: Uuid) -> Result<()>; // Soft delete
    async fn list_categories(&self) -> Result<Vec<Category>>;
    async fn get_category_by_name(&self, name: &str) -> Result<Option<Category>>;
}

#[async_trait]
pub trait LevelRepository: Send + Sync {
    async fn create_level(&self, level: &Level) -> Result<()>;
    async fn get_level_by_id(&self, id: Uuid) -> Result<Option<Level>>;
    async fn list_levels(&self) -> Result<Vec<Level>>;
}

#[async_trait]
pub trait CategoryRequirementRepository: Send + Sync {
    async fn create_category_requirement(&self, requirement: &CategoryRequirement) -> Result<()>;
    async fn delete_category_requirement(
        &self,
        category_req_id: &Uuid,
        category_id: &Uuid,
    ) -> Result<()>;
    async fn get_category_requirements(
        &self,
        category_id: Uuid,
    ) -> Result<Vec<CategoryRequirement>>;
}

#[async_trait]
pub trait UserCategoryRepository: Send + Sync {
    async fn get_user_category(
        &self,
        id_user: Uuid,
        id_category: Uuid,
    ) -> Result<Option<UserCategory>>;

    async fn user_has_category(&self, id_user: Uuid, id_category: Uuid) -> Result<bool>;

    async fn create_user_category(&self, user_category: &UserCategory) -> Result<()>;

    async fn get_user_categories(&self, user_id: Uuid) -> Result<Vec<UserCategory>>;

    async fn update_user_category(
        &self,
        user_id: Uuid,
        id_category: Uuid,
        new_level: Level,
    ) -> Result<()>;

    async fn delete_user_category(&self, user_id: Uuid, id_category: Uuid) -> Result<()>;
}
