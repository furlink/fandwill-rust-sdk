use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct RootResponse {
    pub start_at: DateTime<Utc>,
    pub version: String,
    pub limits: RootLimits,
}

/// Server-enforced limits that clients should honor before issuing requests.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct RootLimits {
    /// Maximum accepted resource upload size in bytes.
    ///
    /// Matches the per-upload [`crate::resources::ResourceUploadVO::max_bytes`] value; exposed here
    /// so clients can validate file sizes before creating an upload.
    pub max_upload_bytes: u64,
}

impl RootResponse {
    pub fn new(
        start_at: impl Into<DateTime<Utc>>,
        version: impl Into<String>,
        limits: RootLimits,
    ) -> Self {
        Self {
            start_at: start_at.into(),
            version: version.into(),
            limits,
        }
    }
}
