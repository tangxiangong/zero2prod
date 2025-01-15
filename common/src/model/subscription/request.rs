use crate::AppError;
use axum::{
    extract::{Form, FromRequest, Request},
    http::StatusCode,
};
use serde::Deserialize;
use utils::validator::{is_valid_email, is_valid_name};

#[derive(Debug, Deserialize)]
pub struct Pagination {
    page: i32,
    per_page: i32,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 10,
        }
    }
}

impl Pagination {
    pub fn page(&self) -> i32 {
        self.page
    }

    pub fn per_page(&self) -> i32 {
        self.per_page
    }
}

#[derive(Debug, Deserialize)]
pub struct SubscriptionRequest {
    name: String,
    email: String,
}

impl SubscriptionRequest {
    pub fn email(&self) -> String {
        self.email.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

// 让 MakeSubscription 可以当作一个提取器
impl<S> FromRequest<S> for SubscriptionRequest
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let sub = Form::<SubscriptionRequest>::from_request(req, _state)
            .await?
            .0;
        if !is_valid_name(&sub.name) {
            return Err(AppError::new(StatusCode::BAD_REQUEST, "名字中包含非法字符"));
        }
        if !is_valid_email(&sub.email) {
            return Err(AppError::new(StatusCode::BAD_REQUEST, "邮箱格式不正确"));
        }
        Ok(sub)
    }
}
