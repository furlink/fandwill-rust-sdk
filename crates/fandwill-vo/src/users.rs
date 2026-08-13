use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A user's local identity and profile.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UserVO {
    pub id: String,
    pub display_name: String,
    pub created_at: DateTime<Utc>,
}

/// Mutable profile fields accepted by `PATCH /users/me`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "garde", derive(garde::Validate))]
#[serde(deny_unknown_fields)]
pub struct UpdateUserProfileVO {
    #[cfg_attr(feature = "garde", garde(length(chars, min = 4, max = 64)))]
    #[cfg_attr(
        feature = "utoipa",
        schema(min_length = 4, max_length = 64, example = "Alice")
    )]
    pub display_name: Option<String>,
}

/// Client-visible capabilities for the authenticated user.
///
/// These values are UI hints; every endpoint still enforces its permissions server-side.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UserCapabilitiesVO {
    /// Whether the authenticated user has administrator privileges.
    ///
    /// This is a UI hint only; administrative endpoints still enforce permissions server-side.
    pub is_admin: bool,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[cfg(feature = "garde")]
    #[test]
    fn update_profile_applies_public_length_gate() {
        use garde::Validate;

        for display_name in ["abc".to_owned(), "用户甲".to_owned(), "a".repeat(65)] {
            let update = UpdateUserProfileVO {
                display_name: Some(display_name),
            };
            assert!(update.validate().is_err());
        }

        for display_name in ["abcd", "用户甲乙"] {
            let update = UpdateUserProfileVO {
                display_name: Some(display_name.to_owned()),
            };
            update
                .validate()
                .expect("four Unicode characters should be valid");
        }

        UpdateUserProfileVO { display_name: None }
            .validate()
            .expect("an omitted display name should be valid");
    }

    #[test]
    fn update_profile_deserializes_missing_and_null_as_no_change() {
        for value in [json!({}), json!({ "display_name": null })] {
            let update: UpdateUserProfileVO = serde_json::from_value(value).unwrap();
            assert_eq!(update.display_name, None);
        }
    }

    #[test]
    fn user_wire_contract_omits_the_iam_subject() {
        let user = UserVO {
            id: "local-user".into(),
            display_name: "Alice".into(),
            created_at: "2026-08-11T00:00:00Z".parse().unwrap(),
        };

        assert_eq!(
            serde_json::to_value(user).unwrap(),
            json!({
                "id": "local-user",
                "display_name": "Alice",
                "created_at": "2026-08-11T00:00:00Z"
            })
        );
    }

    #[test]
    fn user_capabilities_roundtrips_json() {
        let capabilities = UserCapabilitiesVO { is_admin: true };
        let value = serde_json::to_value(capabilities).unwrap();
        assert_eq!(value, json!({ "is_admin": true }));
        assert!(
            serde_json::from_value::<UserCapabilitiesVO>(value)
                .unwrap()
                .is_admin
        );
    }
}
