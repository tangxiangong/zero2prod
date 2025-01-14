use crate::meta::Meta;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, FromRow)]
pub struct Subscription {
    pub id: u64,
    pub email: String,
    pub name: String,
    pub subscribed_at: DateTime<Local>,
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

impl Subscription {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn email(&self) -> String {
        self.email.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn subscribed_at(&self) -> DateTime<Local> {
        self.subscribed_at
    }
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

impl From<MakeSubscription> for Subscription {
    fn from(make_sub: MakeSubscription) -> Self {
        let id = Uuid::new_v4().as_u128() as u64;
        Self {
            id,
            email: make_sub.email,
            name: make_sub.name,
            subscribed_at: Local::now(),
        }
    }
}
