use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    pagination::{
        PaginationParams, default_page, default_page_size, is_default_page, is_default_page_size,
    },
    resources::ResourceVO,
};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateListingVO {
    pub title: String,
    pub description: String,
    pub content: String,
    pub resources: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UpdateListingVO {
    pub title: String,
    pub description: String,
    pub content: String,
    pub resources: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UpdateListingVersionStatusVO {
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ListingsVO {
    pub id: String,
    pub created_by: Option<String>,
    pub title: String,
    pub description: String,
    pub content: String,
    pub banners: Vec<ResourceVO>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ListingVersionVO {
    pub id: String,
    pub title: String,
    pub description: String,
    pub content: String,
    pub changed_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub status: String,
}

/// The authenticated user's bookmark for one listing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct BookmarkVO {
    pub listing_id: String,
    pub created_at: DateTime<Utc>,
}

/// Search mode for `GET /listings`; selects how `q` is interpreted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum SearchMode {
    /// Keyword full-text search; `min_relevance` applies.
    #[serde(alias = "Fts")]
    Fts,
    /// Natural-language semantic search; `max_distance` applies.
    #[serde(alias = "Semantic")]
    Semantic,
}

/// Query parameters accepted by `GET /listings`.
///
/// Without `mode`, the endpoint browses all published listings; `q`, `min_relevance` and
/// `max_distance` must then be omitted. With `mode`, `q` is required and only the threshold
/// of the selected mode may be supplied. Invalid combinations are rejected with 400.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams, utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct ListingsQuery {
    /// Page number (1-based).
    #[serde(default = "default_page", skip_serializing_if = "is_default_page")]
    pub page: u32,

    /// Items per page, clamped to 1..=100.
    #[serde(
        default = "default_page_size",
        skip_serializing_if = "is_default_page_size"
    )]
    pub page_size: u32,

    /// Search mode. Omit to browse all published listings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "utoipa", param(inline))]
    pub mode: Option<SearchMode>,

    /// Search term: keywords in `fts` mode, natural language in `semantic` mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,

    /// Optional `ts_rank` floor; only valid with `mode=fts`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_relevance: Option<f32>,

    /// Optional cosine-distance ceiling; only valid with `mode=semantic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_distance: Option<f32>,
}

impl ListingsQuery {
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

impl Default for ListingsQuery {
    fn default() -> Self {
        Self {
            page: default_page(),
            page_size: default_page_size(),
            mode: None,
            q: None,
            min_relevance: None,
            max_distance: None,
        }
    }
}

#[cfg(test)]
mod query_tests {
    use super::*;

    #[test]
    fn query_defaults_serialize_to_an_empty_object() {
        let query: ListingsQuery = serde_json::from_value(serde_json::json!({})).unwrap();
        assert_eq!(query, ListingsQuery::default());
        assert_eq!(serde_json::to_value(query).unwrap(), serde_json::json!({}));
    }

    #[test]
    fn search_mode_accepts_canonical_and_legacy_spellings() {
        for value in ["fts", "Fts"] {
            let mode: SearchMode = serde_json::from_value(serde_json::json!(value)).unwrap();
            assert_eq!(mode, SearchMode::Fts);
        }
        for value in ["semantic", "Semantic"] {
            let mode: SearchMode = serde_json::from_value(serde_json::json!(value)).unwrap();
            assert_eq!(mode, SearchMode::Semantic);
        }
    }

    #[test]
    fn browse_and_mode_queries_roundtrip_all_public_fields() {
        // Browse mode: an absent `mode` serializes as an omitted field.
        let browse = ListingsQuery {
            page: 2,
            page_size: 10,
            mode: None,
            q: None,
            min_relevance: None,
            max_distance: None,
        };
        let json = serde_json::to_value(&browse).unwrap();
        assert!(json.get("mode").is_none());
        assert_eq!(
            serde_json::from_value::<ListingsQuery>(json).unwrap(),
            browse
        );

        // Semantic search roundtrips `q` and `max_distance`.
        let semantic = ListingsQuery {
            page: 1,
            page_size: 20,
            mode: Some(SearchMode::Semantic),
            q: Some("knowledge graph".into()),
            min_relevance: None,
            max_distance: Some(0.42),
        };
        let json = serde_json::to_value(&semantic).unwrap();
        assert_eq!(json["mode"], "semantic");
        assert_eq!(
            serde_json::from_value::<ListingsQuery>(json).unwrap(),
            semantic
        );
    }

    #[test]
    fn bookmark_roundtrips_json() {
        let bookmark = BookmarkVO {
            listing_id: "listing".into(),
            created_at: DateTime::parse_from_rfc3339("2026-08-04T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
        };
        let value = serde_json::to_value(&bookmark).unwrap();
        assert_eq!(
            serde_json::from_value::<BookmarkVO>(value).unwrap(),
            bookmark
        );
    }
}
