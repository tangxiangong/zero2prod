use axum::http::StatusCode;
use common::{AppResponse, ResponseDetail};

pub async fn index() -> &'static str {
    "hello world"
}

pub async fn health() -> AppResponse {
    (StatusCode::OK, ResponseDetail::from(StatusCode::OK))
}
