use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;
use utoipa::ToSchema;

#[derive(Debug, ToSchema, Serialize, Deserialize, Clone)]
pub struct ResourceVO {
    pub id: String,
    pub mime_type: Option<String>,
    pub hash: Option<String>,
    pub size_bytes: Option<i64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, ToSchema, Serialize, Deserialize, Clone)]
pub struct CreateResourceVO {
    pub id: String,
    pub upload_url: Url,
}
