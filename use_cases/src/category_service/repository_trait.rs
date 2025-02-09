use super::err::Result;
use async_trait::async_trait;
use entities::category::{Category, CategoryRequirement, Level};
use uuid::Uuid;

/// Trait defining category-related operations
#[async_trait]
pub trait CategoryRepository {
    async fn create_category(&self, category: &Category) -> Result<()>;
    fn get_category_by_id(&self, id: Uuid) -> Result<Option<Category>>;
    fn update_category(&self, category: &Category) -> Result<()>;
    fn delete_category(&self, id: Uuid) -> Result<()>; // Soft delete
    fn list_categories(&self) -> Result<Vec<Category>>;
}

/// Trait defining level-related operations
pub trait LevelRepository {
    fn create_level(&self, level: &Level) -> Result<()>;
    fn get_level_by_id(&self, id: Uuid) -> Result<Option<Level>>;
    fn list_levels(&self) -> Result<Vec<Level>>;
}

/// Trait defining category requirements
pub trait CategoryRequirementRepository {
    fn create_category_requirement(&self, requirement: &CategoryRequirement) -> Result<()>;
    fn get_category_requirements(&self, category_id: Uuid) -> Result<Vec<CategoryRequirement>>;
}
