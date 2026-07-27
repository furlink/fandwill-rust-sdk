use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ResourceVO {
    pub id: String,
    pub mime_type: Option<String>,
    pub hash: Option<String>,
    pub size_bytes: Option<i64>,
    pub created_at: DateTime<Utc>,
}

/// S3 POST policy upload descriptor returned by `POST /resources`.
///
/// Clients must `POST` `multipart/form-data` to [`ResourceUploadVO::url`], appending every entry in
/// [`ResourceUploadVO::fields`] and finally the file part named `file`. Uploads larger than
/// [`ResourceUploadVO::max_bytes`] are rejected by the object store.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ResourceUploadVO {
    /// Destination URL for the multipart POST (bucket endpoint).
    pub url: Url,
    /// HTTP method; always `"POST"` for the current contract.
    pub method: String,
    /// Form fields that must be submitted with the upload (`Policy`, `X-Amz-*`, `key`, …).
    pub fields: BTreeMap<String, String>,
    /// Maximum accepted object size in bytes (`content-length-range` upper bound).
    pub max_bytes: u64,
    /// Presign / policy lifetime in seconds.
    pub expires_in_secs: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateResourceVO {
    pub id: String,
    pub upload: ResourceUploadVO,
}
