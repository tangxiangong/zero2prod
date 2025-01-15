use crate::{AppError, Meta};
use axum::{
    extract::{Form, FromRequest, Request},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use utils::validator::{is_valid_email, is_valid_name};

#[derive(Debug, Serialize)]
pub struct SubscriptionMeta {
    count: usize,
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    page: i32,
    per_page: i32,
}

impl Default for PaginationQuery {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 10,
        }
    }
}

impl PaginationQuery {
    pub fn page(&self) -> i32 {
        self.page
    }

    pub fn per_page(&self) -> i32 {
        self.per_page
    }
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    total_page: usize,
    limit: usize,
    current_page: usize,
    current_page_size: usize,
}

impl Meta for PaginationMeta {}

impl PaginationMeta {
    pub fn new(
        total_page: usize,
        limit: usize,
        current_page: usize,
        current_page_size: usize,
    ) -> Self {
        Self {
            total_page,
            limit,
            current_page,
            current_page_size,
        }
    }
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

// 让 MakeSubscription 可以当作一个提取器
impl<S> FromRequest<S> for MakeSubscription
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let sub = Form::<MakeSubscription>::from_request(req, _state).await?.0;
        if !is_valid_name(&sub.name) {
            return Err(AppError::new(StatusCode::BAD_REQUEST, "名字中包含非法字符"));
        }
        if !is_valid_email(&sub.email) {
            return Err(AppError::new(StatusCode::BAD_REQUEST, "邮箱格式不正确"));
        }
        Ok(sub)
    }
}
