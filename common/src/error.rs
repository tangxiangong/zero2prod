use crate::{
    impl_error_from_extract_rejection, impl_error_from_server_error, AppResponse, ErrorResponse,
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
impl_error_from_server_error!(std::io::Error, sqlx::Error, utils::SnowflakeError);

// axum 的 rejection 转换为 AppError
impl_error_from_extract_rejection!(
    rejection::FormRejection,
    rejection::JsonRejection,
    rejection::PathRejection,
    rejection::QueryRejection,
    rejection::StringRejection,
);

/// AppError 转换为 AppResponse
impl From<AppError> for AppResponse {
    fn from(error: AppError) -> Self {
        let status_code = error.status_code;
        let message = error.message;
        (
            status_code,
            ErrorResponse::with_error(status_code, &message),
        )
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use std::io;

    #[test]
    fn test_app_error() {
        let error = AppError::new(StatusCode::BAD_REQUEST, "Bad Request");
        assert_eq!(error.status_code, StatusCode::BAD_REQUEST);
        assert_eq!(error.message, "Bad Request");
    }

    #[test]
    fn test_app_error_from_io_error() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "Not Found");
        let error_message = io_error.to_string();
        let app_error: AppError = io_error.into();
        assert_eq!(app_error.status_code, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(app_error.message, error_message);
    }

    #[test]
    fn test_app_error_from_sqlx_error() {
        let sqlx_error = sqlx::Error::Io(io::Error::new(io::ErrorKind::NotFound, "Not Found"));
        let error_message = sqlx_error.to_string();
        let app_error: AppError = sqlx_error.into();
        assert_eq!(app_error.status_code, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(app_error.message, error_message);
    }

    #[test]
    fn test_app_error_from_form_rejection() {
        let form_rejection =
            rejection::FormRejection::from(rejection::InvalidFormContentType::default());
        let rejection_status = form_rejection.status();
        let rejection_message = form_rejection.body_text();
        let app_error: AppError = form_rejection.into();
        assert_eq!(app_error.status_code, rejection_status);
        assert_eq!(app_error.message, rejection_message);
    }

    #[test]
    fn test_app_error_from_json_rejection() {
        let json_rejection =
            rejection::JsonRejection::from(rejection::MissingJsonContentType::default());
        let rejection_status = json_rejection.status();
        let rejection_message = json_rejection.body_text();
        let app_error: AppError = json_rejection.into();
        assert_eq!(app_error.status_code, rejection_status);
        assert_eq!(app_error.message, rejection_message);
    }

    #[test]
    fn test_app_error_from_path_rejection() {
        let path_rejection =
            rejection::PathRejection::from(rejection::MissingPathParams::default());
        let rejection_status = path_rejection.status();
        let rejection_message = path_rejection.body_text();
        let app_error: AppError = path_rejection.into();
        assert_eq!(app_error.status_code, rejection_status);
        assert_eq!(app_error.message, rejection_message);
    }
}
