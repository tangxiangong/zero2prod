use chrono::NaiveDateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Subsription {
    id: u64,
    email: String,
    name: String,
    subscribed_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateSubsription {
    pub email: String,
    pub name: String,
}

impl CreateSubsription {
    pub fn new(email: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            email: email.into(),
            name: name.into(),
        }
    }
}

impl From<CreateSubsription> for Subsription {
    fn from(subsription: CreateSubsription) -> Self {
        Self {
            id: 0,
            email: subsription.email,
            name: subsription.name,
            subscribed_at: Utc::now().naive_utc(),
        }
    }
}
