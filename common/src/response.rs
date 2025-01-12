use crate::utils::Meta;
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
pub struct AppResponse<T = (), M = ()>
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

impl<T: Serialize, M: Serialize + Meta> Default for AppResponse<T, M> {
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

impl<T: Serialize, M: Serialize + Meta> AppResponse<T, M> {
    pub fn success(status_code: StatusCode, data: T) -> Self {
        Self {
            status_code: status_code.as_u16(),
            status_message: status_code.as_str().to_owned(),
            data: Some(data),
            ..Self::default()
        }
    }

    pub fn with_meta(status_code: StatusCode, data: T, metadata: M) -> Self {
        Self {
            status_code: status_code.as_u16(),
            status_message: status_code.canonical_reason().unwrap().to_owned(),
            data: Some(data),
            metadata: Some(metadata),
            ..Self::default()
        }
    }

    pub fn error(status_code: StatusCode, error_message: &str) -> Self {
        Self {
            status: false,
            status_code: status_code.as_u16(),
            status_message: status_code.canonical_reason().unwrap().to_owned(),
            error_message: Some(error_message.to_owned()),
            ..Self::default()
        }
    }
}

impl From<StatusCode> for AppResponse {
    fn from(code: StatusCode) -> Self {
        Self {
            status_code: code.as_u16(),
            status_message: code.canonical_reason().unwrap().to_owned(),
            ..Self::default()
        }
    }
}

impl<T: Serialize, M: Serialize + Meta> IntoResponse for AppResponse<T, M> {
    fn into_response(self) -> Response {
        Json(json!(self)).into_response()
    }
}
