use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::pagination::{
    PaginationParams, default_page, default_page_size, is_default_page, is_default_page_size,
};

#[cfg(feature = "garde")]
fn content_not_blank(value: &str, _: &()) -> garde::Result {
    if value.trim().is_empty() {
        Err(garde::Error::new("content must not be blank"))
    } else {
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "garde", derive(garde::Validate))]
pub struct CreateReviewVO {
    #[cfg_attr(feature = "garde", garde(skip))]
    pub listing_id: String,
    #[cfg_attr(feature = "garde", garde(custom(content_not_blank)))]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "garde", derive(garde::Validate))]
pub struct CreateReplyVO {
    #[cfg_attr(feature = "garde", garde(custom(content_not_blank)))]
    pub content: String,
    #[cfg_attr(feature = "garde", garde(skip))]
    pub parent_reply_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ReviewReplyVO {
    pub id: String,
    pub review_id: String,
    pub parent_reply_id: Option<String>,
    pub created_by: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ReviewsVO {
    pub id: String,
    pub created_by: String,
    pub listing_id: String,
    pub content: String,
}

fn option_string_is_none_or_empty(value: &Option<String>) -> bool {
    value.as_deref().map(str::is_empty).unwrap_or(true)
}

/// Query parameters accepted by `GET /reviews`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams, utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct ReviewFilter {
    /// Optional listing Sqid used to limit returned reviews.
    #[serde(default, skip_serializing_if = "option_string_is_none_or_empty")]
    pub listing_id: Option<String>,

    #[serde(default = "default_page", skip_serializing_if = "is_default_page")]
    pub page: u32,

    #[serde(
        default = "default_page_size",
        skip_serializing_if = "is_default_page_size"
    )]
    pub page_size: u32,
}

impl ReviewFilter {
    pub fn normalize(mut self) -> Self {
        let pagination = self.pagination().normalize();
        self.page = pagination.page;
        self.page_size = pagination.page_size;
        self
    }

    pub const fn pagination(&self) -> PaginationParams {
        PaginationParams {
            page: self.page,
            page_size: self.page_size,
        }
    }
}

impl Default for ReviewFilter {
    fn default() -> Self {
        Self {
            listing_id: None,
            page: default_page(),
            page_size: default_page_size(),
        }
    }
}

#[cfg(all(test, feature = "garde"))]
mod validation_tests {
    use garde::Validate;

    use super::*;

    #[test]
    fn create_review_rejects_blank_content() {
        for content in ["", "   ", "\t\n "] {
            let vo = CreateReviewVO {
                listing_id: "listing".into(),
                content: content.into(),
            };
            assert!(vo.validate().is_err(), "expected rejection of {content:?}");
        }
    }

    #[test]
    fn create_reply_rejects_blank_content() {
        for content in ["", "   ", "\t\n "] {
            let vo = CreateReplyVO {
                content: content.into(),
                parent_reply_id: None,
            };
            assert!(vo.validate().is_err(), "expected rejection of {content:?}");
        }
    }

    #[test]
    fn create_requests_accept_non_blank_content() {
        let review = CreateReviewVO {
            listing_id: "listing".into(),
            content: "  looks great  ".into(),
        };
        assert!(review.validate().is_ok());

        let reply = CreateReplyVO {
            content: "thanks".into(),
            parent_reply_id: Some("reply".into()),
        };
        assert!(reply.validate().is_ok());
    }
}

#[cfg(test)]
mod filter_tests {
    use super::*;

    #[test]
    fn review_filter_defaults_and_roundtrips() {
        let default: ReviewFilter = serde_json::from_value(serde_json::json!({})).unwrap();
        assert_eq!(default, ReviewFilter::default());
        assert_eq!(
            serde_json::to_value(default).unwrap(),
            serde_json::json!({})
        );

        let filter = ReviewFilter {
            listing_id: Some("listing".into()),
            page: 2,
            page_size: 50,
        };
        let json = serde_json::to_value(&filter).unwrap();
        assert_eq!(
            serde_json::from_value::<ReviewFilter>(json).unwrap(),
            filter
        );
    }
}
