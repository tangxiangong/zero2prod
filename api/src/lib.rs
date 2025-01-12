use axum::routing::get;
use axum::Router;
use service::general;
use sqlx::MySqlPool;

pub type StateRouter = Router<MySqlPool>;

pub fn compose() -> StateRouter {
    Router::new()
        .route("/", get(general::index))
        .route("/health", get(general::health))
}
