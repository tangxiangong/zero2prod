use crate::{
    impl_error_from_extract_rejection, impl_error_from_server_error, AppResponse, ResponseDetail,
};
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

impl AppError {
    pub fn new(status_code: impl Into<StatusCode>, message: impl Into<String>) -> Self {
        Self {
            status_code: status_code.into(),
            message: message.into(),
        }
    }
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

/* 将
 * std::io::Error,
 * sqlx::Error
 * 转换为 AppError (HTTP 的 INTERNAL_SERVER_ERROR)
 * */
impl_error_from_server_error!(std::io::Error, sqlx::Error,);

// axum 的 rejection 转换为 AppError
impl_error_from_extract_rejection!(
    rejection::FormRejection,
    rejection::JsonRejection,
    rejection::PathRejection,
    rejection::QueryRejection,
);

/// AppError 转换为 AppResponse
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
