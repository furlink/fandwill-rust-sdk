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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum SearchMode {
    #[default]
    #[serde(alias = "Fts")]
    Fts,
    #[serde(alias = "Semantic")]
    Semantic,
}

const fn is_default_search_mode(value: &SearchMode) -> bool {
    matches!(value, SearchMode::Fts)
}

fn option_string_is_none_or_empty(value: &Option<String>) -> bool {
    value.as_deref().map(str::is_empty).unwrap_or(true)
}

/// Query parameters accepted by `GET /listings`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams, utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct ListingsQuery {
    #[serde(default = "default_page", skip_serializing_if = "is_default_page")]
    pub page: u32,

    #[serde(
        default = "default_page_size",
        skip_serializing_if = "is_default_page_size"
    )]
    pub page_size: u32,

    #[serde(default, skip_serializing_if = "is_default_search_mode")]
    #[cfg_attr(feature = "utoipa", param(inline))]
    pub mode: SearchMode,

    /// Full-text query; required in [`SearchMode::Fts`] mode.
    #[serde(default, skip_serializing_if = "option_string_is_none_or_empty")]
    pub by: Option<String>,

    /// Optional `ts_rank` floor for full-text search.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_relevance: Option<f32>,

    /// Natural-language query; required in [`SearchMode::Semantic`] mode.
    #[serde(default, skip_serializing_if = "option_string_is_none_or_empty")]
    pub query: Option<String>,

    /// Optional cosine-distance ceiling for semantic search.
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
            mode: SearchMode::default(),
            by: None,
            min_relevance: None,
            query: None,
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
    fn semantic_query_roundtrips_all_public_fields() {
        let query = ListingsQuery {
            page: 2,
            page_size: 10,
            mode: SearchMode::Semantic,
            by: None,
            min_relevance: None,
            query: Some("knowledge graph".into()),
            max_distance: Some(0.42),
        };

        let json = serde_json::to_value(&query).unwrap();
        assert_eq!(json["mode"], "semantic");
        assert_eq!(
            serde_json::from_value::<ListingsQuery>(json).unwrap(),
            query
        );
    }
}
