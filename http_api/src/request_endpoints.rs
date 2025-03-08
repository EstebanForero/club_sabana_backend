use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use entities::request::Request;
use tracing::error;
use use_cases::request_service::{err::Error, RequestService};
use uuid::Uuid;

use super::err::{HttpError, ToErrResponse};
use crate::{auth::UserInfoAuth, err::HttpResult};

pub fn request_router(request_service: RequestService) -> Router {
    Router::new()
        .route("/health-request", get(alive))
        .route("/requests", post(create_request).get(list_requests))
        .route("/requests/{id}", get(get_request))
        .route("/requests/{id}/complete/{approved}", post(complete_request))
        .route("/requests/user/{user_id}", get(list_user_requests))
        .with_state(request_service)
}

async fn alive() -> &'static str {
    "Request service is alive"
}

async fn create_request(
    State(request_service): State<RequestService>,
    Json(request): Json<Request>,
) -> HttpResult<impl IntoResponse> {
    request_service
        .create_request(
            request.requester_id,
            request.requested_command,
            request.justification,
        )
        .await
        .http_err("create request")?;

    Ok((StatusCode::CREATED, "Request created successfully"))
}

async fn get_request(
    State(request_service): State<RequestService>,
    Path(id): Path<Uuid>,
) -> HttpResult<Json<Request>> {
    let request = request_service
        .get_request_by_id(id)
        .await
        .http_err("get request")?
        .ok_or(Error::RequestNotFound)
        .http_err("get request")?;

    Ok(Json(request))
}

async fn complete_request(
    State(request_service): State<RequestService>,
    Path((id, approved)): Path<(Uuid, bool)>,
    Extension(user_info): Extension<UserInfoAuth>,
) -> HttpResult<impl IntoResponse> {
    request_service
        .complete_request(id, user_info.user_id, approved)
        .await
        .http_err("complete request")?;

    Ok((StatusCode::OK, "Request completed successfully"))
}

async fn list_requests(
    State(request_service): State<RequestService>,
) -> HttpResult<Json<Vec<Request>>> {
    let requests = request_service
        .list_requests()
        .await
        .http_err("list requests")?;

    Ok(Json(requests))
}

async fn list_user_requests(
    State(request_service): State<RequestService>,
    Path(user_id): Path<Uuid>,
) -> HttpResult<Json<Vec<Request>>> {
    let requests = request_service
        .list_user_requests(user_id)
        .await
        .http_err("list user requests")?;

    Ok(Json(requests))
}

impl<T> HttpError<T> for use_cases::request_service::err::Result<T> {
    fn http_err(self, endpoint_name: &str) -> crate::err::HttpResult<T> {
        self.map_err(|err| {
            error!("Error in: {endpoint_name}");
            match err {
                Error::UnknownDatabaseError(error) => {
                    error!("{error}");
                    "We are having problems in the server, try again"
                }
                Error::RequestNotFound => "Request not found",
                Error::RequestAlreadyCompleted => "Request already completed",
                Error::SelfApprovalNotAllowed => "Cannot approve/reject your own request",
                Error::InvalidApprover => "Invalid approver ID",
            }
            .to_err_response()
        })
    }
}
