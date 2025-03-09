use crate::category::LevelName;

use super::date_serde;
use super::datetime_serde;
use chrono::{NaiveDate, NaiveDateTime};
use enum2str::EnumStr;
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq, Partial)]
#[partial(
    "UserInfo",
    derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq),
    omit(password)
)]
#[partial(
    "UserCreation",
    derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq),
    omit(id_user, registration_date, email_verified, user_rol)
)]
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
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone)]
pub struct UserLogInInfo {
    pub identifier: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct DocInfo {
    pub identification_number: String,
    pub identification_type: IdType,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq, EnumStr, Clone)]
pub enum IdType {
    #[default]
    CC,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq, EnumStr, Clone)]
pub enum URol {
    #[default]
    USER,
    ADMIN,
    TRAINER,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRole {
    pub user_rol: URol,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentificationInfo {
    pub identification_type: IdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCategory {
    pub id_user: Uuid,
    pub id_category: Uuid,
    pub user_level: LevelName,
}
