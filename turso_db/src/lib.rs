use std::sync::Arc;

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
    pub async fn from(url: &str, token: &str) -> std::result::Result<TursoDb, String> {
        let db = libsql::Builder::new_remote(url.to_string(), token.to_string())
            .build()
            .await
            .map_err(|err| format!("Error creating new remote database for libsql: {err}"))?;

        Ok(Self { db: Arc::new(db) })
    }

    async fn get_connection(&self) -> Result<libsql::Connection, String> {
        self.db
            .connect()
            .map_err(|err| format!("Error getting db connection: {err}"))
    }
}
