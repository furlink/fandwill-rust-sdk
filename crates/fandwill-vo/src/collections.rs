use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Request body for adding a resource to the authenticated user's collection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AddToCollectionRequest {
    /// Sqid of the resource to add.
    pub resource_id: String,
}

/// A single entry in the authenticated user's collection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CollectionEntryVO {
    /// Sqid of this collection entry.
    pub id: String,

    /// Sqid of the resource.
    pub resource_id: String,

    /// ISO-8601 timestamp of when it was added.
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collection_entry_roundtrips_json() {
        let entry = CollectionEntryVO {
            id: "entry".into(),
            resource_id: "resource".into(),
            created_at: DateTime::parse_from_rfc3339("2026-07-24T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
        };

        let json = serde_json::to_value(&entry).unwrap();
        assert_eq!(
            serde_json::from_value::<CollectionEntryVO>(json).unwrap(),
            entry
        );
    }
}
