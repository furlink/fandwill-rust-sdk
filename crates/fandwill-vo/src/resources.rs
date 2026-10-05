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
    /// BlurHash placeholder for progressive image rendering, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blurhash: Option<String>,
    /// EXIF-oriented display width in pixels, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    /// EXIF-oriented display height in pixels, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_metadata_roundtrips_and_legacy_payloads_default_to_none() {
        let resource = ResourceVO {
            id: "resource".into(),
            mime_type: Some("image/png".into()),
            hash: None,
            size_bytes: Some(42),
            created_at: DateTime::<Utc>::from_timestamp(0, 0).unwrap(),
            blurhash: Some("LEHV6nWB2yk8pyo0adR*.7kCMdnj".into()),
            width: Some(640),
            height: Some(480),
        };
        let value = serde_json::to_value(&resource).unwrap();
        assert_eq!(value["width"], 640);
        assert_eq!(value["height"], 480);
        assert_eq!(value["blurhash"], "LEHV6nWB2yk8pyo0adR*.7kCMdnj");

        let legacy: ResourceVO = serde_json::from_value(serde_json::json!({
            "id": "legacy",
            "mime_type": null,
            "hash": null,
            "size_bytes": null,
            "created_at": "1970-01-01T00:00:00Z"
        }))
        .unwrap();
        assert!(legacy.blurhash.is_none());
        assert!(legacy.width.is_none());
        assert!(legacy.height.is_none());
        let legacy_json = serde_json::to_value(legacy).unwrap();
        assert!(legacy_json.get("blurhash").is_none());
        assert!(legacy_json.get("width").is_none());
        assert!(legacy_json.get("height").is_none());
    }
}
