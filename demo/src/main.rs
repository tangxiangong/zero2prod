use axum::{http::StatusCode, routing::get, Router};
use common::{utils::Meta, AppResponse};
use dotenvy::dotenv;
use serde::Serialize;
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
        .route("/test", get(test))
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    info!(
        "server listener on http://{}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, route).await.unwrap();
}

#[derive(Serialize)]
struct Book {
    author: String,
}

#[derive(Serialize)]
struct BookMeta {
    number: usize,
}

impl Meta for BookMeta {
    type Item = Book;
}

#[axum::debug_handler]
async fn test() -> AppResponse<Vec<Book>, BookMeta> {
    let data = vec![
        Book {
            author: "1".to_owned(),
        },
        Book {
            author: "2".to_owned(),
        },
    ];
    let metadata = BookMeta { number: 2 };
    AppResponse::with_meta(StatusCode::OK, data, metadata)
}
