use serde::{Deserialize, Serialize};

pub const DEFAULT_PAGE: u32 = 1;
pub const DEFAULT_PAGE_SIZE: u32 = 20;
pub const MAX_PAGE_SIZE: u32 = 100;

pub(crate) const fn default_page() -> u32 {
    DEFAULT_PAGE
}

pub(crate) const fn default_page_size() -> u32 {
    DEFAULT_PAGE_SIZE
}

pub(crate) const fn is_default_page(value: &u32) -> bool {
    *value == DEFAULT_PAGE
}

pub(crate) const fn is_default_page_size(value: &u32) -> bool {
    *value == DEFAULT_PAGE_SIZE
}

/// Shared one-based page and page-size query parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams, utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct PaginationParams {
    /// Page number (1-based).
    #[serde(default = "default_page", skip_serializing_if = "is_default_page")]
    pub page: u32,

    /// Items per page, clamped to 1..=100 by [`Self::normalize`].
    #[serde(
        default = "default_page_size",
        skip_serializing_if = "is_default_page_size"
    )]
    pub page_size: u32,
}

impl PaginationParams {
    pub fn normalize(mut self) -> Self {
        if self.page == 0 {
            self.page = DEFAULT_PAGE;
        }
        self.page_size = self.page_size.clamp(1, MAX_PAGE_SIZE);
        self
    }

    pub const fn offset(&self) -> u64 {
        ((self.page - 1) as u64) * (self.page_size as u64)
    }

    pub const fn limit(&self) -> u64 {
        self.page_size as u64
    }
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: DEFAULT_PAGE,
            page_size: DEFAULT_PAGE_SIZE,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PageInfo {
    pub has_next: bool,
    pub total: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PagedResponse<T> {
    pub items: Vec<T>,
    pub page_info: PageInfo,
}

impl<T> PagedResponse<T> {
    pub fn new(items: Vec<T>, total: usize, params: &PaginationParams) -> Self {
        let has_next = (params.page as usize) * (params.page_size as usize) < total;

        Self {
            items,
            page_info: PageInfo { has_next, total },
        }
    }

    pub fn with_entire<I>(data: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let items = data.into_iter().collect::<Vec<_>>();
        let page_info = PageInfo {
            has_next: false,
            total: items.len(),
        };

        Self { items, page_info }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pagination_defaults_roundtrip_as_empty_query_object() {
        let params: PaginationParams = serde_json::from_value(serde_json::json!({})).unwrap();
        assert_eq!(params, PaginationParams::default());
        assert_eq!(serde_json::to_value(params).unwrap(), serde_json::json!({}));
    }

    #[test]
    fn normalization_matches_backend_limits() {
        let params = PaginationParams {
            page: 0,
            page_size: 500,
        }
        .normalize();

        assert_eq!(params.page, 1);
        assert_eq!(params.page_size, 100);
        assert_eq!(params.offset(), 0);
        assert_eq!(params.limit(), 100);
    }
}
