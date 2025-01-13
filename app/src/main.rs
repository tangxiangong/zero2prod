use sqlx::mysql::MySqlPoolOptions;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to MySQL");

    // migrate!("../migrations")
    //     .run(&pool)
    //     .await
    //     .expect("Failed to migrate database");

    let app = api::compose()
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    info!(
        "server listener on http://{}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
