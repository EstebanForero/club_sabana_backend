use super::date_serde;
use super::datetime_serde;
use chrono::{NaiveDate, NaiveDateTime};
use enum2str::EnumStr;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
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
    pub user_rol: URol,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq, EnumStr)]
pub enum IdType {
    #[default]
    CC,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq, EnumStr)]
pub enum URol {
    #[default]
    USER,
    ADMIN,
    TRAINER,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRole {
    pub user_rol: URol,
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
