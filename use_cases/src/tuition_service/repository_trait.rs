use super::err::Result;
use entities::tuition::Tuition;
use uuid::Uuid;

pub trait TuitionRepository {
    fn record_tuition_payment(&self, tuition: &Tuition) -> Result<()>;
    fn get_tuition_by_id(&self, id: Uuid) -> Result<Option<Tuition>>;
    fn list_tuition_payments_for_user(&self, user_id: Uuid) -> Result<Vec<Tuition>>;
    fn list_all_tuition_payments(&self) -> Result<Vec<Tuition>>;
}
