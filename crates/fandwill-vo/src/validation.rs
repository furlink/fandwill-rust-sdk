use serde::Serialize;

use crate::listings::ListingsVO;
use crate::reviews::{ReviewReplyVO, ReviewsVO};

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ListingsVOWithValidation {
    #[serde(flatten)]
    pub inner: ListingsVO,
    pub markdown_validation: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ReviewsVOWithValidation {
    #[serde(flatten)]
    pub inner: ReviewsVO,
    pub markdown_validation: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ReviewReplyVOWithValidation {
    #[serde(flatten)]
    pub inner: ReviewReplyVO,
    pub markdown_validation: Vec<serde_json::Value>,
}
