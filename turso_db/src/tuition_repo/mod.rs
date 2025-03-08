use async_trait::async_trait;
use entities::tuition::Tuition;
use libsql::params;
use use_cases::tuition_service::err::{Error, Result};
use use_cases::tuition_service::repository_trait::TuitionRepository;
use uuid::Uuid;

use crate::TursoDb;

#[async_trait]
impl TuitionRepository for TursoDb {
    async fn record_tuition_payment(&self, tuition: &Tuition) -> Result<()> {
        self.execute_with_error(
            "INSERT INTO tuition (
                id_tuition, id_user, amount, payment_date, deleted
            ) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                tuition.id_tuition.to_string(),
                tuition.id_user.to_string(),
                tuition.amount,
                tuition.payment_date.to_string(),
                0
            ],
            Error::UnknownDatabaseError,
        )
        .await
    }

    async fn get_tuition_by_id(&self, id: Uuid) -> Result<Option<Tuition>> {
        self.query_one_with_error(
            "SELECT id_tuition, id_user, amount, payment_date 
             FROM tuition 
             WHERE id_tuition = ?1 AND deleted = 0",
            params![id.to_string()],
            Error::UnknownDatabaseError,
        )
        .await
    }

    async fn list_tuition_payments_for_user(&self, user_id: Uuid) -> Result<Vec<Tuition>> {
        self.query_many_with_error(
            "SELECT id_tuition, id_user, amount, payment_date 
             FROM tuition 
             WHERE id_user = ?1 AND deleted = 0",
            params![user_id.to_string()],
            Error::UnknownDatabaseError,
        )
        .await
    }

    async fn list_all_tuition_payments(&self) -> Result<Vec<Tuition>> {
        self.query_many_with_error(
            "SELECT id_tuition, id_user, amount, payment_date 
             FROM tuition 
             WHERE deleted = 0",
            params![],
            Error::UnknownDatabaseError,
        )
        .await
    }

    async fn has_active_tuition(&self, user_id: Uuid) -> Result<bool> {
        let tuitions = self.list_tuition_payments_for_user(user_id).await?;
        Ok(!tuitions.is_empty())
    }
}

#[cfg(test)]
mod test {
    use std::future::Future;

    use super::*;
    use chrono::Utc;
    use entities::tuition::Tuition;
    use rstest::{fixture, rstest};
    use uuid::Uuid;

    #[fixture]
    async fn repository() -> TursoDb {
        let db = crate::TestDbBuilder::create()
            .await
            .apply_doc_types()
            .await
            .apply_user_roles()
            .await
            .build();

        // Create test tuition table
        let conn = db.get_connection().await.expect("Failed to get connection");
        conn.execute_batch(
            "CREATE TABLE tuition (
                id_tuition TEXT PRIMARY KEY,
                id_user TEXT NOT NULL,
                amount REAL NOT NULL,
                payment_date TEXT NOT NULL,
                deleted INTEGER NOT NULL DEFAULT 0
            );",
        )
        .await
        .expect("Failed to create test tuition table");

        db
    }

    #[rstest]
    #[tokio::test]
    async fn test_record_and_get_tuition(repository: impl Future<Output = TursoDb>) {
        let db = repository.await;
        let tuition_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let tuition = Tuition {
            id_tuition: tuition_id,
            id_user: user_id,
            amount: 100.0,
            payment_date: Utc::now().naive_utc(),
        };

        db.record_tuition_payment(&tuition)
            .await
            .expect("Failed to record tuition");

        let retrieved_tuition = db
            .get_tuition_by_id(tuition_id)
            .await
            .expect("Failed to get tuition")
            .expect("Tuition not found");

        assert_eq!(tuition, retrieved_tuition);
    }

    #[rstest]
    #[tokio::test]
    async fn test_list_tuitions_for_user(repository: impl Future<Output = TursoDb>) {
        let db = repository.await;
        let user1_id = Uuid::new_v4();
        let user2_id = Uuid::new_v4();

        // Create 2 tuitions for user1
        for i in 0..2 {
            let tuition = Tuition {
                id_tuition: Uuid::new_v4(),
                id_user: user1_id,
                amount: 100.0 + i as f64,
                payment_date: Utc::now().naive_utc(),
            };

            db.record_tuition_payment(&tuition)
                .await
                .expect("Failed to record tuition");
        }

        // Create 1 tuition for user2
        let tuition = Tuition {
            id_tuition: Uuid::new_v4(),
            id_user: user2_id,
            amount: 200.0,
            payment_date: Utc::now().naive_utc(),
        };

        db.record_tuition_payment(&tuition)
            .await
            .expect("Failed to record tuition");

        // Test listing for user1
        let user1_tuitions = db
            .list_tuition_payments_for_user(user1_id)
            .await
            .expect("Failed to list tuitions");

        assert_eq!(user1_tuitions.len(), 2);

        // Test listing for user2
        let user2_tuitions = db
            .list_tuition_payments_for_user(user2_id)
            .await
            .expect("Failed to list tuitions");

        assert_eq!(user2_tuitions.len(), 1);
    }

    #[rstest]
    #[tokio::test]
    async fn test_has_active_tuition(repository: impl Future<Output = TursoDb>) {
        let db = repository.await;
        let user_id = Uuid::new_v4();

        // Initially no active tuition
        assert!(!db.has_active_tuition(user_id).await.unwrap());

        // Add a tuition
        let tuition = Tuition {
            id_tuition: Uuid::new_v4(),
            id_user: user_id,
            amount: 100.0,
            payment_date: Utc::now().naive_utc(),
        };

        db.record_tuition_payment(&tuition)
            .await
            .expect("Failed to record tuition");

        // Now has active tuition
        assert!(db.has_active_tuition(user_id).await.unwrap());
    }
}
