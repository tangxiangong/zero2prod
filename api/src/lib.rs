use axum::http::Method;
use axum::{routing::get, Router};
use sea_orm::DbConn;
use service::handler::general;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

pub type StateRouter = Router<DbConn>;

mod sub_api;

pub fn compose() -> StateRouter {
    Router::new()
        .route("/", get(general::index))
        .route("/health", get(general::health))
        .merge(sub_api::route())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(
                    CorsLayer::new()
                        // allow `GET` and `POST` when accessing the resource
                        .allow_methods([Method::GET, Method::POST])
                        // allow requests from any origin
                        .allow_origin(Any),
                )
                .layer(TimeoutLayer::new(Duration::from_secs(10))),
        )
}
