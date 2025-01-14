use chrono::Local;
use common::{
    sub_dto::{GetSubscription, MakeSubscription, SubscriptionMeta},
    AppResult,
};
use sqlx::{query, query_as, MySqlPool};
use utils::snowflake::Generator;

/// 增
pub async fn create(pool: &MySqlPool, sub: &MakeSubscription) -> AppResult {
    let id = Generator::default().next_id()?;
    let subscribed_at = Local::now();
    query!(
        "INSERT INTO subscription (id, email, name, subscribed_at) VALUES (?, ?, ?, ?)",
        id,
        sub.email(),
        sub.name(),
        subscribed_at,
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// 查
pub async fn list(pool: &MySqlPool) -> AppResult<(SubscriptionMeta, Vec<GetSubscription>)> {
    let subscriptions = query_as!(GetSubscription, "SELECT name, email  FROM subscription")
        .fetch_all(pool)
        .await?;
    let meta = SubscriptionMeta::new(subscriptions.len());
    Ok((meta, subscriptions))
}
