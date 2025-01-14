use axum::{
    extract::{rejection::FormRejection, State},
    http::StatusCode,
    Form,
};
use common::{
    dto::subscription::{MakeSubscription, Subscription, SubscriptionMeta},
    AppResponseResult, ResponseDetail, SuccessResponse,
};
use database::crud::subscription as db;
use sqlx::MySqlPool;

/// POST "/subscription"
/// Parse the `x-www-form-urlencoded` body data into type `MakeSubscription`
/// and insert it into the database.
pub async fn insert(
    State(pool): State<MySqlPool>,
    payload: Result<Form<MakeSubscription>, FormRejection>,
) -> AppResponseResult<Subscription> {
    let make_sub = payload?.0;
    let sub = Subscription::new(make_sub.name(), make_sub.email())?;
    db::create(&pool, &sub).await?;
    Ok((
        StatusCode::CREATED,
        SuccessResponse::with_data(StatusCode::CREATED, sub),
    ))
}

pub async fn list(
    State(pool): State<MySqlPool>,
) -> AppResponseResult<Vec<Subscription>, SubscriptionMeta> {
    let (meta, subs) = db::list(&pool).await?;
    Ok((
        StatusCode::OK,
        ResponseDetail::with_meta(StatusCode::OK, subs, meta),
    ))
}
