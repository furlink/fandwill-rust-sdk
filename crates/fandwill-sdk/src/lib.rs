//! Async HTTP client for the [Fandwill](https://github.com/furlink/fandwil-rust-sdk) API.
//!
//! Uses [`reqwest`] on native targets and browser `fetch` on `wasm32-unknown-unknown`.
//! Payload types come from [`fandwill_vo`].

mod auth;
mod client;
mod error;

pub use auth::Auth;
pub use client::FandwillClient;
pub use error::SdkError;

/// Request/response types for the Fandwill API (re-exported from `fandwill-vo`).
pub use fandwill_vo;
