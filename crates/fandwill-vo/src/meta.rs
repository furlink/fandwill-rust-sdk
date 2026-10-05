use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct RootResponse {
    pub start_at: DateTime<Utc>,
    pub version: String,
    /// Whether clients must supply an invitation code when signing up.
    #[serde(default)]
    pub invitation_required: bool,
    pub limits: RootLimits,
    /// Trusted host rules for external images embedded in listing Markdown.
    #[serde(default)]
    pub trusted_image_domains: Vec<TrustedImageDomainVO>,
}

/// One trusted host rule for external images embedded in listing Markdown.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct TrustedImageDomainVO {
    pub domain: String,
    pub include_subdomains: bool,
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

    /// Maximum decoded pixel count (`width × height`) accepted by the JPEG/PNG/WebP ingestion gate.
    pub max_image_pixels: u64,
}

impl RootResponse {
    pub fn new(
        start_at: impl Into<DateTime<Utc>>,
        version: impl Into<String>,
        invitation_required: bool,
        limits: RootLimits,
        trusted_image_domains: Vec<TrustedImageDomainVO>,
    ) -> Self {
        Self {
            start_at: start_at.into(),
            version: version.into(),
            invitation_required,
            limits,
            trusted_image_domains,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_limits_roundtrips_with_image_pixel_limit() {
        let limits = RootLimits {
            max_upload_bytes: 20_971_520,
            max_image_pixels: 25_000_000,
        };
        let value = serde_json::to_value(&limits).unwrap();
        assert_eq!(
            value,
            serde_json::json!({
                "max_upload_bytes": 20_971_520,
                "max_image_pixels": 25_000_000,
            })
        );
        assert_eq!(
            serde_json::from_value::<RootLimits>(value)
                .unwrap()
                .max_image_pixels,
            limits.max_image_pixels
        );
    }

    #[test]
    fn root_response_roundtrips_runtime_registration_and_image_policy() {
        let response = RootResponse::new(
            DateTime::<Utc>::from_timestamp(0, 0).unwrap(),
            "0.5.0",
            true,
            RootLimits {
                max_upload_bytes: 20_971_520,
                max_image_pixels: 25_000_000,
            },
            vec![TrustedImageDomainVO {
                domain: "images.example.com".into(),
                include_subdomains: true,
            }],
        );
        let value = serde_json::to_value(&response).unwrap();
        assert_eq!(value["invitation_required"], true);
        assert_eq!(
            value["trusted_image_domains"][0]["domain"],
            "images.example.com"
        );

        let decoded: RootResponse = serde_json::from_value(value).unwrap();
        assert!(decoded.invitation_required);
        assert_eq!(decoded.trusted_image_domains.len(), 1);
    }

    #[test]
    fn root_response_defaults_fields_missing_from_older_servers() {
        let response: RootResponse = serde_json::from_value(serde_json::json!({
            "start_at": "1970-01-01T00:00:00Z",
            "version": "0.4.0",
            "limits": { "max_upload_bytes": 10, "max_image_pixels": 20 }
        }))
        .unwrap();

        assert!(!response.invitation_required);
        assert!(response.trusted_image_domains.is_empty());
    }
}
