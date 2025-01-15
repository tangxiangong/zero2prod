use axum::{
    extract::{rejection::QueryRejection, Query, State},
    http::StatusCode,
};
use common::{
    model::subscription::{Pagination, PaginationMeta, SubscriptionRequest, SubscriptionResponse},
    AppResponseResult, ResponseDetail, SuccessResponse,
};
use database::sub_crud as crud;
use sqlx::MySqlPool;

/// POST "/subscription"
/// Parse the `x-www-form-urlencoded` body data into type `MakeSubscription`
/// and insert it into the database.
pub async fn make_sub(
    State(pool): State<MySqlPool>,
    new_sub: SubscriptionRequest,
) -> AppResponseResult {
    crud::create(&pool, &new_sub).await?;
    Ok((
        StatusCode::CREATED,
        SuccessResponse::new(StatusCode::CREATED),
    ))
}

pub async fn pagination_list(
    State(pool): State<MySqlPool>,
    payload: Result<Query<Pagination>, QueryRejection>,
) -> AppResponseResult<Vec<SubscriptionResponse>, PaginationMeta> {
    let pagination = payload?.0;
    let (meta, data) = crud::pagination_list(&pool, pagination).await?;

    Ok((
        StatusCode::OK,
        ResponseDetail::with_meta(StatusCode::OK, data, meta),
    ))
}
