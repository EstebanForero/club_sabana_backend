use std::sync::Arc;

use chrono::{Datelike, NaiveDate, Utc};
use entities::user::{URol, UserCreation, UserInfo, UserLogInInfo};
use hasher_trait::PasswordHasher;
use repository_trait::UserRepository;

pub mod err;
pub mod hasher_trait;
pub mod repository_trait;
mod unique_identifier;

use err::{Error, Result};
use unique_identifier::{EmailIdentifier, Identifier, PhoneIdentifier};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
    user_repo: Arc<dyn UserRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
}

#[derive(Clone, Debug)]
pub struct LogInResponse {
    pub user_id: Uuid,
    pub user_rol: URol,
}

impl UserService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        password_hasher: Arc<dyn PasswordHasher>,
    ) -> Self {
        Self {
            user_repo,
            password_hasher,
        }
    }

    pub async fn register_user(&self, user_creation: UserCreation) -> Result<UserInfo> {
        validate_birth_date(user_creation.birth_date)?;

        let email = user_creation.email.clone();
        let phone = user_creation.phone_number.clone();
        let identification_number = user_creation.identification_number.clone();
        let identification_type = user_creation.identification_type.clone();

        if self.user_repo.get_user_id_by_email(&email).await?.is_some() {
            return Err(Error::EmailAlreadyExists);
        }
        if self.user_repo.get_user_id_by_phone(&phone).await?.is_some() {
            return Err(Error::PhoneAlreadyExists);
        }
        if self
            .user_repo
            .get_user_id_by_identification(&identification_number, &identification_type)
            .await?
            .is_some()
        {
            return Err(Error::DocumentAlreadyExists);
        }

        let hashed_password = self.password_hasher.hash(&user_creation.password)?;
        let user_id = Uuid::new_v4();
        let registration_date = Utc::now().naive_utc(); // Use UTC consistently

        let mut user = user_creation.to_user(
            user_id,
            registration_date,
            false,      // email_verified
            URol::USER, // default role
        );
        user.password = hashed_password;

        self.user_repo.create_user(&user).await?;
        Ok(UserInfo::from(user))
    }

    pub async fn update_user_role(&self, user_id: Uuid, user_rol: URol) -> Result<UserInfo> {
        let mut user = self
            .user_repo
            .get_user_by_id(user_id)
            .await?
            .ok_or(Error::UserIdDontExist)?;

        user.user_rol = user_rol;
        self.user_repo.update_user(&user).await?;
        Ok(UserInfo::from(user))
    }

    pub async fn get_all_users(&self) -> Result<Vec<UserInfo>> {
        let users = self.user_repo.list_users().await?;
        let users_info = users.into_iter().map(UserInfo::from).collect();
        Ok(users_info)
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<UserInfo> {
        self.user_repo
            .get_user_by_id(user_id)
            .await?
            .map(UserInfo::from)
            .ok_or(Error::UserIdDontExist)
    }

    pub async fn update_user(
        &self,
        user_id: Uuid,
        user_update_payload: UserCreation,
    ) -> Result<UserInfo> {
        // Changed user_id to user_id
        let mut current_user = self
            .user_repo
            .get_user_by_id(user_id) // Corrected: use user_id
            .await?
            .ok_or(Error::UserIdDontExist)?;

        validate_birth_date(user_update_payload.birth_date)?;

        if current_user.email != user_update_payload.email
            && self
                .user_repo
                .get_user_id_by_email(&user_update_payload.email)
                .await?
                .is_some()
        {
            return Err(Error::EmailAlreadyExists);
        }
        if current_user.phone_number != user_update_payload.phone_number
            && self
                .user_repo
                .get_user_id_by_phone(&user_update_payload.phone_number)
                .await?
                .is_some()
        {
            return Err(Error::PhoneAlreadyExists);
        }
        if (current_user.identification_number != user_update_payload.identification_number
            || current_user.identification_type != user_update_payload.identification_type)
            && self
                .user_repo
                .get_user_id_by_identification(
                    &user_update_payload.identification_number,
                    &user_update_payload.identification_type,
                )
                .await?
                .is_some()
        {
            return Err(Error::DocumentAlreadyExists);
        }

        current_user.first_name = user_update_payload.first_name;
        current_user.last_name = user_update_payload.last_name;
        current_user.birth_date = user_update_payload.birth_date;
        current_user.email = user_update_payload.email;
        current_user.phone_number = user_update_payload.phone_number;
        current_user.country_code = user_update_payload.country_code;
        current_user.identification_number = user_update_payload.identification_number;
        current_user.identification_type = user_update_payload.identification_type;
        if !user_update_payload.password.is_empty()
            && user_update_payload.password != current_user.password
        {
            current_user.password = self.password_hasher.hash(&user_update_payload.password)?;
        }

        self.user_repo.update_user(&current_user).await?;
        Ok(UserInfo::from(current_user))
    }

    pub async fn log_in_user(&self, user_log_in_info: &UserLogInInfo) -> Result<LogInResponse> {
        let identifier = &user_log_in_info.identifier;

        // Simplified identifier chain for example. Consider if doc_identifier should be supported for login.
        let email_identifier: Arc<dyn Identifier> =
            Arc::new(EmailIdentifier::new(self.user_repo.clone(), None));
        let phone_identifier: Arc<dyn Identifier> = Arc::new(PhoneIdentifier::new(
            self.user_repo.clone(),
            Some(email_identifier),
        ));
        // Add DocumentIdentifier if needed

        let user_id = phone_identifier.identify(identifier).await?;

        let user = self
            .user_repo
            .get_user_by_id(user_id)
            .await?
            .ok_or(Error::UserIdDontExist)?; // Should not happen if identify worked, but good practice

        let is_valid = self
            .password_hasher
            .verify(&user_log_in_info.password, &user.password)?;

        if !is_valid {
            return Err(Error::InvalidPassword);
        }

        Ok(LogInResponse {
            user_id,
            user_rol: user.user_rol,
        })
    }

    // Method for verifying email with code (placeholder for actual implementation)
    pub async fn verify_email_with_code(&self, user_id: Uuid, _code: &str) -> Result<()> {
        let mut user = self
            .user_repo
            .get_user_by_id(user_id)
            .await?
            .ok_or(Error::UserIdDontExist)?;
        // Add logic here to validate the code (e.g., from a cache or temp table)
        user.email_verified = true;
        self.user_repo.update_user(&user).await?;
        Ok(())
    }
}

fn validate_birth_date(birth_date: NaiveDate) -> Result<()> {
    let today = Utc::now().naive_utc().date();
    let age = today.year()
        - birth_date.year()
        - if today.ordinal() < birth_date.ordinal() {
            1
        } else {
            0
        };

    if age < 7 {
        return Err(Error::InvalidBirthDate(
            "User must be at least 7 years old.".to_string(),
        ));
    }
    if age > 100 {
        return Err(Error::InvalidBirthDate(
            "User cannot be older than 100 years.".to_string(),
        ));
    }
    Ok(())
}
