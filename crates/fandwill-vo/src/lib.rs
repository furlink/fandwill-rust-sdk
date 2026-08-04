//! Shared value objects for the [Fandwill](https://github.com/furlink/fandwill-rust-sdk) HTTP API.
//!
//! Use the same crate version in the Fandwill backend, future Rust SDK, and any generated
//! client bindings so JSON payloads stay aligned.
//!
//! ## Features
//!
//! - **`garde`** (default) — `garde::Validate` on request types.
//! - **`utoipa`** (default) — `utoipa::ToSchema` for OpenAPI.
//!
//! Serde-only build: `default-features = false`.

pub mod auth;
pub mod collections;
pub mod listings;
pub mod meta;
pub mod notifications;
pub mod pagination;
pub mod resources;
pub mod reviews;
pub mod users;
pub mod validation;

#[cfg(all(test, feature = "garde"))]
mod tests {
    use crate::auth::SignUpVO;
    use garde::Validate;

    #[test]
    fn sign_up_vo_roundtrip_json() {
        let vo = SignUpVO {
            email: "user@example.com".into(),
            password: "password123".into(),
            nickname: Some("TestUser".into()),
        };
        vo.validate().expect("valid fixture");
        let json = serde_json::to_string(&vo).unwrap();
        let back: SignUpVO = serde_json::from_str(&json).unwrap();
        assert_eq!(back.email, vo.email);
        assert_eq!(back.password, vo.password);
        assert_eq!(back.nickname, vo.nickname);
    }
}

#[cfg(all(test, feature = "utoipa"))]
mod openapi_tests {
    use crate::{
        listings::ListingsQuery, notifications::NotificationsQuery, pagination::PaginationParams,
        reviews::ReviewFilter,
    };
    use utoipa::{IntoParams, openapi::path::ParameterIn};

    fn parameter_names<T: IntoParams>() -> Vec<String> {
        T::into_params(|| Some(ParameterIn::Query))
            .into_iter()
            .map(|parameter| parameter.name)
            .collect()
    }

    #[test]
    fn shared_query_types_expose_every_openapi_parameter() {
        assert_eq!(parameter_names::<PaginationParams>(), ["page", "page_size"]);
        assert_eq!(
            parameter_names::<ListingsQuery>(),
            [
                "page",
                "page_size",
                "mode",
                "q",
                "min_relevance",
                "max_distance",
            ]
        );
        assert_eq!(
            parameter_names::<ReviewFilter>(),
            ["listing_id", "page", "page_size"]
        );
        assert_eq!(
            parameter_names::<NotificationsQuery>(),
            ["page", "page_size", "unread_only", "after_id"]
        );
    }
}
