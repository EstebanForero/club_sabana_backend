use chrono::Utc;
use entities::{
    category::{Category, CategoryCreation, CategoryRequirement, Level, LevelName},
    user::UserCategory,
};
use err::{Error, Result};
use repository_trait::{CategoryRepository, CategoryRequirementRepository, UserCategoryRepository};
use std::sync::Arc;
use tracing::{debug, info};
use uuid::Uuid;

use crate::user_service::UserService;

pub mod err;
pub mod repository_trait;

#[derive(Clone)]
pub struct CategoryService {
    category_repo: Arc<dyn CategoryRepository>,
    requirement_repo: Arc<dyn CategoryRequirementRepository>,
    user_category_repo: Arc<dyn UserCategoryRepository>,
    user_service: UserService,
}

impl CategoryService {
    pub fn new(
        category_repo: Arc<dyn CategoryRepository>,
        requirement_repo: Arc<dyn CategoryRequirementRepository>,
        user_category_repo: Arc<dyn UserCategoryRepository>,
        user_service: UserService,
    ) -> Self {
        Self {
            category_repo,
            requirement_repo,
            user_category_repo,
            user_service,
        }
    }

    pub async fn delete_user_from_category(&self, user_id: Uuid, category_id: Uuid) -> Result<()> {
        self.user_category_repo
            .delete_user_category(user_id, category_id)
            .await
    }

    pub async fn update_user_category_level(
        &self,
        user_id: Uuid,
        category_id: Uuid,
        new_level: LevelName,
    ) -> Result<()> {
        self.user_category_repo
            .update_user_category(
                user_id,
                category_id,
                Level {
                    level_name: new_level,
                },
            )
            .await
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

    pub async fn add_category(&self, category_creation: CategoryCreation) -> Result<()> {
        if self
            .category_repo
            .get_category_by_name(&category_creation.name)
            .await?
            .is_some()
        {
            return Err(Error::CategoryAlreadyExists);
        }

        let category = category_creation.to_category(Uuid::new_v4());

        self.category_repo.create_category(&category).await?;

        Ok(())
    }

    pub async fn add_category_requirement(&self, category_req: &CategoryRequirement) -> Result<()> {
        self.requirement_repo
            .create_category_requirement(category_req)
            .await
    }

    pub async fn delete_category_requirement(
        &self,
        category_req_id: &Uuid,
        category_id: &Uuid,
    ) -> Result<()> {
        self.requirement_repo
            .delete_category_requirement(category_req_id, category_id)
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
        info!("user_id: {user_id}, category_id: {category_id}");
        self.user_category_repo
            .get_user_category(user_id, category_id)
            .await
    }

    pub async fn user_has_category(&self, user_id: Uuid, category_id: Uuid) -> Result<bool> {
        info!("[User has category]. user id: {user_id} | category id: {category_id}");
        self.user_category_repo
            .user_has_category(user_id, category_id)
            .await
    }

    pub async fn get_user_categories(&self, user_id: Uuid) -> Result<Vec<UserCategory>> {
        self.user_category_repo.get_user_categories(user_id).await
    }

    pub async fn add_user_to_category(&self, user_id: Uuid, category_id: Uuid) -> Result<()> {
        if self.user_has_category(user_id, category_id).await? {
            return Err(Error::UserAlreadyHasCategory);
        }

        self.perform_eligibility_checks(user_id, category_id)
            .await?;

        let user_category = UserCategory {
            id_user: user_id,
            id_category: category_id,
            user_level: LevelName::BEGGINER,
        };

        self.user_category_repo
            .create_user_category(&user_category)
            .await?;

        Ok(())
    }

    pub async fn is_user_eligible_for_category(
        &self,
        user_id: Uuid,
        category_id: Uuid,
    ) -> Result<()> {
        self.perform_eligibility_checks(user_id, category_id)
            .await
            .map(|_| ())
    }

    async fn perform_eligibility_checks(
        &self,
        user_id: Uuid,
        category_id: Uuid,
    ) -> Result<Category> {
        let category = self.get_category_by_id(category_id).await?;

        let user = self.user_service.get_user_by_id(user_id).await?;

        let current_date = Utc::now().naive_utc().date();
        let birth_date = user.birth_date;
        let user_age = current_date.years_since(birth_date).unwrap_or(0);
        let min_age = category.min_age as u32;
        let max_age = category.max_age as u32;

        if user_age < min_age || user_age > max_age {
            return Err(Error::InvalidUserAge);
        }

        let requirements = self.get_category_requirements(category_id).await?;

        for requirement in requirements {
            match self
                .get_user_category(user_id, requirement.id_category_requirement)
                .await?
            {
                Some(user_prerequisite_category) => {
                    if user_prerequisite_category.user_level < requirement.required_level {
                        return Err(Error::InvalidRequirementLevel);
                    }
                }
                None => {
                    return Err(Error::UserDoesNotMeetRequirements);
                }
            }
        }

        Ok(category)
    }
}
