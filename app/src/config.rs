use sqlx::mysql::MySqlPoolOptions;
use tokio::net::TcpListener;

pub async fn config() -> (sqlx::MySqlPool, TcpListener) {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");

    let port = std::env::var("PORT").unwrap_or("3000".to_string());

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to MySQL");

    // migrate!("../migrations")
    //     .run(&pool)
    //     .await
    //     .expect("Failed to migrate database");

    (pool, listener)
}
