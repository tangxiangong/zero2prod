fn main() {
    println!("hello world");
}

// use axum::routing::get;
// use axum::Router;
// // use dotenvy::dotenv;
// use tokio::net::TcpListener;
// use tower_http::trace::TraceLayer;
// use tracing::info;
//
// #[tokio::main]
// async fn main() {
//     dotenv().ok();
//     tracing_subscriber::fmt::init();
//     let route = Router::new()
//         .route("/", get(|| async { "hello world" }))
//         .layer(TraceLayer::new_for_http());
//     let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
//     info!("server listener on {}", listener.local_addr().unwrap());
//     axum::serve(listener, route).await.unwrap();
// }
