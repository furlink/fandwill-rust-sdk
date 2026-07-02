use serde::Serialize;

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchMode {
    #[default]
    Fts,
    Semantic,
}

#[derive(Debug, Default, Serialize)]
pub struct ListingsQuery<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<SearchMode>,
    /// Full-text search query (required when mode=Fts)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by: Option<&'a str>,
    /// Semantic search query (required when mode=Semantic)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<&'a str>,
}
