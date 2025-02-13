use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type HttpResult<T> = std::result::Result<T, Response>;

pub trait HttpError<T> {
    fn http_err(self, endpoint: &str) -> HttpResult<T>;
}

pub trait ToErrResponse {
    fn to_err_response(self) -> Response;
}

impl ToErrResponse for &str {
    fn to_err_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

impl ToErrResponse for String {
    fn to_err_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
