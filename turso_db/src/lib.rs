use std::{error::Error, sync::Arc};

use libsql::params;

pub mod category_repo;
mod migration;
pub mod tournament_repo;
pub mod training_repo;
pub mod tuition_repo;
pub mod user_repo;

#[derive(Clone)]
pub struct TursoDb {
    db: Arc<libsql::Database>,
}

impl TursoDb {
    pub async fn from(url: &str, token: &str) -> Result<TursoDb, Box<dyn Error>> {
        let db = libsql::Builder::new_remote(url.to_string(), token.to_string())
            .build()
            .await?;

        Ok(Self { db: Arc::new(db) })
    }

    async fn get_connection(&self) -> Result<libsql::Connection, Box<dyn Error>> {
        Ok(self.db.connect()?)
    }

    pub async fn test_db() -> Result<TursoDb, Box<dyn Error>> {
        let db = libsql::Builder::new_local(":memory:").build().await?;

        let turso_db = Self { db: Arc::new(db) };

        let conn = turso_db.get_connection().await?;

        conn.execute_batch(&migration::get_migration())
            .await
            .expect("Error applying migration");

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

        while let Some(row) = rows.next().await? {
            let table_name = row.get_str(0).unwrap_or("Unknown table");
            let table_sql = row.get_str(1).unwrap_or("No SQL available");
            println!("Table: {}\nSQL: {}\n", table_name, table_sql);
        }

        println!("Migration applied successfully");

        Ok(turso_db)
    }
}
