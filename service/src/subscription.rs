use axum::{extract::State, http::StatusCode, Json};
use common::{
    dto::subscription::{CreateSubscription, Subscription, SubscriptionMeta},
    AppResponse, AppResult, ResponseDetail,
};
use database::crud::subscription as db;
use sqlx::MySqlPool;

pub async fn create(
    State(pool): State<MySqlPool>,
    Json(create_sub): Json<CreateSubscription>,
) -> AppResult<AppResponse<Subscription>> {
    let sub = Subscription::from(create_sub);
    db::create(&pool, &sub).await?;
    Ok((
        StatusCode::CREATED,
        ResponseDetail::success(StatusCode::CREATED, sub),
    ))
}

pub async fn list(
    State(pool): State<MySqlPool>,
) -> AppResult<AppResponse<Vec<Subscription>, SubscriptionMeta>> {
    let (meta, subs) = db::list(&pool).await?;
    Ok((
        StatusCode::FOUND,
        ResponseDetail::with_meta(StatusCode::FOUND, subs, meta),
    ))
}
