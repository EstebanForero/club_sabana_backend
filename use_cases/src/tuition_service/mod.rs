pub mod err;
pub mod repository_trait;
mod tests;

use self::err::{Error, Result};
use chrono::Utc;
use entities::tuition::Tuition;
use repository_trait::TuitionRepository;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct TuitionService {
    tuition_repo: Arc<dyn TuitionRepository + Send + Sync>,
}

impl TuitionService {
    pub fn new(tuition_repo: Arc<dyn TuitionRepository + Send + Sync>) -> Self {
        Self { tuition_repo }
    }

    pub async fn pay_tuition(&self, user_id: Uuid, amount: f64) -> Result<()> {
        if amount <= 0.0 {
            return Err(Error::InvalidAmount);
        }

        if self.tuition_repo.has_active_tuition(user_id).await? {
            return Err(Error::ActiveTuitionExists);
        }

        let new_tuition = Tuition {
            id_tuition: Uuid::new_v4(),
            id_user: user_id,
            amount,
            payment_date: Utc::now().naive_utc(),
        };

        self.tuition_repo.record_tuition_payment(&new_tuition).await
    }

    pub async fn has_active_tuition(&self, user_id: Uuid) -> Result<bool> {
        self.tuition_repo.has_active_tuition(user_id).await
    }

    pub async fn get_user_tuitions(&self, user_id: Uuid) -> Result<Vec<Tuition>> {
        self.tuition_repo
            .list_tuition_payments_for_user(user_id)
            .await
    }

    pub async fn get_all_tuitions(&self) -> Result<Vec<Tuition>> {
        self.tuition_repo.list_all_tuition_payments().await
    }
}
