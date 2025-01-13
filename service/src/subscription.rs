use axum::{
    extract::{rejection::FormRejection, State},
    http::StatusCode,
    Form,
};
use common::{
    dto::subscription::{MakeSubscription, Subscription, SubscriptionMeta},
    AppResponse, AppResult, ResponseDetail,
};
use database::crud::subscription as db;
use sqlx::MySqlPool;

/// make a new subscription
/// POST /subscription
/// Parse the x-www-form-urlencoded body into a `MakeSubscription` struct
pub async fn make(
    State(pool): State<MySqlPool>,
    payload: Result<Form<MakeSubscription>, FormRejection>,
) -> AppResult<AppResponse<Subscription>> {
    let make_sub = payload?.0;
    let sub = Subscription::from(make_sub);
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
