use crate::models::{IdentificationType, User, UserCategory, UserRole};
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use uuid::Uuid;

pub trait UserRepository {
    fn create_user(&self, user: &User) -> Result<(), String>;
    fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>, String>;
    fn get_user_by_email(&self, email: &str) -> Result<Option<User>, String>;
    fn update_user(&self, user: &User) -> Result<(), String>;
    fn delete_user(&self, id: Uuid) -> Result<(), String>; // Soft delete (set deleted = true)
    fn list_users(&self) -> Result<Vec<User>, String>;
}

pub trait UserRoleRepository {
    fn create_role(&self, role: &UserRole) -> Result<(), String>;
    fn get_role_by_id(&self, id: Uuid) -> Result<Option<UserRole>, String>;
    fn list_roles(&self) -> Result<Vec<UserRole>, String>;
}

pub trait IdentificationTypeRepository {
    fn create_identification_type(&self, id_type: &IdentificationType) -> Result<(), String>;
    fn get_identification_type_by_id(&self, id: Uuid)
        -> Result<Option<IdentificationType>, String>;
    fn list_identification_types(&self) -> Result<Vec<IdentificationType>, String>;
}

pub trait UserCategoryRepository {
    fn assign_category_to_user(&self, user_category: &UserCategory) -> Result<(), String>;
    fn get_user_categories(&self, user_id: Uuid) -> Result<Vec<UserCategory>, String>;
}
