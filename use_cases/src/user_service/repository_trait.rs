use super::err::Result;
use entities::user::*;
use uuid::Uuid;

pub trait UserRepository {
    fn create_user(&self, user: &User) -> Result<()>;
    fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>>;
    fn get_user_by_email(&self, email: &str) -> Result<Option<User>>;
    fn update_user(&self, user: &User) -> Result<()>;
    fn delete_user(&self, id: Uuid) -> Result<()>; // Soft delete (set deleted = true)
    fn list_users(&self) -> Result<Vec<User>>;
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
