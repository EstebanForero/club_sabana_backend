use std::sync::Arc;

use axum::Router;
use serde::Deserialize;
use tracing::error;
use turso_db::TursoDb;
use use_cases::user_service::UserService;

mod user_endpoints;

#[derive(Debug, Deserialize)]
struct Config {
    db_url: String,
    db_token: String,
    port: String,
    token_key: String,
}

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt::init();

    let config: Config = envy::from_env().expect("Error generating config with the .env file");

    let mut main_router = Router::new();

    let turso_db = TursoDb::from(&config.db_url, &config.db_token)
        .await
        .inspect_err(|err| error!("Error creating turso db: {err}"))
        .expect("Error creating turso db");

    let user_service = UserService::new(Arc::new(turso_db));

    main_router = main_router.merge(user_endpoints::user_router(user_service));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, main_router).await.unwrap();
}
