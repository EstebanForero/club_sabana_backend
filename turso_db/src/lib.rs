use std::{error::Error, sync::Arc};

use libsql::{params, Connection};

pub mod category_repo;
mod migration;
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

    pub fn build(self) -> TursoDb {
        TursoDb {
            db: self.db,
            conn: Some(self.conn),
        }
    }
}
