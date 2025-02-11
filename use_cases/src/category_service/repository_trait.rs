use super::err::Result;
use async_trait::async_trait;
use entities::{
    category::{Category, CategoryRequirement, Level},
    user::UserCategory,
};
use uuid::Uuid;

/// Trait defining category-related operations
#[async_trait]
pub trait CategoryRepository: Send + Sync {
    async fn create_category(&self, category: &Category) -> Result<()>;
    async fn get_category_by_id(&self, id: Uuid) -> Result<Option<Category>>;
    async fn update_category(&self, category: &Category) -> Result<()>;
    async fn delete_category(&self, id: Uuid) -> Result<()>; // Soft delete
    async fn list_categories(&self) -> Result<Vec<Category>>;
    async fn get_category_by_name(&self, name: &str) -> Result<Option<Category>>;
}

/// Trait defining level-related operations
pub trait LevelRepository: Send + Sync {
    fn create_level(&self, level: &Level) -> Result<()>;
    fn get_level_by_id(&self, id: Uuid) -> Result<Option<Level>>;
    fn list_levels(&self) -> Result<Vec<Level>>;
}

/// Trait defining category requirements
#[async_trait]
pub trait CategoryRequirementRepository: Send + Sync {
    async fn create_category_requirement(&self, requirement: &CategoryRequirement) -> Result<()>;
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
}
