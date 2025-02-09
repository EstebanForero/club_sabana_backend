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

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct UserInfo {
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
    pub identification_number: String,
    pub identification_type: IdType,
    pub user_rol: URol,
}

impl From<User> for UserInfo {
    fn from(value: User) -> Self {
        UserInfo {
            id_user: value.id_user,
            first_name: value.first_name,
            last_name: value.last_name,
            birth_date: value.birth_date,
            registration_date: value.registration_date,
            email: value.email,
            email_verified: value.email_verified,
            phone_number: value.phone_number,
            country_code: value.country_code,
            identification_number: value.identification_number,
            identification_type: value.identification_type,
            user_rol: value.user_rol,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct UserCreation {
    pub first_name: String,
    pub last_name: String,
    #[serde(with = "date_serde")]
    pub birth_date: NaiveDate,
    #[serde(with = "datetime_serde")]
    pub registration_date: NaiveDateTime,
    pub email: String,
    pub phone_number: String,
    pub country_code: String,
    pub password: String,
    pub identification_number: String,
    pub identification_type: IdType,
}

pub struct UserCreationExtra {
    pub id_user: Uuid,
    pub user_rol: URol,
    pub deleted: bool,
    pub email_verified: bool,
}

impl UserCreation {
    pub fn build_user(self, user_creation_extra: UserCreationExtra) -> User {
        User {
            id_user: user_creation_extra.id_user,
            user_rol: user_creation_extra.user_rol,
            deleted: user_creation_extra.deleted,
            email_verified: user_creation_extra.email_verified,
            first_name: self.first_name,
            last_name: self.last_name,
            birth_date: self.birth_date,
            registration_date: self.registration_date,
            email: self.email,
            phone_number: self.phone_number,
            country_code: self.country_code,
            password: self.password,
            identification_number: self.identification_number,
            identification_type: self.identification_type,
        }
    }
}

pub enum IdentifierType {
    SingleValue(String),
    Identification(IdentificationInfo),
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct UserLogInInfo {
    pub identifier: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct DocInfo {
    pub identification_number: String,
    pub identification_type: IdType,
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
pub struct IdentificationInfo {
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
