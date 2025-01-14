use chrono::{DateTime, Local};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Subscription {
    pub id: u64,
    pub email: String,
    pub name: String,
    pub subscribed_at: DateTime<Local>,
}
