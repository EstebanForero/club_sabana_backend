use std::{error::Error, sync::Arc};

use libsql::params;
use libsql::{de, params::IntoParams, Connection, Rows};
use serde::Deserialize;
use uuid::Uuid;

pub mod category_repo;
mod migration;
pub mod request_repo;
pub mod tournament_repo;
pub mod training_repo;
pub mod tuition_repo;
pub mod user_repo;

#[derive(Clone)]
pub struct TursoDb {
    db: Arc<libsql::Database>,
    conn: Option<Connection>,
}

impl TursoDb {
    pub async fn from(url: &str, token: &str) -> Result<TursoDb, Box<dyn Error>> {
        let db = libsql::Builder::new_remote(url.to_string(), token.to_string())
            .build()
            .await?;

        Ok(Self {
            db: Arc::new(db),
            conn: None,
        })
    }

    async fn get_connection(&self) -> Result<libsql::Connection, Box<dyn Error>> {
        match self.conn.clone() {
            Some(conn) => Ok(conn),
            None => Ok(self.db.connect()?),
        }
    }

    async fn get_connection_with_error<E>(
        &self,
        error_builder: impl Fn(String) -> E,
    ) -> Result<libsql::Connection, E> {
        match self.conn.clone() {
            Some(conn) => Ok(conn),
            None => Ok(self
                .db
                .connect()
                .map_err(|err| error_builder(format!("Error in connection: {err}"))))?,
        }
    }

    pub async fn query_one_with_error<T, E>(
        &self,
        sql: &str,
        params: impl IntoParams,
        error_builder: impl Fn(String) -> E,
    ) -> Result<Option<T>, E>
    where
        T: for<'de> Deserialize<'de>,
    {
        let conn = self.get_connection_with_error(&error_builder).await?;

        let rows = conn.query(sql, params).await;

        let value: Option<T> = self.get_value_from_row(rows, error_builder).await?;

        Ok(value)
    }

    pub async fn query_many_with_error<T, E>(
        &self,
        sql: &str,
        params: impl IntoParams,
        error_builder: impl Fn(String) -> E,
    ) -> Result<Vec<T>, E>
    where
        T: for<'de> Deserialize<'de>,
    {
        let conn = self.get_connection_with_error(&error_builder).await?;

        let rows = conn.query(sql, params).await;

        let value: Vec<T> = self.get_values_from_rows(rows, error_builder).await?;

        Ok(value)
    }

    pub async fn execute_with_error<E>(
        &self,
        sql: &str,
        params: impl IntoParams,
        error_builder: impl Fn(String) -> E,
    ) -> Result<(), E> {
        let conn = self.get_connection_with_error(&error_builder).await?;

        conn.execute(sql, params)
            .await
            .map_err(|err| error_builder(err.to_string()))?;

        Ok(())
    }

    pub async fn get_values_from_rows<T, E>(
        &self,
        rows: Result<Rows, libsql::Error>,
        error_builder: impl Fn(String) -> E,
    ) -> Result<Vec<T>, E>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut rows = rows.map_err(|err| error_builder(err.to_string()))?;

        let mut elements = Vec::new();

        while let Some(row_result) = rows
            .next()
            .await
            .map_err(|err| error_builder(err.to_string()))?
        {
            let element =
                de::from_row::<T>(&row_result).map_err(|err| error_builder(err.to_string()))?;
            elements.push(element);
        }

        Ok(elements)
    }

    pub async fn create_test_user(&self, user_id: Uuid) -> Result<(), Box<dyn Error>> {
        self.conn
            .clone()
            .unwrap()
            .execute(
                "INSERT INTO person (
id_user, first_name, last_name, birth_date, registration_date,
email, email_verified, phone_number, country_code, password,
identification_number, identification_type, user_rol, deleted
) VALUES (?1, 'Test', 'User', '2000-01-01', '2023-01-01 00:00:00',
?2, 1, '123456789', 'CO', 'password',
'123456789', 'CC', 'USER', 0)",
                params![user_id.to_string(), format!("test{}@example.com", user_id)],
            )
            .await?;

        Ok(())
    }

    pub async fn get_value_from_row<T, E>(
        &self,
        rows: Result<Rows, libsql::Error>,
        error_builder: impl Fn(String) -> E,
    ) -> Result<Option<T>, E>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut rows = rows.map_err(|err| error_builder(err.to_string()))?;

        if let Some(row_result) = rows
            .next()
            .await
            .map_err(|err| error_builder(err.to_string()))?
        {
            let element =
                de::from_row::<T>(&row_result).map_err(|err| error_builder(err.to_string()))?;

            Ok(Some(element))
        } else {
            Ok(None)
        }
    }
}

pub struct TestDbBuilder {
    db: Arc<libsql::Database>,
    conn: Connection,
}

impl TestDbBuilder {
    pub async fn create() -> Self {
        let db = libsql::Builder::new_local(":memory:")
            .build()
            .await
            .expect("Error building in memory db");

        let conn = db.connect().expect("Error getting connection");

        let turso_db = Self {
            db: Arc::new(db),
            conn: conn.clone(),
        };

        conn.execute_batch(&migration::get_migration())
            .await
            .expect("Error applying migration");

        println!("Migration applied successfully");

        turso_db
    }

    pub async fn create_full() -> TursoDb {
        Self::create()
            .await
            .apply_doc_types()
            .await
            .apply_user_roles()
            .await
            .build()
    }

    pub async fn print_tables(&self) {
        let conn = self.conn.clone();

        let mut rows = conn
            .query(
                "
SELECT name, sql
FROM sqlite_master
WHERE type = 'table';
",
                params![],
            )
            .await
            .expect("Error getting tables info");

        while let Some(row) = rows.next().await.unwrap() {
            let table_name = row.get_str(0).unwrap_or("Unknown table");
            let table_sql = row.get_str(1).unwrap_or("No SQL available");
            println!("Table: {}\nSQL: {}\n", table_name, table_sql);
        }
    }

    pub async fn apply_doc_types(self) -> Self {
        self.conn
            .execute(
                "INSERT INTO identification_type (identification_type, deleted) 
VALUES ('CC', false)",
                params![],
            )
            .await
            .unwrap();

        self
    }

    pub async fn apply_user_roles(self) -> Self {
        self.conn
            .execute(
                "INSERT INTO user_rol (user_rol, deleted) 
VALUES ('ADMIN', 0), ('USER', 0), ('TRAINER', 0)",
                params![],
            )
            .await
            .unwrap();

        self
    }

    pub async fn apply_levels(self) -> Self {
        self.conn
            .execute(
                "INSERT INTO user_rol (user_rol) 
VALUES ('BEGGINER'), ('AMATEUR'), ('PROFESSIONAL')",
                params![],
            )
            .await
            .unwrap();

        self
    }

    pub fn build(self) -> TursoDb {
        TursoDb {
            db: self.db,
            conn: Some(self.conn),
        }
    }
}
