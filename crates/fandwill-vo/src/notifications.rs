use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::pagination::{
    PaginationParams, default_page, default_page_size, is_default_page, is_default_page_size,
};

fn is_false(value: &bool) -> bool {
    !*value
}

/// Query parameters accepted by `GET /users/me/notifications`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams, utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct NotificationsQuery {
    #[serde(default = "default_page", skip_serializing_if = "is_default_page")]
    pub page: u32,

    #[serde(
        default = "default_page_size",
        skip_serializing_if = "is_default_page_size"
    )]
    pub page_size: u32,

    /// Return only notifications that have not been read.
    #[serde(default, skip_serializing_if = "is_false")]
    pub unread_only: bool,

    /// Incremental polling cursor. When present, the server ignores `page`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after_id: Option<String>,
}

impl NotificationsQuery {
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

impl Default for NotificationsQuery {
    fn default() -> Self {
        Self {
            page: default_page(),
            page_size: default_page_size(),
            unread_only: false,
            after_id: None,
        }
    }
}

/// An action a system notification can ask a client to open.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "kind", content = "data", rename_all = "snake_case")]
pub enum NotificationActionVO {
    OpenListing {
        listing_id: String,
    },
    OpenReview {
        listing_id: String,
        review_id: String,
    },
    OpenReply {
        listing_id: String,
        review_id: String,
        reply_id: String,
    },
}

/// Type-specific notification payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "kind", content = "data", rename_all = "snake_case")]
pub enum NotificationPayloadVO {
    ListingPublished {
        listing_id: String,
        listing_version_id: String,
    },
    ListingArchived {
        listing_id: String,
        listing_version_id: String,
    },
    ListingPendingReview {
        listing_id: String,
        listing_version_id: String,
    },
    ReviewCreated {
        listing_id: String,
        review_id: String,
    },
    ReviewReplied {
        listing_id: String,
        review_id: String,
        reply_id: String,
    },
    ReviewLiked {
        listing_id: String,
        review_id: String,
    },
    System {
        title: String,
        body: String,
        action: Option<NotificationActionVO>,
    },
}

/// A notification returned to its authenticated recipient.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NotificationVO {
    pub id: String,
    pub actor_id: Option<String>,
    pub payload: NotificationPayloadVO,
    pub created_at: DateTime<Utc>,
    pub read_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NotificationSummaryVO {
    pub unread_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MarkAllNotificationsReadVO {
    pub updated_count: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_payload_roundtrip(
        payload: NotificationPayloadVO,
        kind: &str,
        data: serde_json::Value,
    ) {
        let value = serde_json::to_value(&payload).unwrap();
        assert_eq!(value, serde_json::json!({ "kind": kind, "data": data }));
        assert_eq!(
            serde_json::from_value::<NotificationPayloadVO>(value).unwrap(),
            payload
        );
    }

    #[test]
    fn every_notification_payload_matches_the_wire_contract() {
        for (payload, kind, data) in [
            (
                NotificationPayloadVO::ListingPublished {
                    listing_id: "listing".into(),
                    listing_version_id: "version".into(),
                },
                "listing_published",
                serde_json::json!({ "listing_id": "listing", "listing_version_id": "version" }),
            ),
            (
                NotificationPayloadVO::ListingArchived {
                    listing_id: "listing".into(),
                    listing_version_id: "version".into(),
                },
                "listing_archived",
                serde_json::json!({ "listing_id": "listing", "listing_version_id": "version" }),
            ),
            (
                NotificationPayloadVO::ListingPendingReview {
                    listing_id: "listing".into(),
                    listing_version_id: "version".into(),
                },
                "listing_pending_review",
                serde_json::json!({ "listing_id": "listing", "listing_version_id": "version" }),
            ),
            (
                NotificationPayloadVO::ReviewCreated {
                    listing_id: "listing".into(),
                    review_id: "review".into(),
                },
                "review_created",
                serde_json::json!({ "listing_id": "listing", "review_id": "review" }),
            ),
            (
                NotificationPayloadVO::ReviewReplied {
                    listing_id: "listing".into(),
                    review_id: "review".into(),
                    reply_id: "reply".into(),
                },
                "review_replied",
                serde_json::json!({
                    "listing_id": "listing",
                    "review_id": "review",
                    "reply_id": "reply"
                }),
            ),
            (
                NotificationPayloadVO::ReviewLiked {
                    listing_id: "listing".into(),
                    review_id: "review".into(),
                },
                "review_liked",
                serde_json::json!({ "listing_id": "listing", "review_id": "review" }),
            ),
            (
                NotificationPayloadVO::System {
                    title: "Maintenance".into(),
                    body: "Scheduled".into(),
                    action: None,
                },
                "system",
                serde_json::json!({ "title": "Maintenance", "body": "Scheduled", "action": null }),
            ),
            (
                NotificationPayloadVO::System {
                    title: "Reply".into(),
                    body: "Open it".into(),
                    action: Some(NotificationActionVO::OpenReply {
                        listing_id: "listing".into(),
                        review_id: "review".into(),
                        reply_id: "reply".into(),
                    }),
                },
                "system",
                serde_json::json!({
                    "title": "Reply",
                    "body": "Open it",
                    "action": {
                        "kind": "open_reply",
                        "data": {
                            "listing_id": "listing",
                            "review_id": "review",
                            "reply_id": "reply"
                        }
                    }
                }),
            ),
        ] {
            assert_payload_roundtrip(payload, kind, data);
        }
    }

    #[test]
    fn every_notification_action_matches_the_wire_contract() {
        for (action, expected) in [
            (
                NotificationActionVO::OpenListing {
                    listing_id: "listing".into(),
                },
                serde_json::json!({
                    "kind": "open_listing",
                    "data": { "listing_id": "listing" }
                }),
            ),
            (
                NotificationActionVO::OpenReview {
                    listing_id: "listing".into(),
                    review_id: "review".into(),
                },
                serde_json::json!({
                    "kind": "open_review",
                    "data": { "listing_id": "listing", "review_id": "review" }
                }),
            ),
            (
                NotificationActionVO::OpenReply {
                    listing_id: "listing".into(),
                    review_id: "review".into(),
                    reply_id: "reply".into(),
                },
                serde_json::json!({
                    "kind": "open_reply",
                    "data": {
                        "listing_id": "listing",
                        "review_id": "review",
                        "reply_id": "reply"
                    }
                }),
            ),
        ] {
            let value = serde_json::to_value(&action).unwrap();
            assert_eq!(value, expected);
            assert_eq!(
                serde_json::from_value::<NotificationActionVO>(value).unwrap(),
                action
            );
        }
    }

    #[test]
    fn notifications_query_defaults_and_cursor_roundtrip() {
        let default: NotificationsQuery = serde_json::from_value(serde_json::json!({})).unwrap();
        assert_eq!(default, NotificationsQuery::default());
        assert_eq!(
            serde_json::to_value(default).unwrap(),
            serde_json::json!({})
        );

        let query = NotificationsQuery {
            page: 3,
            page_size: 50,
            unread_only: true,
            after_id: Some("watermark".into()),
        };
        let value = serde_json::to_value(&query).unwrap();
        assert_eq!(
            serde_json::from_value::<NotificationsQuery>(value).unwrap(),
            query
        );
    }
}
