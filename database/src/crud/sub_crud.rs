use chrono::Local;
use common::{
    model::subscription::{Pagination, PaginationMeta, SubscriptionRequest, SubscriptionResponse},
    AppError, AppResult,
};
use sqlx::{query, query_as, MySqlPool};
use utils::snowflake::Generator;

/// 增
pub async fn create(pool: &MySqlPool, sub: &SubscriptionRequest) -> AppResult {
    let id = Generator::default().next_id()?;
    let subscribed_at = Local::now();
    let res = query!(
        "INSERT INTO subscription (id, email, name, subscribed_at) VALUES (?, ?, ?, ?)",
        id,
        sub.email(),
        sub.name(),
        subscribed_at,
    )
    .execute(pool)
    .await;

    match res {
        Ok(_) => Ok(()),
        Err(e) => {
            let db_error = e.as_database_error();
            match db_error {
                Some(db_e) => {
                    if db_e.is_unique_violation() {
                        Err(AppError::confict("邮件已订阅"))
                    } else {
                        Err(AppError::from(e))
                    }
                }
                None => Err(AppError::from(e)),
            }
        }
    }
}

// TODO 使用 Redis 缓存查询结果，减少数据库查询次数
/// 查
pub async fn pagination_list(
    pool: &MySqlPool,
    pagination: Pagination,
) -> AppResult<(PaginationMeta, Vec<SubscriptionResponse>)> {
    let page = pagination.page() as i64;
    let limit = pagination.per_page() as i64;
    let total = query!("SELECT COUNT(*) as count FROM subscription")
        .fetch_one(pool)
        .await?
        .count;
    let total_page = total / limit + 1;
    if page > total_page {
        return Err(AppError::not_found("Page not found"));
    }
    let off_set = (page - 1) * limit;
    let data = query_as!(
        SubscriptionResponse,
        "SELECT name, email FROM subscription LIMIT ? OFFSET ?",
        limit,
        off_set
    )
    .fetch_all(pool)
    .await?;
    let meta = PaginationMeta::new(
        total_page as usize,
        limit as usize,
        page as usize,
        data.len(),
    );
    Ok((meta, data))
}
