pub mod err;
pub mod repository_trait;

use err::{Error, Result};
use repository_trait::{TrainingRegistrationRepository, TrainingRepository};

use crate::category_service::CategoryService;
use entities::training::{Training, TrainingCreation, TrainingRegistration};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct TrainingService {
    training_repo: Arc<dyn TrainingRepository + Send + Sync>,
    registration_repo: Arc<dyn TrainingRegistrationRepository + Send + Sync>,
    category_service: CategoryService,
}

impl TrainingService {
    pub fn new(
        training_repo: Arc<dyn TrainingRepository + Send + Sync>,
        registration_repo: Arc<dyn TrainingRegistrationRepository + Send + Sync>,
        category_service: CategoryService,
    ) -> Self {
        Self {
            training_repo,
            registration_repo,
            category_service,
        }
    }

    pub async fn create_training(&self, training_creation: &TrainingCreation) -> Result<()> {
        if training_creation.start_datetime >= training_creation.end_datetime {
            return Err(Error::InvalidDates);
        }

        let training = training_creation.to_training_cloned(Uuid::new_v4());

        self.training_repo.create_training(&training).await
    }

    pub async fn get_training(&self, id: Uuid) -> Result<Training> {
        self.training_repo
            .get_training_by_id(id)
            .await?
            .ok_or(Error::TrainingNotFound)
    }

    pub async fn update_training(&self, training: &Training) -> Result<()> {
        if training.start_datetime >= training.end_datetime {
            return Err(Error::InvalidDates);
        }

        if self
            .training_repo
            .get_training_by_id(training.id_training)
            .await?
            .is_none()
        {
            return Err(Error::TrainingNotFound);
        }

        self.training_repo.update_training(training).await
    }

    pub async fn delete_training(&self, id: Uuid) -> Result<()> {
        if self.training_repo.get_training_by_id(id).await?.is_none() {
            return Err(Error::TrainingNotFound);
        }

        self.training_repo.delete_training(id).await
    }

    pub async fn list_trainings(&self) -> Result<Vec<Training>> {
        self.training_repo.list_trainings().await
    }

    pub async fn register_user(&self, registration: TrainingRegistration) -> Result<()> {
        let training = self
            .training_repo
            .get_training_by_id(registration.id_training)
            .await?
            .ok_or(Error::TrainingNotFound)?;

        // Check if user has the required category
        if !self
            .category_service
            .user_has_category(registration.id_user, training.id_category)
            .await?
        {
            return Err(Error::UserDoesNotMeetCategoryRequirements);
        }

        // Check if user is already registered
        let registrations = self
            .registration_repo
            .get_training_registrations(registration.id_training)
            .await?;
        if registrations
            .iter()
            .any(|r| r.id_user == registration.id_user)
        {
            return Err(Error::UserAlreadyRegistered);
        }

        self.registration_repo
            .register_user_for_training(&registration)
            .await
    }

    pub async fn mark_attendance(
        &self,
        training_id: Uuid,
        user_id: Uuid,
        attended: bool,
    ) -> Result<()> {
        // Check if training exists
        if self
            .training_repo
            .get_training_by_id(training_id)
            .await?
            .is_none()
        {
            return Err(Error::TrainingNotFound);
        }

        // Check if user is registered
        let registrations = self
            .registration_repo
            .get_training_registrations(training_id)
            .await?;
        if !registrations.iter().any(|r| r.id_user == user_id) {
            return Err(Error::UserNotRegistered);
        }

        self.registration_repo
            .mark_training_attendance(training_id, user_id, attended)
            .await
    }

    pub async fn get_eligible_trainings(&self, user_id: Uuid) -> Result<Vec<Training>> {
        let all_trainings = self.training_repo.list_trainings().await?;

        let user_categories = self.category_service.get_user_categories(user_id).await?;
        let user_category_ids: Vec<Uuid> = user_categories
            .into_iter()
            .map(|uc| uc.id_category)
            .collect();

        let eligible_trainings = all_trainings
            .into_iter()
            .filter(|training| user_category_ids.contains(&training.id_category))
            .collect();

        Ok(eligible_trainings)
    }
}
