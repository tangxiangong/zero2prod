use common::{
    model::{
        entity::{ActiveSubscription, Subscription},
        subscription_dto::{Pagination, PaginationMeta, SubscriptionRequest, SubscriptionResponse},
    },
    AppError, AppResult,
};
use sea_orm::{prelude::*, QuerySelect, Set};
use utils::snowflake::Generator;

// TODO 使用 Redis 缓存查询结果，减少数据库查询次数

pub async fn create(db_conn: &DbConn, sub: &SubscriptionRequest) -> AppResult {
    let id = Generator::default().next_id()?;
    let active_sub = ActiveSubscription {
        id: Set(id),
        name: Set(sub.name()),
        email: Set(sub.email()),
        ..Default::default()
    };
    let res = active_sub.insert(db_conn).await;
    if let Err(db_err) = res {
        let sql_err = db_err.sql_err();
        match sql_err {
            Some(err) => {
                if let SqlErr::UniqueConstraintViolation(_) = err {
                    Err(AppError::confict("邮件已订阅"))
                } else {
                    Err(AppError::from(db_err))
                }
            }
            None => Err(AppError::from(db_err)),
        }
    } else {
        Ok(())
    }
}

pub async fn pagination_list(
    db_conn: &DbConn,
    pagination: Pagination,
) -> AppResult<(PaginationMeta, Vec<SubscriptionResponse>)> {
    let page = pagination.page() as u64;
    let limit = pagination.per_page() as u64;
    let total = Subscription::find().count(db_conn).await?;

    let total_page = total / limit + 1;
    if page > total_page {
        return Err(AppError::not_found("Page not found"));
    }
    let off_set = (page - 1) * limit;
    let data = Subscription::find()
        .offset(off_set)
        .limit(limit)
        .into_partial_model::<SubscriptionResponse>()
        .all(db_conn)
        .await?;
    let meta = PaginationMeta::new(
        total_page as usize,
        limit as usize,
        page as usize,
        data.len(),
    );
    Ok((meta, data))
}
