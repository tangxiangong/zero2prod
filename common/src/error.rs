use crate::{AppResponse, ResponseDetail};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

/// 自定义错误类型
#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Internal Server Error")]
    InternalError,
    #[error("SQL Error: {0}")]
    SQLError(#[from] sqlx::Error),
}

impl From<AppError> for AppResponse {
    fn from(error: AppError) -> Self {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            ResponseDetail::error(StatusCode::INTERNAL_SERVER_ERROR, &error.to_string()),
        )
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        AppResponse::from(self).into_response()
    }
}

/// 自定义结果类型
pub type AppResult<T> = Result<T, AppError>;
