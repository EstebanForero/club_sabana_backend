use std::sync::Arc;

use entities::user::{URol, UserCreation, UserCreationExtra, UserInfo, UserLogInInfo};
use hasher_trait::PasswordHasher;
use repository_trait::UserRepository;

pub mod err;
pub mod hasher_trait;
pub mod repository_trait;

use err::{Error, Result};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
    user_repo: Arc<dyn UserRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
}

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

        let mut user = user_creation.build_user(UserCreationExtra {
            id_user: Uuid::new_v4(),
            email_verified: false,
            user_rol: URol::USER,
            deleted: false,
        });

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

    pub async fn log_in_user(&self, user_log_in_info: UserLogInInfo) -> Result<LogInResponse> {
        let identifier = user_log_in_info.identifier;

        let user_id =
            if let Some(user_id) = self.user_repo.get_user_id_by_email(&identifier).await? {
                user_id
            } else if let Some(user_id) = self.user_repo.get_user_id_by_phone(&identifier).await? {
                user_id
            } else {
                return Err(Error::UserIdDontExist);
            };

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
            user_rol: URol::ADMIN,
        })
    }
}
