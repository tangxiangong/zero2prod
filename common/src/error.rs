use crate::{AppResponse, ResponseDetail};
use axum::{
    extract::rejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};

/// 自定义错误类型
#[derive(Debug)]
pub struct AppError {
    status_code: StatusCode,
    message: String,
}

impl std::error::Error for AppError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

// 使用 ？ 将其他错误自动转换为自定义错误，当返回值为 AppResult 时

/// IO 错误转换为自定义错误
impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: error.to_string(),
        }
    }
}

/// SQL 错误转换为自定义错误
impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: error.to_string(),
        }
    }
}

/// FormRejection 错误转换为自定义错误
impl From<rejection::FormRejection> for AppError {
    fn from(error: rejection::FormRejection) -> Self {
        Self {
            status_code: error.status(),
            message: error.to_string(),
        }
    }
}

impl From<AppError> for AppResponse {
    fn from(error: AppError) -> Self {
        let status_code = error.status_code;
        let message = error.message;
        (status_code, ResponseDetail::error(status_code, &message))
    }
}

/// 实现 IntoResponse trait，使得 AppError 可以作为 handler 的返回值
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        AppResponse::from(self).into_response()
    }
}

/// 自定义结果类型
pub type AppResult<T> = Result<T, AppError>;
