use async_trait::async_trait;
use entities::user::User;
use libsql::{de, params};
use use_cases::user_service::err::{Error, Result};
use use_cases::user_service::repository_trait::UserRepository;
use uuid::Uuid;

use crate::TursoDb;

#[async_trait]
impl UserRepository for TursoDb {
    async fn create_user(&self, user: &User) -> Result<()> {
        let conn = self.get_connection().await?;

        conn.execute(
            "INSERT INTO \"user\" (
                id_user, first_name, last_name, birth_date, registration_date, 
                email, email_verified, phone_number, country_code, password, 
                identification_number, identification_type, user_rol, deleted
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            params![
                user.id_user.to_string(),
                user.first_name.to_string(),
                user.last_name.to_string(),
                user.birth_date.format("%Y-%m-%d").to_string(),
                user.registration_date
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
                user.email.to_string(),
                user.email_verified as i32,
                user.phone_number.to_string(),
                user.country_code.to_string(),
                user.password.to_string(),
                user.identification_number.to_string(),
                format!("{:?}", user.identification_type),
                user.user_rol.to_string(),
                user.deleted as i32,
            ],
        )
        .await
        .map_err::<Box<dyn std::error::Error>, _>(|err| Box::new(err))?;

        Ok(())
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let conn = self.get_connection().await?;

        let mut rows = conn
            .query(
                "SELECT name, age, vision, avatar FROM \"user\" WHERE id = ?1",
                params![id.to_string()],
            )
            .await
            .map_err::<Box<dyn std::error::Error>, _>(|err| Box::new(err))?;

        if let Some(row_result) = rows
            .next()
            .await
            .map_err::<Box<dyn std::error::Error>, _>(|err| Box::new(err))?
        {
            let row = row_result;
            let user = de::from_row::<User>(&row)
                .map_err::<Box<dyn std::error::Error>, _>(|err| Box::new(err))?;
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        todo!()
    }

    async fn update_user(&self, user: &User) -> Result<()> {
        todo!()
    }

    async fn delete_user(&self, id: Uuid) -> Result<()> {
        todo!()
    }

    async fn list_users(&self) -> Result<Vec<User>> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use entities::user::User;
    use use_cases::user_service::repository_trait::UserRepository;
    use uuid::Uuid;

    use crate::TursoDb;

    #[tokio::test]
    async fn test_create_user() {
        let db = TursoDb::test_db()
            .await
            .expect("Failed to initialize test db");

        let user_id = Uuid::new_v4();

        let user = User {
            id_user: user_id,
            email: "estebanmff@gmail.com".to_string(),
            ..User::default()
        };

        db.create_user(&user).await.expect("Error creating user");

        let user_db = db
            .get_user_by_id(user_id)
            .await
            .expect("Error getting user by id")
            .expect("User was not added");

        assert_eq!(user, user_db)
    }
}
