use axum::{routing::get, Router};
use service::general;
use sqlx::MySqlPool;

pub type StateRouter = Router<MySqlPool>;

mod subscription;

pub fn compose() -> StateRouter {
    Router::new()
        .route("/", get(general::index))
        .route("/health", get(general::health))
        .merge(subscription::route())
}
