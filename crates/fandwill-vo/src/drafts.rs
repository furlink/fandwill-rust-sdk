use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// One saved listing draft owned by an authenticated author.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ListingDraftVO {
    pub id: String,
    /// Owning listing id; `null` means the draft starts a new listing on submit.
    pub listing_id: Option<String>,
    pub title: String,
    pub description: String,
    pub content: String,
    /// Referenced banner resource ids in submission order.
    pub resources: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request to create a draft. Only `title` is required; other fields may be filled in later.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateListingDraftVO {
    /// Required even for unfinished drafts; trimmed, non-blank, and at most 120 characters.
    pub title: String,
    /// Existing listing to edit on submit; omit for a new listing draft.
    pub listing_id: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub resources: Option<Vec<String>>,
}

/// Request to update a draft. Omitted or `null` fields remain unchanged.
///
/// `listing_id` is immutable and therefore cannot be changed here. Send an empty string or an
/// empty resource list to clear those values.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UpdateListingDraftVO {
    /// When supplied, must be trimmed, non-blank, and at most 120 characters.
    pub title: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub resources: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_draft_accepts_partial_content() {
        let request: CreateListingDraftVO = serde_json::from_value(serde_json::json!({
            "title": "Work in progress",
            "listing_id": null
        }))
        .unwrap();

        assert_eq!(request.title, "Work in progress");
        assert!(request.listing_id.is_none());
        assert!(request.description.is_none());
        assert!(request.content.is_none());
        assert!(request.resources.is_none());
    }

    #[test]
    fn update_draft_treats_omitted_and_null_fields_as_unchanged() {
        let omitted: UpdateListingDraftVO = serde_json::from_value(serde_json::json!({})).unwrap();
        let nulls: UpdateListingDraftVO = serde_json::from_value(serde_json::json!({
            "title": null,
            "description": null,
            "content": null,
            "resources": null
        }))
        .unwrap();

        assert!(omitted.title.is_none());
        assert!(omitted.description.is_none());
        assert!(omitted.content.is_none());
        assert!(omitted.resources.is_none());
        assert!(nulls.title.is_none());
        assert!(nulls.description.is_none());
        assert!(nulls.content.is_none());
        assert!(nulls.resources.is_none());

        let clear_values: UpdateListingDraftVO = serde_json::from_value(serde_json::json!({
            "description": "",
            "resources": []
        }))
        .unwrap();
        assert_eq!(clear_values.description.as_deref(), Some(""));
        assert_eq!(clear_values.resources.as_deref(), Some([].as_slice()));
    }
}
