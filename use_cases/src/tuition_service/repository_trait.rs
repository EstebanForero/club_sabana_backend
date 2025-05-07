use super::err::Result;
use async_trait::async_trait;
use entities::tuition::Tuition;
use uuid::Uuid;

#[async_trait]
pub trait TuitionRepository: Send + Sync {
    async fn record_tuition_payment(&self, tuition: &Tuition) -> Result<()>;
    async fn get_tuition_by_id(&self, id: Uuid) -> Result<Option<Tuition>>;
    async fn list_tuition_payments_for_user(&self, user_id: Uuid) -> Result<Vec<Tuition>>;
    async fn list_all_tuition_payments(&self) -> Result<Vec<Tuition>>;
    async fn has_active_tuition(&self, user_id: Uuid) -> Result<bool>;
    async fn has_active_tuition_with_amount(
        &self,
        user_id: Uuid,
        required_amount: f64,
    ) -> Result<bool>;
}
