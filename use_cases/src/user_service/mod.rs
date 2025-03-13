use std::sync::Arc;

use chrono::Utc;
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

    pub async fn register_user(&self, user_creation: UserCreation) -> Result<()> {
        let email = user_creation.email.clone();
        let phone = user_creation.phone_number.clone();
        let identification_number = user_creation.identification_number.clone();
        let identification_type = user_creation.identification_type.clone();

        let mut user =
            user_creation.to_user(Uuid::new_v4(), Utc::now().naive_utc(), false, URol::USER);

        if self.user_repo.get_user_id_by_email(&email).await?.is_some() {
            return Err(Error::EmailAlreadyExists);
        } else if self.user_repo.get_user_id_by_phone(&phone).await?.is_some() {
            return Err(Error::PhoneAlreadyExists);
        } else if self
            .user_repo
            .get_user_id_by_identification(&identification_number, &identification_type)
            .await?
            .is_some()
        {
            return Err(Error::DocumentAlreadyExists);
        }

        user.password = self.password_hasher.hash(&user.password)?;

        self.user_repo.create_user(&user).await?;

        Ok(())
    }

    pub async fn get_all_users(&self) -> Result<Vec<UserInfo>> {
        let users = self.user_repo.list_users().await?;

        let users_info = users.into_iter().map(UserInfo::from).collect();

        Ok(users_info)
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<UserInfo> {
        let user = self.user_repo.get_user_by_id(user_id).await?;

        match user {
            Some(user) => Ok(UserInfo::from(user)),
            None => Err(Error::UserIdDontExist),
        }
    }

    pub async fn log_in_user(&self, user_log_in_info: &UserLogInInfo) -> Result<LogInResponse> {
        let identifier = &user_log_in_info.identifier;

        let email_identifier: Arc<dyn Identifier> =
            Arc::new(EmailIdentifier::new(self.user_repo.clone(), None));

        let phone_identifier: Arc<dyn Identifier> = Arc::new(PhoneIdentifier::new(
            self.user_repo.clone(),
            Some(email_identifier),
        ));

        let user_id = phone_identifier.identify(identifier).await?;

        let user_info = match self.user_repo.get_user_by_id(user_id).await? {
            Some(user_info) => user_info,
            None => return Err(Error::UserIdDontExist),
        };

        let is_valid = self
            .password_hasher
            .verify(&user_log_in_info.password, &user_info.password)?;

        if !is_valid {
            return Err(Error::InvalidPassword);
        }

        Ok(LogInResponse {
            user_id,
            user_rol: user_info.user_rol,
        })
    }
}
