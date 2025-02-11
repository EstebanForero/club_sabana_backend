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

struct CategoryService<
    T: CategoryRepository + CategoryRequirementRepository + UserCategoryRepository,
> {
    db: Arc<T>,
}

impl<T> CategoryService<T>
where
    T: CategoryRepository + CategoryRequirementRepository + UserCategoryRepository,
{
    pub fn new(db: T) -> Self {
        Self { db: Arc::new(db) }
    }

    //  delete_category
    pub async fn delete_category(&self, id: Uuid) -> Result<()> {
        self.db.delete_category(id).await?;

        Ok(())
    }

    // add_category
    pub async fn add_category(&self, category: &Category) -> Result<()> {
        self.db.create_category(category).await?;

        Ok(())
    }

    // update_category
    pub async fn update_category(&self, category: &Category) -> Result<()> {
        self.db.update_category(category).await?;

        Ok(())
    }

    // get category by id
    pub async fn get_category_by_id(&self, id: Uuid) -> Result<Category> {
        let category = self.db.get_category_by_id(id).await?;

        if category.is_none() {
            return Err(Error::CategoryNotFound);
        }

        Ok(category.unwrap())
    }
    // list categories
    pub async fn get_all_categories(&self) -> Result<Vec<Category>> {
        self.db.list_categories().await
    }

    // add category requirement
    pub async fn add_category_requirement(&self, category_req: &CategoryRequirement) -> Result<()> {
        self.db.create_category_requirement(category_req).await
    }

    // get_category_requirements
    pub async fn get_category_requirements(
        &self,
        category_id: Uuid,
    ) -> Result<Vec<CategoryRequirement>> {
        self.db.get_category_requirements(category_id).await
    }

    // get_user_category
    async fn get_user_category(
        &self,
        user_id: Uuid,
        user_category: Uuid,
    ) -> Result<Option<UserCategory>> {
        self.db.get_user_category(user_id, user_category).await
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
