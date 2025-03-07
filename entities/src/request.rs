use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Request {
    pub id: Uuid,
    pub requester_id: Uuid,
    pub requested_command: String,
    pub justification: String,
    pub approved: Option<bool>,
    pub approver_id: Option<Uuid>,
}
