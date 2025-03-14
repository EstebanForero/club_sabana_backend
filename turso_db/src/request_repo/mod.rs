use async_trait::async_trait;
use entities::request::Request;
use libsql::params;
use use_cases::request_service::err::{Error, Result};
use use_cases::request_service::repository_trait::RequestRepository;
use uuid::Uuid;

use crate::TursoDb;

#[async_trait]
impl RequestRepository for TursoDb {
    async fn create_request(&self, request: &Request) -> Result<()> {
        self.execute_with_error(
            "INSERT INTO request (
                request_id, requester_id, requested_command, justification, approved, approver_id
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                request.request_id.to_string(),
                request.requester_id.to_string(),
                request.requested_command.clone(),
                request.justification.clone(),
                request.approved,
                request.approver_id.map(|id| id.to_string())
            ],
            Error::UnknownDatabaseError,
        )
        .await
    }

    async fn get_request_by_id(&self, id: Uuid) -> Result<Option<Request>> {
        self.query_one_with_error(
            "SELECT request_id, requester_id, requested_command, justification, approved, approver_id 
             FROM request 
             WHERE request_id = ?1",
            params![id.to_string()],
            Error::UnknownDatabaseError,
        )
        .await
    }

    async fn update_request(&self, request: &Request) -> Result<()> {
        self.execute_with_error(
            "UPDATE request SET 
                requester_id = ?1, 
                requested_command = ?2, 
                justification = ?3, 
                approved = ?4, 
                approver_id = ?5
             WHERE request_id = ?6",
            params![
                request.requester_id.to_string(),
                request.requested_command.clone(),
                request.justification.clone(),
                request.approved,
                request.approver_id.map(|id| id.to_string()),
                request.request_id.to_string()
            ],
            Error::UnknownDatabaseError,
        )
        .await
    }

    async fn list_requests(&self) -> Result<Vec<Request>> {
        self.query_many_with_error(
            "SELECT request_id, requester_id, requested_command, justification, approved, approver_id 
             FROM request",
            params![],
            Error::UnknownDatabaseError,
        )
        .await
    }

    async fn list_requests_by_user(&self, user_id: Uuid) -> Result<Vec<Request>> {
        self.query_many_with_error(
            "SELECT request_id, requester_id, requested_command, justification, approved, approver_id 
             FROM request 
             WHERE requester_id = ?1",
            params![user_id.to_string()],
            Error::UnknownDatabaseError,
        )
        .await
    }
}

#[cfg(test)]
mod test {
    use std::future::Future;

    use super::*;
    use entities::request::Request;
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

        db
    }

    #[rstest]
    #[tokio::test]
    async fn test_create_and_get_request(repository: impl Future<Output = TursoDb>) {
        let db = repository.await;
        let request_id = Uuid::new_v4();
        let requester_id = Uuid::new_v4();

        db.create_test_user(requester_id)
            .await
            .expect("Error creating test user");

        let request = Request {
            request_id,
            requester_id,
            requested_command: "Test Command".to_string(),
            justification: "Test Justification".to_string(),
            approved: None,
            approver_id: None,
        };

        db.create_request(&request)
            .await
            .expect("Failed to create request");

        let retrieved_request = db
            .get_request_by_id(request_id)
            .await
            .expect("Failed to get request")
            .expect("Request not found");

        assert_eq!(request, retrieved_request);
    }

    #[rstest]
    #[tokio::test]
    async fn test_update_request(repository: impl Future<Output = TursoDb>) {
        let db = repository.await;
        let request_id = Uuid::new_v4();
        let requester_id = Uuid::new_v4();
        let approver_id = Uuid::new_v4();

        db.create_test_user(requester_id)
            .await
            .expect("Error creating test user");

        db.create_test_user(approver_id)
            .await
            .expect("Error creating test user");

        let mut request = Request {
            request_id,
            requester_id,
            requested_command: "Test Command".to_string(),
            justification: "Test Justification".to_string(),
            approved: None,
            approver_id: None,
        };

        db.create_request(&request)
            .await
            .expect("Failed to create request");

        // Update request
        request.approved = Some(true);
        request.approver_id = Some(approver_id);

        db.update_request(&request)
            .await
            .expect("Failed to update request");

        let updated_request = db
            .get_request_by_id(request_id)
            .await
            .expect("Failed to get request")
            .expect("Request not found");

        assert_eq!(request, updated_request);
    }

    #[rstest]
    #[tokio::test]
    async fn test_list_requests(repository: impl Future<Output = TursoDb>) {
        let db = repository.await;
        let requester_id = Uuid::new_v4();

        db.create_test_user(requester_id)
            .await
            .expect("Error creating test user");

        // Create 3 requests
        for i in 0..3 {
            let request = Request {
                request_id: Uuid::new_v4(),
                requester_id,
                requested_command: format!("Command {i}"),
                justification: format!("Justification {i}"),
                approved: None,
                approver_id: None,
            };

            db.create_request(&request)
                .await
                .expect("Failed to create request");
        }

        let requests = db.list_requests().await.expect("Failed to list requests");

        assert_eq!(requests.len(), 3);
    }

    #[rstest]
    #[tokio::test]
    async fn test_list_requests_by_user(repository: impl Future<Output = TursoDb>) {
        let db = repository.await;
        let user1_id = Uuid::new_v4();
        let user2_id = Uuid::new_v4();

        db.create_test_user(user1_id)
            .await
            .expect("Error creating test user");

        db.create_test_user(user2_id)
            .await
            .expect("Error creating test user");

        // Create 2 requests for user1
        for i in 0..2 {
            let request = Request {
                request_id: Uuid::new_v4(),
                requester_id: user1_id,
                requested_command: format!("Command {i}"),
                justification: format!("Justification {i}"),
                approved: None,
                approver_id: None,
            };

            db.create_request(&request)
                .await
                .expect("Failed to create request");
        }

        // Create 1 request for user2
        let request = Request {
            request_id: Uuid::new_v4(),
            requester_id: user2_id,
            requested_command: "Command".to_string(),
            justification: "Justification".to_string(),
            approved: None,
            approver_id: None,
        };

        db.create_request(&request)
            .await
            .expect("Failed to create request");

        // Test listing for user1
        let user1_requests = db
            .list_requests_by_user(user1_id)
            .await
            .expect("Failed to list requests");

        assert_eq!(user1_requests.len(), 2);

        // Test listing for user2
        let user2_requests = db
            .list_requests_by_user(user2_id)
            .await
            .expect("Failed to list requests");

        assert_eq!(user2_requests.len(), 1);
    }
}
