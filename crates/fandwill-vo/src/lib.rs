//! Shared value objects for the [Fandwill](https://github.com/furlink/fandwil-rust-sdk) HTTP API.
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
pub mod listings;
pub mod meta;
pub mod resources;
pub mod reviews;
pub mod users;

#[cfg(all(test, feature = "garde"))]
mod tests {
    use crate::auth::SignUpVO;
    use garde::Validate;
    use serde_json;

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
