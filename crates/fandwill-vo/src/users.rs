use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UsersVO {
    pub id: String,
    pub sub: String,
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
    use super::*;

    #[test]
    fn user_capabilities_roundtrips_json() {
        let capabilities = UserCapabilitiesVO { is_admin: true };
        let value = serde_json::to_value(capabilities).unwrap();
        assert_eq!(value, serde_json::json!({ "is_admin": true }));
        assert!(
            serde_json::from_value::<UserCapabilitiesVO>(value)
                .unwrap()
                .is_admin
        );
    }
}
