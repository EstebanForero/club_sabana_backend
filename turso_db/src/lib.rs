use std::{error::Error, sync::Arc};

pub mod category_repo;
pub mod tournament_repo;
pub mod training_repo;
pub mod tuition_repo;
pub mod user_repo;

#[derive(Clone)]
pub struct TursoDb {
    db: Arc<libsql::Database>,
}

impl TursoDb {
    pub async fn from(url: &str, token: &str) -> std::result::Result<TursoDb, Box<dyn Error>> {
        let db = libsql::Builder::new_remote(url.to_string(), token.to_string())
            .build()
            .await?;

        Ok(Self { db: Arc::new(db) })
    }

    async fn get_connection(&self) -> Result<libsql::Connection, Box<dyn Error>> {
        Ok(self.db.connect()?)
    }
}
