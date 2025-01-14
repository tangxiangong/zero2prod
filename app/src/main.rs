use tracing::info;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let (pool, listener) = setting::get().await;

    tracing_subscriber::fmt::init();

    let app = api::compose().with_state(pool);
    info!(
        "server listener on http://{}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}
