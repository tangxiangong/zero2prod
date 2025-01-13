use chrono::NaiveDateTime;

#[derive(Debug, sqlx::FromRow)]
pub struct Subscription {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub subscribed_at: NaiveDateTime,
}