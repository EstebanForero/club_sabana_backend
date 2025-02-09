use super::date_serde;
use super::datetime_serde;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id_user: Uuid,
    pub first_name: String,
    pub last_name: String,
    #[serde(with = "date_serde")]
    pub birth_date: NaiveDate,
    #[serde(with = "datetime_serde")]
    pub registration_date: NaiveDateTime,
    pub email: String,
    pub email_verified: bool,
    pub phone_number: String,
    pub country_code: String,
    pub password: String,
    pub identification_number: String,
    pub identification_type: IdType,
    pub user_rol: Uuid,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IdType {
    CC,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRole {
    pub user_rol: Uuid,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentificationType {
    pub identification_type: IdType,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCategory {
    pub id_user: Uuid,
    pub id_category: Uuid,
    pub user_level: String,
    pub deleted: bool,
}
