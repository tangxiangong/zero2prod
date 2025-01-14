use axum::{
    extract::{rejection::FormRejection, State},
    http::StatusCode,
    Form,
};
use common::{
    sub_dto::{GetSubscription, MakeSubscription, SubscriptionMeta},
    AppResponseResult, ResponseDetail, SuccessResponse,
};
use database::sub_crud as crud;
use sqlx::MySqlPool;

/// POST "/subscription"
/// Parse the `x-www-form-urlencoded` body data into type `MakeSubscription`
/// and insert it into the database.
pub async fn insert(
    State(pool): State<MySqlPool>,
    payload: Result<Form<MakeSubscription>, FormRejection>,
) -> AppResponseResult {
    let make_sub = payload?.0;
    crud::create(&pool, &make_sub).await?;
    Ok((
        StatusCode::CREATED,
        SuccessResponse::new(StatusCode::CREATED),
    ))
}

pub async fn list(
    State(pool): State<MySqlPool>,
) -> AppResponseResult<Vec<GetSubscription>, SubscriptionMeta> {
    let (meta, subs) = crud::list(&pool).await?;
    Ok((
        StatusCode::OK,
        ResponseDetail::with_meta(StatusCode::OK, subs, meta),
    ))
}
