use common::{
    dto::subscription::{Subscription, SubscriptionMeta},
    AppResult,
};
use sqlx::{query, query_as, MySqlPool};

pub async fn create(pool: &MySqlPool, subscription: &Subscription) -> AppResult<()> {
    query!(
        r#"INSERT INTO `subscription` (`id`, `email`, `name`, `subscribed_at`) VALUES (?, ?, ?, ?)"#,
        subscription.id(),
        subscription.email(),
        subscription.name(),
        subscription.subscribed_at()
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn list(pool: &MySqlPool) -> AppResult<(SubscriptionMeta, Vec<Subscription>)> {
    let subscriptions: Vec<Subscription> = query_as("SELECT * FROM `subscription`")
        .fetch_all(pool)
        .await?;
    let meta = SubscriptionMeta::new(subscriptions.len());
    Ok((meta, subscriptions))
}
