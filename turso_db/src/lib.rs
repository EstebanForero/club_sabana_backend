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

        conn.execute(&migration::get_migration(), params![])
            .await
            .expect("Error applying migration");

        println!("Migration applied succesfully");

        Ok(turso_db)
    }
}
