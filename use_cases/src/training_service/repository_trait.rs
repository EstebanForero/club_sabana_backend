use super::err::Result;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use entities::training::{Training, TrainingRegistration};
use uuid::Uuid;

#[async_trait]
pub trait TrainingRepository {
    async fn create_training(&self, training: &Training) -> Result<()>;
    async fn get_training_by_id(&self, id: Uuid) -> Result<Option<Training>>;
    async fn update_training(&self, training: &Training) -> Result<()>;
    async fn delete_training(&self, id: Uuid) -> Result<()>; // Soft delete
    async fn list_trainings(&self) -> Result<Vec<Training>>;
}

#[async_trait]
pub trait TrainingRegistrationRepository {
    async fn register_user_for_training(&self, registration: &TrainingRegistration) -> Result<()>;

    async fn get_training_registrations(
        &self,
        training_id: Uuid,
    ) -> Result<Vec<TrainingRegistration>>;

    async fn mark_training_attendance(
        &self,
        training_id: Uuid,
        user_id: Uuid,
        attended: bool,
        attendance_date: NaiveDateTime,
    ) -> Result<()>;

    async fn get_user_training_registrations(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<TrainingRegistration>>;

    async fn delete_training_registration(&self, training_id: Uuid, user_id: Uuid) -> Result<()>;
}
