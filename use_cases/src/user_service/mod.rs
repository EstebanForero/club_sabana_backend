use std::sync::Arc;

use entities::user::{URol, UserCreation, UserCreationExtra, UserInfo, UserLogInInfo};
use repository_trait::UserRepository;

pub mod err;
pub mod repository_trait;

use err::{Error, Result};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
    user_repo: Arc<dyn UserRepository>,
}

pub struct LogInResponse {
    pub user_id: Uuid,
    pub user_rol: URol,
}

impl UserService {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn register_user(&self, user_creation: UserCreation) -> Result<()> {
        let user = user_creation.build_user(UserCreationExtra {
            id_user: Uuid::new_v4(),
            email_verified: false,
            user_rol: URol::USER,
            deleted: false,
        });

        self.user_repo.create_user(&user).await?;

        Ok(())
    }

    pub async fn get_all_users(&self) -> Result<Vec<UserInfo>> {
        let users = self.user_repo.list_users().await?;

        let users_info = users.into_iter().map(|user| UserInfo::from(user)).collect();

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

        Ok(LogInResponse {
            user_id,
            user_rol: URol::ADMIN,
        })
    }
}
