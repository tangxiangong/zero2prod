use crate::meta::Meta;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::prelude::{DateTime, Local};
use serde::Serialize;
use serde_json::json;

/// 自定义响应类型
///
/// # Filed
///
/// - `status:bool` **响应状态, 是否成功**
/// - `status_code:u16` **HTTP 响应的状态码**
/// - `status_message:String` **响应状态信息**
/// - `local_time:DateTime<Local>` **响应时的本地时间**
/// - `error_message:Option<String>` **错误响应对应的错误信息**
/// - `data:Option<T>` **响应数据**
/// - `metadata:Option<M>` **响应数据对应的元数据**
#[derive(Serialize)]
pub struct ResponseDetail<T = (), M = ()>
where
    M: Meta,
{
    #[serde(rename = "success")]
    status: bool,
    status_code: u16,
    status_message: String,
    local_time: DateTime<Local>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<M>,
}

pub type AppResponse<T = (), M = ()> = (StatusCode, ResponseDetail<T, M>);
pub type SuccessResponse<T> = ResponseDetail<T, ()>;
pub type ErrorResponse = ResponseDetail<(), ()>;

impl<T, M: Meta> Default for ResponseDetail<T, M> {
    fn default() -> Self {
        Self {
            status: true,
            status_code: StatusCode::OK.as_u16(),
            status_message: StatusCode::OK.canonical_reason().unwrap().to_owned(),
            local_time: Local::now(),
            error_message: None,
            data: None,
            metadata: None,
        }
    }
}

impl<T, M: Meta> ResponseDetail<T, M> {
    pub fn with_meta(status_code: StatusCode, data: T, metadata: M) -> Self {
        Self {
            status_code: status_code.as_u16(),
            status_message: status_code.canonical_reason().unwrap().to_owned(),
            data: Some(data),
            metadata: Some(metadata),
            ..Self::default()
        }
    }
}

impl<T> SuccessResponse<T> {
    pub fn with_data(status_code: StatusCode, data: T) -> Self {
        Self {
            status: true,
            status_code: status_code.as_u16(),
            status_message: status_code.canonical_reason().unwrap().to_owned(),
            local_time: Local::now(),
            data: Some(data),
            error_message: None,
            metadata: None,
        }
    }
}

impl ErrorResponse {
    pub fn with_error(status_code: StatusCode, error_message: &str) -> Self {
        Self {
            status: false,
            status_code: status_code.as_u16(),
            status_message: status_code.canonical_reason().unwrap().to_owned(),
            local_time: Local::now(),
            data: None,
            error_message: Some(error_message.to_owned()),
            metadata: None,
        }
    }
}

impl From<StatusCode> for ResponseDetail {
    fn from(code: StatusCode) -> Self {
        Self {
            status_code: code.as_u16(),
            status_message: code.canonical_reason().unwrap().to_owned(),
            ..Self::default()
        }
    }
}

impl<T: Serialize, M: Serialize + Meta> IntoResponse for ResponseDetail<T, M> {
    fn into_response(self) -> Response {
        Json(json!(self)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use serde_json::Value;

    #[test]
    fn test_response_detail_default() {
        let response = ResponseDetail::<Value, Value>::default();
        assert!(response.status);
        assert_eq!(response.status_code, StatusCode::OK.as_u16());
        assert_eq!(
            response.status_message,
            StatusCode::OK.canonical_reason().unwrap()
        );
        assert!(response.error_message.is_none());
        assert!(response.data.is_none());
        assert!(response.metadata.is_none());
    }

    #[test]
    fn test_response_detail_success() {
        let response = ResponseDetail::with_data(StatusCode::CREATED, json!({ "id": 1 }));
        assert!(response.status);
        assert_eq!(response.status_code, StatusCode::CREATED.as_u16());
        assert_eq!(
            response.status_message,
            StatusCode::CREATED.canonical_reason().unwrap()
        );
        assert!(response.error_message.is_none());
        assert_eq!(response.data.unwrap(), json!({ "id": 1 }));
        assert!(response.metadata.is_none());
    }

    #[test]
    fn test_response_detail_with_meta() {
        let response = ResponseDetail::with_meta(
            StatusCode::CREATED,
            json!({ "id": 1 }),
            json!({ "total": 1 }),
        );
        assert!(response.status);
        assert_eq!(response.status_code, StatusCode::CREATED.as_u16());
        assert_eq!(
            response.status_message,
            StatusCode::CREATED.canonical_reason().unwrap()
        );
        assert!(response.error_message.is_none());
        assert_eq!(response.data.unwrap(), json!({ "id": 1 }));
        assert_eq!(response.metadata.unwrap(), json!({ "total": 1 }));
    }

    #[test]
    fn test_response_detail_error() {
        let response = ResponseDetail::with_error(StatusCode::BAD_REQUEST, "Bad Request");
        assert!(!response.status);
        assert_eq!(response.status_code, StatusCode::BAD_REQUEST.as_u16());
        assert_eq!(
            response.status_message,
            StatusCode::BAD_REQUEST.canonical_reason().unwrap()
        );
        assert_eq!(response.error_message.unwrap(), "Bad Request");
        assert!(response.data.is_none());
        assert!(response.metadata.is_none());
    }
}
