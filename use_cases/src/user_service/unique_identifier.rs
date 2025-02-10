use std::sync::Arc;

use super::err::Error;
use super::{err::Result, repository_trait::UserRepository};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Identifier: Sync + Send {
    async fn identify(&self, identifier: &str) -> Result<Uuid>;

    async fn next(&mut self, next: Arc<dyn Identifier>);
}

pub struct EmailIdentifier {
    repo: Arc<dyn UserRepository>,
    next: Option<Arc<dyn Identifier>>,
}

impl EmailIdentifier {
    pub fn new(repo: Arc<dyn UserRepository>, next: Option<Arc<dyn Identifier>>) -> Self {
        Self { repo, next }
    }
}

#[async_trait]
impl Identifier for EmailIdentifier {
    async fn identify(&self, identifier: &str) -> Result<Uuid> {
        if let Some(user_id) = self.repo.get_user_id_by_email(identifier).await? {
            Ok(user_id)
        } else {
            if let Some(next) = self.next.clone() {
                return next.identify(identifier).await;
            }

            Err(Error::InvalidIdentifier)
        }
    }

    async fn next(&mut self, next: Arc<dyn Identifier>) {
        self.next = Some(next);
    }
}

pub struct PhoneIdentifier {
    repo: Arc<dyn UserRepository>,
    next: Option<Arc<dyn Identifier>>,
}

impl PhoneIdentifier {
    pub fn new(repo: Arc<dyn UserRepository>, next: Option<Arc<dyn Identifier>>) -> Self {
        Self { repo, next }
    }
}

#[async_trait]
impl Identifier for PhoneIdentifier {
    async fn identify(&self, identifier: &str) -> Result<Uuid> {
        if let Some(user_id) = self.repo.get_user_id_by_phone(identifier).await? {
            Ok(user_id)
        } else {
            if let Some(next) = self.next.clone() {
                return next.identify(identifier).await;
            }

            Err(Error::InvalidIdentifier)
        }
    }

    async fn next(&mut self, next: Arc<dyn Identifier>) {
        self.next = Some(next);
    }
}
