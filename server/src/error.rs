use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use tracing::log::error;

// https://nick.groenen.me/posts/rust-error-handling/
// TODO: Do something with errors
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("")]
    RecordNotFound,
    #[error("")]
    BadRequest,
    #[error("")] // Can also use {0} with tuple struct
    Internal(#[source] sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::RecordNotFound => (StatusCode::NOT_FOUND, ""),
            AppError::BadRequest => (StatusCode::BAD_REQUEST, ""),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, ""),
        };

        let body = Json(serde_json::json!({
            "msg": error_message,
        }));

        (status, body).into_response()
    }
}

pub fn sqlx_error(source: sqlx::Error) -> AppError {
    match source {
        sqlx::Error::RowNotFound => AppError::RecordNotFound,
        _ => AppError::Internal(source),
    }
}
