use chrono::{DateTime, Utc};
use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateReviewVO {
    #[garde(skip)]
    pub of_listing: String,
    #[garde(range(min = 0, max = 100))]
    pub rating: i32,
    #[garde(skip)]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateReplyVO {
    pub content: String,
    pub parent_id: Option<String>,
}

#[derive(Debug, ToSchema, Serialize, Deserialize, Clone)]
pub struct ReviewReplyVO {
    pub id: String,
    pub review_id: String,
    pub parent_id: Option<String>,
    pub created_by: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, ToSchema, Serialize, Deserialize, Clone)]
pub struct ReviewsVO {
    pub id: String,
    pub created_by: String,
    pub of_listing: String,
    pub content: String,
    pub rating: i32,
}
