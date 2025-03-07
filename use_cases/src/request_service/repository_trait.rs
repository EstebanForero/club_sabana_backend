use super::err::Result;
use async_trait::async_trait;
use entities::request::Request;
use uuid::Uuid;

#[async_trait]
pub trait RequestRepository: Send + Sync {
    async fn create_request(&self, request: &Request) -> Result<()>;
    async fn get_request_by_id(&self, id: Uuid) -> Result<Option<Request>>;
    async fn update_request(&self, request: &Request) -> Result<()>;
    async fn list_requests(&self) -> Result<Vec<Request>>;
    async fn list_requests_by_user(&self, user_id: Uuid) -> Result<Vec<Request>>;
}
