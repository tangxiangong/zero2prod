use tower_http::trace::TraceLayer;
use tracing::info;

#[path = "config.rs"]
mod config;

#[tokio::main]
async fn main() {
    let (pool, listener) = config::config().await;

    tracing_subscriber::fmt::init();
    let app = api::compose()
        .layer(TraceLayer::new_for_http())
        .with_state(pool);
    info!(
        "server listener on http://{}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}
