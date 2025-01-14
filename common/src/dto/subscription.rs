use crate::{AppResult, Meta};
use chrono::{DateTime, Local};
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;
use utils::snowflake::Generator;

#[derive(Debug, Serialize, FromRow)]
pub struct Subscription {
    pub id: u64,
    pub email: String,
    pub name: String,
    pub subscribed_at: DateTime<Local>,
}

impl Subscription {
    pub fn new(name: String, email: String) -> AppResult<Self> {
        let id = Generator::default().next_id()?;
        let subscribed_at = Local::now();
        Ok(Self {
            id,
            email,
            name,
            subscribed_at,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct SubscriptionMeta {
    count: usize,
}

impl SubscriptionMeta {
    pub fn new(count: usize) -> Self {
        Self { count }
    }
}

impl Meta for SubscriptionMeta {
    type Item = Subscription;
}

#[derive(Debug, Deserialize)]
pub struct MakeSubscription {
    name: String,
    email: String,
}

impl MakeSubscription {
    pub fn new(email: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            email: email.into(),
            name: name.into(),
        }
    }

    pub fn email(&self) -> String {
        self.email.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}
