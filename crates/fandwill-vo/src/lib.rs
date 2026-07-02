//! Shared value objects for the [Fandwill](https://github.com/furlink/fandwil-rust-sdk) HTTP API.
//!
//! Use the same crate version in the Fandwill backend, future Rust SDK, and any generated
//! client bindings so JSON payloads stay aligned.

pub mod auth;
pub mod listings;
pub mod meta;
pub mod resources;
pub mod reviews;
pub mod users;
