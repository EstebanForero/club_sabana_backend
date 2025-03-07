pub mod err;
pub mod repository_trait;

use self::err::{Error, Result};
use entities::request::Request;
use repository_trait::RequestRepository;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct RequestService {
    request_repo: Arc<dyn RequestRepository + Send + Sync>,
}

impl RequestService {
    pub fn new(request_repo: Arc<dyn RequestRepository + Send + Sync>) -> Self {
        Self { request_repo }
    }

    pub async fn create_request(
        &self,
        requester_id: Uuid,
        requested_command: String,
        justification: String,
    ) -> Result<()> {
        let request = Request {
            id: Uuid::new_v4(),
            requester_id,
            requested_command,
            justification,
            approved: None,
            approver_id: None,
        };

        self.request_repo.create_request(&request).await
    }

    pub async fn complete_request(
        &self,
        request_id: Uuid,
        approver_id: Uuid,
        approved: bool,
    ) -> Result<()> {
        let mut request = self
            .request_repo
            .get_request_by_id(request_id)
            .await?
            .ok_or(Error::RequestNotFound)?;

        if request.requester_id == approver_id {
            return Err(Error::SelfApprovalNotAllowed);
        }

        if request.approved.is_some() {
            return Err(Error::RequestAlreadyCompleted);
        }

        request.approved = Some(approved);
        request.approver_id = Some(approver_id);

        self.request_repo.update_request(&request).await
    }

    pub async fn list_requests(&self) -> Result<Vec<Request>> {
        self.request_repo.list_requests().await
    }

    pub async fn list_user_requests(&self, user_id: Uuid) -> Result<Vec<Request>> {
        self.request_repo.list_requests_by_user(user_id).await
    }

    pub async fn get_request_by_id(&self, id: Uuid) -> Result<Option<Request>> {
        self.request_repo.get_request_by_id(id).await
    }
}
