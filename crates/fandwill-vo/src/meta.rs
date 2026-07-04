use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct RootResponse {
    pub start_at: DateTime<Utc>,
    pub version: String,
}

impl RootResponse {
    pub fn new(start_at: impl Into<DateTime<Utc>>, version: impl Into<String>) -> Self {
        Self {
            start_at: start_at.into(),
            version: version.into(),
        }
    }
}
