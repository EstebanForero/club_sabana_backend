use async_trait::async_trait;
use entities::user::User;
use use_cases::user_service::err::{Error, Result};
use use_cases::user_service::repository_trait::UserRepository;
use uuid::Uuid;

use crate::TursoDb;

#[async_trait]
impl UserRepository for TursoDb {
    async fn create_user(&self, user: &User) -> Result<()> {
        let conn = self.get_connection().await?;

        conn.execute("", params)

        Ok(())
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>> {
        todo!()
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        todo!()
    }

    async fn update_user(&self, user: &User) -> Result<()> {
        todo!()
    }

    async fn delete_user(&self, id: Uuid) -> Result<()> {
        todo!()
    }

    async fn list_users(&self) -> Result<Vec<User>> {
        todo!()
    }
}
