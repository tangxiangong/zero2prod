use axum::http::StatusCode;
use common::AppResponse;

pub async fn index() -> &'static str {
    "hello world"
}

pub async fn health() -> AppResponse {
    AppResponse::from(StatusCode::OK)
}
