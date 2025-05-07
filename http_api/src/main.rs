use std::{
    net::{Ipv4Addr, SocketAddrV4},
    sync::Arc,
};

use axum::Router;
// Import new endpoint modules if you create them (e.g., court_endpoints)
use report_endpoints::report_router;
use request_endpoints::request_router;
use serde::Deserialize;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::{error, info};
use training_endpoints::training_router;
use tuition_endpoints::tuition_router;
use turso_db::TursoDb;
use use_cases::{
    category_service::CategoryService,
    court_service::CourtService, // New
    report_service::ReportService,
    request_service::RequestService,
    tournament_service::TournamentService,
    training_service::TrainingService,
    tuition_service::TuitionService,
    user_service::UserService,
};

mod auth;
mod category_endpoints;
mod court_endpoints;
mod err;
mod report_endpoints;
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
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config: Config = envy::from_env().expect("Error generating config with the .env file");

    let turso_db_arc = Arc::new(
        TursoDb::from(&config.db_url, &config.db_token)
            .await
            .inspect_err(|err| error!("Error creating turso db: {err}"))
            .expect("Error creating turso db"),
    );

    // It's good practice to run migrations explicitly, perhaps here or via a separate script/command.
    // For simplicity, assuming migrations are handled or `TestDbBuilder` in tests takes care of it for dev.
    // let initial_conn = turso_db_arc.get_connection().await.expect("DB connection for migration");
    // initial_conn.execute_batch(&turso_db::migration::get_migration_sql()).await.expect("Migration failed");
    // info!("Database migrations applied successfully.");

    let password_hasher = Arc::new(bcrypt_hasher::BcryptHasher);
    let user_service = UserService::new(turso_db_arc.clone(), password_hasher);

    let category_service = CategoryService::new(
        turso_db_arc.clone(),
        turso_db_arc.clone(),
        turso_db_arc.clone(),
        user_service.clone(), // Pass Arc<UserService>
    );

    let court_service_arc = CourtService::new(turso_db_arc.clone(), turso_db_arc.clone()); // New

    let tuition_service_arc = TuitionService::new(turso_db_arc.clone()); // New

    let training_service = TrainingService::new(
        turso_db_arc.clone(),
        turso_db_arc.clone(),
        category_service.clone(),
        court_service_arc.clone(),   // Pass Arc<CourtService>
        user_service.clone(),        // Pass Arc<UserService>
        tuition_service_arc.clone(), // Pass Arc<TuitionService>
    );

    let tournament_service = TournamentService::new(
        turso_db_arc.clone(),
        turso_db_arc.clone(),
        turso_db_arc.clone(),
        category_service.clone(),
        court_service_arc.clone(), // Pass Arc<CourtService>
    );

    let request_service = RequestService::new(turso_db_arc.clone());

    let report_service = ReportService::new(
        user_service.clone(),
        category_service.clone(),
        training_service.clone(),
        tournament_service.clone(),
        tuition_service_arc.clone(),
        request_service.clone(),
    );

    let mut main_router = Router::new()
        .merge(user_endpoints::user_router(
            Arc::new(user_service),
            &config.token_key,
        ))
        .merge(category_endpoints::category_router(category_service))
        .merge(court_endpoints::court_router(
            court_service_arc.clone(),
            config.token_key.clone(),
        )) // New
        .merge(training_router(training_service.clone())) // Pass cloned services
        .merge(tournament_endpoints::tournament_router(
            tournament_service.clone(),
        ))
        .merge(request_router(request_service, config.token_key.clone()))
        .merge(tuition_router(
            tuition_service_arc.clone(),
            config.token_key.clone(),
        ))
        .merge(report_router(report_service));

    let cors_layer = CorsLayer::permissive();
    main_router = main_router
        .layer(cors_layer)
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), config.port);
    info!("Starting server in the addr: {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, main_router).await.unwrap();
}
