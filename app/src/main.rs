use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main]
async fn main() {
    let (pool, listener) = setting::get().await;

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
