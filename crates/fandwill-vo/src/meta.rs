use chrono::{DateTime, FixedOffset};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, ToSchema, Serialize)]
pub struct RootResponse {
    pub start_at: DateTime<FixedOffset>,
    pub version: String,
}

impl RootResponse {
    pub fn new(start_at: DateTime<FixedOffset>, version: impl Into<String>) -> Self {
        Self {
            start_at,
            version: version.into(),
        }
    }
}
