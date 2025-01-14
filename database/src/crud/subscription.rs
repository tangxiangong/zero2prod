use common::{
    dto::subscription::{Subscription, SubscriptionMeta},
    AppResult,
};
use sqlx::{query, query_as, MySqlPool};

/// 增
pub async fn create(pool: &MySqlPool, sub: &Subscription) -> AppResult {
    query!(
        "INSERT INTO subscription (id, email, name, subscribed_at) VALUES (?, ?, ?, ?)",
        sub.id,
        sub.email,
        sub.name,
        sub.subscribed_at,
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// 查
pub async fn list(pool: &MySqlPool) -> AppResult<(SubscriptionMeta, Vec<Subscription>)> {
    let subscriptions = query_as!(Subscription, "SELECT * FROM subscription")
        .fetch_all(pool)
        .await?;
    let meta = SubscriptionMeta::new(subscriptions.len());
    Ok((meta, subscriptions))
}
