use axum::{http::StatusCode, routing::get, Router};
use common::AppResponse;
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    let route = Router::new()
        .route("/", get(|| async { "hello world" }))
        .route(
            "/health_check",
            get(|| async { AppResponse::from(StatusCode::OK) }),
        )
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    info!(
        "server listener on http://{}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, route).await.unwrap();
}