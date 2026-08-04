//! Async HTTP client for the [Fandwill](https://github.com/furlink/fandwill-rust-sdk) API.
//!
//! Uses [`reqwest`] with rustls on native targets. Payload types come from [`fandwill_vo`].

mod auth;
mod client;
mod endpoints;
mod error;
mod query;
mod response;

pub use auth::Auth;
pub use client::FandwillClient;
pub use error::{ApiError, Error};
pub use query::{ListingsQuery, NotificationsQuery, PaginationParams, ReviewFilter, SearchMode};
pub use response::{PageInfo, PagedResponse};

/// Request/response types for the Fandwill API (re-exported from `fandwill-vo`).
pub use fandwill_vo;
