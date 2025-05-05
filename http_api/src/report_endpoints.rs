use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use entities::report::Report;
use uuid::Uuid;

use use_cases::report_service::ReportService;

pub fn report_router(report_service: ReportService) -> Router {
    Router::new()
        .route("/reports/user/{user_id}", get(get_user_report))
        .with_state(report_service)
}

async fn get_user_report(
    State(report_service): State<ReportService>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Report>, (StatusCode, String)> {
    match report_service.generate_user_report(user_id).await {
        Ok(report) => Ok(Json(report)),
        Err(e) => {
            tracing::error!("Error generating report for user {}: {:?}", user_id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error generating report in the backend".to_string(),
            ))
        }
    }
}
