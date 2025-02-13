use async_trait::async_trait;
use entities::user::{IdType, User};
use libsql::{de, params};
use use_cases::user_service::err::{Error, Result};
use use_cases::user_service::repository_trait::UserRepository;
use uuid::Uuid;

use crate::TursoDb;

#[async_trait]
impl UserRepository for TursoDb {
    async fn create_user(&self, user: &User) -> Result<()> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        conn.execute(
            "INSERT INTO person (
                id_user, first_name, last_name, birth_date, registration_date, 
                email, email_verified, phone_number, country_code, password, 
                identification_number, identification_type, user_rol
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
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
                user.identification_type.to_string(),
                user.user_rol.to_string(),
            ],
        )
        .await
        .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        Ok(())
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("connection error: {err}")))?;

        let mut rows = conn
            .query(
                "SELECT id_user, first_name, last_name, birth_date, registration_date, email,
email_verified, phone_number, country_code, password, identification_number, identification_type, user_rol FROM person WHERE id_user = ?1 AND deleted = 0",
                params![id.to_string()],
            )
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        if let Some(row_result) = rows
            .next()
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?
        {
            let row = row_result;
            let user = de::from_row::<User>(&row)
                .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    async fn get_user_id_by_email(&self, email: &str) -> Result<Option<Uuid>> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        let mut rows = conn
            .query(
                "SELECT id_user FROM person 
                 WHERE email = ?1 AND deleted = 0",
                params![email],
            )
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        if let Some(row_result) = rows
            .next()
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?
        {
            let id_str: String = row_result
                .get(0)
                .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

            let uuid = Uuid::parse_str(&id_str)
                .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;
            Ok(Some(uuid))
        } else {
            Ok(None)
        }
    }

    async fn get_user_id_by_phone(&self, phone_number: &str) -> Result<Option<Uuid>> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        let mut rows = conn
            .query(
                "SELECT id_user FROM person 
                 WHERE phone_number = ?1 AND deleted = 0",
                params![phone_number],
            )
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        if let Some(row_result) = rows
            .next()
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?
        {
            let id_str: String = row_result
                .get(0)
                .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

            let uuid = Uuid::parse_str(&id_str)
                .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;
            Ok(Some(uuid))
        } else {
            Ok(None)
        }
    }

    async fn get_user_id_by_identification(
        &self,
        identification_number: &str,
        identification_type: &IdType,
    ) -> Result<Option<Uuid>> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        let mut rows = conn
            .query(
                "SELECT id_user FROM person 
                 WHERE identification_number = ?1 
AND identification_type = ?2 
AND deleted = 0",
                params![identification_number, identification_type.to_string()],
            )
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        if let Some(row_result) = rows
            .next()
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?
        {
            let id_str: String = row_result
                .get(0)
                .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;
            let uuid = Uuid::parse_str(&id_str)
                .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;
            Ok(Some(uuid))
        } else {
            Ok(None)
        }
    }

    async fn update_user(&self, user: &User) -> Result<()> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        conn.execute(
            "UPDATE person SET 
first_name = ?1, 
last_name = ?2, 
birth_date = ?3, 
registration_date = ?4, 
email = ?5, 
email_verified = ?6, 
phone_number = ?7, 
country_code = ?8, 
password = ?9, 
identification_number = ?10, 
identification_type = ?11, 
user_rol = ?12,
deleted = ?13
WHERE id_user = ?14",
            params![
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
                user.identification_type.to_string(),
                user.user_rol.to_string(),
                user.id_user.to_string(),
            ],
        )
        .await
        .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        Ok(())
    }

    async fn delete_user(&self, id: Uuid) -> Result<()> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        conn.execute(
            "UPDATE person SET deleted = 1 WHERE id_user = ?1",
            params![id.to_string()],
        )
        .await
        .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        Ok(())
    }

    async fn list_users(&self) -> Result<Vec<User>> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        let mut rows = conn
            .query(
                "SELECT id_user, first_name, last_name, birth_date, registration_date, email,
email_verified, phone_number, country_code, password, identification_number,
identification_type, user_rol 
FROM person 
WHERE deleted = 0",
                params![],
            )
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        let mut users = Vec::new();
        while let Some(row_result) = rows
            .next()
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?
        {
            let user = de::from_row::<User>(&row_result)
                .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;
            users.push(user);
        }

        Ok(users)
    }
}

#[cfg(test)]
mod test {
    use std::{future::Future, process::Output};

    use entities::user::{IdType, User};
    use rstest::{fixture, rstest};
    use use_cases::user_service::repository_trait::UserRepository;
    use uuid::Uuid;

    use crate::{TestDbBuilder, TursoDb};

    #[fixture]
    async fn repository() -> TursoDb {
        let db = TestDbBuilder::create()
            .await
            .apply_doc_types()
            .await
            .apply_user_roles()
            .await
            .build();

        db
    }

    #[rstest]
    #[tokio::test]
    async fn test_create_user(repository: impl Future<Output = TursoDb>) {
        let user_id = Uuid::new_v4();

        let db = repository.await;

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

    #[rstest]
    #[tokio::test]
    async fn test_get_user_by_id(repository: impl Future<Output = TursoDb>) {
        let user_id = Uuid::new_v4();

        let db = repository.await;

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

        assert_eq!(user, user_db);

        let user_db = db
            .get_user_by_id(Uuid::new_v4())
            .await
            .expect("Error getting user by id");

        assert!(user_db.is_none());
    }

    #[rstest]
    #[tokio::test]
    async fn test_get_user_id_by_email(repository: impl Future<Output = TursoDb>) {
        let user_id = Uuid::new_v4();
        let email = "test_email@example.com".to_string();
        let db = repository.await;

        let user = User {
            id_user: user_id,
            email: email.clone(),
            phone_number: "1234567890".to_string(),
            identification_number: "ID_EMAIL_1".to_string(),
            ..User::default()
        };

        db.create_user(&user).await.expect("Error creating user");

        let fetched_id = db
            .get_user_id_by_email(&email)
            .await
            .expect("Error fetching user id by email");
        assert_eq!(fetched_id, Some(user_id));
    }

    #[rstest]
    #[tokio::test]
    async fn test_get_user_id_by_phone(repository: impl Future<Output = TursoDb>) {
        let user_id = Uuid::new_v4();
        let phone_number = "555-1234".to_string();
        let db = repository.await;

        let user = User {
            id_user: user_id,
            email: "test_phone@example.com".to_string(),
            phone_number: phone_number.clone(),
            identification_number: "ID_PHONE_1".to_string(),
            ..User::default()
        };

        db.create_user(&user).await.expect("Error creating user");

        let fetched_id = db
            .get_user_id_by_phone(&phone_number)
            .await
            .expect("Error fetching user id by phone");
        assert_eq!(fetched_id, Some(user_id));
    }

    #[rstest]
    #[tokio::test]
    async fn test_get_user_id_by_identification(repository: impl Future<Output = TursoDb>) {
        let user_id = Uuid::new_v4();
        let identification_number = "ID-IDENT-1234".to_string();
        let identification_type = IdType::default(); // default is CC, for example
        let db = repository.await;

        let user = User {
            id_user: user_id,
            email: "test_ident@example.com".to_string(),
            identification_number: identification_number.clone(),
            identification_type,
            phone_number: "9876543210".to_string(),
            ..User::default()
        };

        db.create_user(&user).await.expect("Error creating user");

        let fetched_id = db
            .get_user_id_by_identification(&identification_number, &IdType::default())
            .await
            .expect("Error fetching user id by identification");
        assert_eq!(fetched_id, Some(user_id));
    }

    #[rstest]
    #[tokio::test]
    async fn test_update_user(repository: impl Future<Output = TursoDb>) {
        let user_id = Uuid::new_v4();
        let db = repository.await;

        let mut user = User {
            id_user: user_id,
            email: "original@example.com".to_string(),
            first_name: "Original".to_string(),
            phone_number: "0001112222".to_string(),
            identification_number: "ID_UPDATE_1".to_string(),
            ..User::default()
        };

        db.create_user(&user).await.expect("Error creating user");

        // Update some fields.
        user.first_name = "Updated".to_string();
        user.email = "updated@example.com".to_string();

        db.update_user(&user).await.expect("Error updating user");

        let updated_user = db
            .get_user_by_id(user_id)
            .await
            .expect("Error fetching updated user")
            .expect("User not found after update");

        assert_ne!(user, updated_user);
    }

    #[rstest]
    #[tokio::test]
    async fn test_delete_user(repository: impl Future<Output = TursoDb>) {
        let user_id = Uuid::new_v4();
        let db = repository.await;

        let user = User {
            id_user: user_id,
            email: "delete_me@example.com".to_string(),
            ..User::default()
        };

        db.create_user(&user).await.expect("Error creating user");

        db.delete_user(user_id).await.expect("Error deleting user");

        let deleted_user = db
            .get_user_by_id(user_id)
            .await
            .expect("Error fetching user after delete");
        assert!(deleted_user.is_none());
    }

    #[rstest]
    #[tokio::test]
    async fn test_list_users(repository: impl Future<Output = TursoDb>) {
        let db = repository.await;

        // Get the initial list of users.
        let initial_users = db.list_users().await.expect("Error listing initial users");
        let initial_count = initial_users.len();

        // Create two users.
        let user1 = User {
            id_user: Uuid::new_v4(),
            email: "list1@example.com".to_string(),
            phone_number: "111111".to_string(),
            identification_number: "ID_LIST_1".to_string(),
            ..User::default()
        };
        let user2 = User {
            id_user: Uuid::new_v4(),
            email: "list2@example.com".to_string(),
            phone_number: "222222".to_string(),
            identification_number: "ID_LIST_2".to_string(),
            ..User::default()
        };

        db.create_user(&user1).await.expect("Error creating user1");
        db.create_user(&user2).await.expect("Error creating user2");

        let users_after_insert = db
            .list_users()
            .await
            .expect("Error listing users after insert");
        assert_eq!(users_after_insert.len(), initial_count + 2);

        // Delete one user.
        db.delete_user(user1.id_user)
            .await
            .expect("Error deleting user1");

        let users_after_delete = db
            .list_users()
            .await
            .expect("Error listing users after deletion");
        assert_eq!(users_after_delete.len(), initial_count + 1);

        // Ensure that user1 is no longer listed.
        for user in users_after_delete {
            assert_ne!(user.id_user, user1.id_user);
        }
    }
}
