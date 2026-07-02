use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "garde", derive(garde::Validate))]
pub struct CreateReviewVO {
    #[cfg_attr(feature = "garde", garde(skip))]
    pub of_listing: String,
    #[cfg_attr(feature = "garde", garde(range(min = 0, max = 100)))]
    pub rating: i32,
    #[cfg_attr(feature = "garde", garde(skip))]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateReplyVO {
    pub content: String,
    pub parent_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ReviewReplyVO {
    pub id: String,
    pub review_id: String,
    pub parent_id: Option<String>,
    pub created_by: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ReviewsVO {
    pub id: String,
    pub created_by: String,
    pub of_listing: String,
    pub content: String,
    pub rating: i32,
}
