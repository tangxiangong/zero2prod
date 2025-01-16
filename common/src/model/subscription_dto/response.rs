use sea_orm::{DerivePartialModel, FromQueryResult};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SubscriptionResponseMeta {
    count: usize,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    total_page: usize,
    limit: usize,
    current_page: usize,
    current_page_size: usize,
}

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

#[derive(Debug, Serialize, DerivePartialModel, FromQueryResult)]
#[sea_orm(entity = "crate::model::entity::Subscription")]
pub struct SubscriptionResponse {
    pub name: String,
    pub email: String,
}

impl SubscriptionResponseMeta {
    pub fn new(count: usize) -> Self {
        Self { count }
    }
}
