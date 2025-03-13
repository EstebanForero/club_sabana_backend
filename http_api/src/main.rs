use std::{
    future::Future,
    net::{Ipv4Addr, SocketAddrV4},
    sync::Arc,
};

use auth::auth_middleware;
use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::{self, FromFnLayer, Next},
    response::IntoResponse,
    Router,
};
use request_endpoints::request_router;
use serde::Deserialize;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::{error, info};
use training_endpoints::training_router;
use tuition_endpoints::tuition_router;
use turso_db::TursoDb;
use use_cases::{
    category_service::CategoryService, request_service::RequestService,
    tournament_service::TournamentService, training_service::TrainingService,
    tuition_service::TuitionService, user_service::UserService,
};

mod auth;
mod category_endpoints;
mod err;
mod request_endpoints;
mod tournament_endpoints;
mod training_endpoints;
mod tuition_endpoints;
mod user_endpoints;

#[derive(Debug, Deserialize)]
struct Config {
    db_url: String,
    db_token: String,
    port: u16,
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

    let password_hasher = bcrypt_hasher::BcryptHasher;

    let user_service = UserService::new(Arc::new(turso_db.clone()), Arc::new(password_hasher));

    let category_service = CategoryService::new(
        Arc::new(turso_db.clone()),
        Arc::new(turso_db.clone()),
        Arc::new(turso_db.clone()),
        user_service.clone(),
    );

    let tournament_service = TournamentService::new(
        Arc::new(turso_db.clone()),
        Arc::new(turso_db.clone()),
        Arc::new(turso_db.clone()),
        category_service.clone(),
    );

    let request_service = RequestService::new(Arc::new(turso_db.clone()));

    let tuition_service = TuitionService::new(Arc::new(turso_db.clone()));

    let training_service = TrainingService::new(
        Arc::new(turso_db.clone()),
        Arc::new(turso_db.clone()),
        category_service.clone(),
    );

    main_router = main_router
        .merge(user_endpoints::user_router(user_service, &config.token_key))
        .merge(tournament_endpoints::tournament_router(tournament_service))
        .merge(category_endpoints::category_router(category_service))
        .merge(training_router(training_service))
        .merge(request_router(request_service))
        .merge(tuition_router(tuition_service, config.token_key));

    let cors_layer = CorsLayer::permissive();

    main_router = main_router
        .layer(cors_layer)
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), config.port);

    info!("Starting server in the addr: {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, main_router).await.unwrap();
}
