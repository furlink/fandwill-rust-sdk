use serde::{Deserialize, Serialize};

use crate::listings::ListingsVO;
use crate::reviews::{ReviewReplyVO, ReviewsVO};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ListingsVOWithValidation {
    #[serde(flatten)]
    pub inner: ListingsVO,
    pub markdown_validation: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ReviewsVOWithValidation {
    #[serde(flatten)]
    pub inner: ReviewsVO,
    pub markdown_validation: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ReviewReplyVOWithValidation {
    #[serde(flatten)]
    pub inner: ReviewReplyVO,
    pub markdown_validation: Vec<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn listing_validation_response_deserializes_from_flattened_json() {
        let response: ListingsVOWithValidation = serde_json::from_value(serde_json::json!({
            "id": "listing",
            "created_by": "user",
            "title": "Title",
            "description": "Description",
            "content": "Content",
            "banners": [],
            "created_at": "2026-08-04T00:00:00Z",
            "updated_at": "2026-08-04T00:00:00Z",
            "markdown_validation": [{"level": "warning"}]
        }))
        .unwrap();

        assert_eq!(response.inner.id, "listing");
        assert_eq!(response.markdown_validation.len(), 1);

        let roundtrip: ListingsVOWithValidation =
            serde_json::from_value(serde_json::to_value(&response).unwrap()).unwrap();
        assert_eq!(roundtrip.inner.id, response.inner.id);
        assert_eq!(roundtrip.markdown_validation, response.markdown_validation);
    }

    #[test]
    fn review_validation_responses_deserialize_from_flattened_json() {
        let review: ReviewsVOWithValidation = serde_json::from_value(serde_json::json!({
            "id": "review",
            "created_by": "user",
            "listing_id": "listing",
            "content": "Content",
            "like_count": 0,
            "viewer_liked": false,
            "created_at": "2026-08-04T00:00:00Z",
            "markdown_validation": []
        }))
        .unwrap();
        assert_eq!(review.inner.id, "review");
        let review_roundtrip: ReviewsVOWithValidation =
            serde_json::from_value(serde_json::to_value(&review).unwrap()).unwrap();
        assert_eq!(review_roundtrip.inner.id, review.inner.id);

        let reply: ReviewReplyVOWithValidation = serde_json::from_value(serde_json::json!({
            "id": "reply",
            "review_id": "review",
            "parent_reply_id": null,
            "created_by": "user",
            "content": "Content",
            "created_at": "2026-08-04T00:00:00Z",
            "markdown_validation": []
        }))
        .unwrap();
        assert_eq!(reply.inner.id, "reply");
        let reply_roundtrip: ReviewReplyVOWithValidation =
            serde_json::from_value(serde_json::to_value(&reply).unwrap()).unwrap();
        assert_eq!(reply_roundtrip.inner.id, reply.inner.id);
    }
}
