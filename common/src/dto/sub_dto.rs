use crate::Meta;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SubscriptionMeta {
    count: usize,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct GetSubscription {
    pub name: String,
    pub email: String,
}

impl SubscriptionMeta {
    pub fn new(count: usize) -> Self {
        Self { count }
    }
}

impl Meta for SubscriptionMeta {}

#[derive(Debug, Deserialize)]
pub struct MakeSubscription {
    name: String,
    email: String,
}

impl MakeSubscription {
    pub fn email(&self) -> String {
        self.email.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}
