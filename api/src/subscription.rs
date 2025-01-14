use super::StateRouter;
use axum::{
    routing::{get, post},
    Router,
};
use service::subscription as handler;

pub fn route() -> StateRouter {
    Router::new()
        .route("/subscription", get(handler::list))
        .route("/subscription", post(handler::insert))
}
