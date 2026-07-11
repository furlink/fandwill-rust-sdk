use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::resources::ResourceVO;

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
