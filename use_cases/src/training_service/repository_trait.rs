use super::err::Result;
use entities::training::{Training, TrainingRegistration};
use uuid::Uuid;

pub trait TrainingRepository {
    fn create_training(&self, training: &Training) -> Result<()>;
    fn get_training_by_id(&self, id: Uuid) -> Result<Option<Training>>;
    fn update_training(&self, training: &Training) -> Result<()>;
    fn delete_training(&self, id: Uuid) -> Result<()>; // Soft delete
    fn list_trainings(&self) -> Result<Vec<Training>>;
}

pub trait TrainingRegistrationRepository {
    fn register_user_for_training(&self, registration: &TrainingRegistration) -> Result<()>;

    fn get_training_registrations(&self, training_id: Uuid) -> Result<Vec<TrainingRegistration>>;

    fn mark_training_attendance(
        &self,
        training_id: Uuid,
        user_id: Uuid,
        attended: bool,
    ) -> Result<()>;
}
