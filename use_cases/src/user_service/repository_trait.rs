use super::err::Result;
use async_trait::async_trait;
use entities::user::*;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository {
    async fn create_user(&self, user: &User) -> Result<()>;
    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>>;
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn update_user(&self, user: &User) -> Result<()>;
    async fn delete_user(&self, id: Uuid) -> Result<()>;
    async fn list_users(&self) -> Result<Vec<User>>;
}

pub trait UserRoleRepository {
    fn create_role(&self, role: &UserRole) -> Result<()>;
    fn get_role_by_id(&self, id: Uuid) -> Result<Option<UserRole>>;
    fn list_roles(&self) -> Result<Vec<UserRole>>;
}

pub trait IdentificationTypeRepository {
    fn create_identification_type(&self, id_type: &IdentificationType) -> Result<()>;
    fn get_identification_type_by_id(&self, id: Uuid) -> Result<Option<IdentificationType>>;
    fn list_identification_types(&self) -> Result<Vec<IdentificationType>>;
}

pub trait UserCategoryRepository {
    fn assign_category_to_user(&self, user_category: &UserCategory) -> Result<()>;
    fn get_user_categories(&self, user_id: Uuid) -> Result<Vec<UserCategory>>;
}
