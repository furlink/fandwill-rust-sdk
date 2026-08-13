# Changelog

All notable changes to published crates in this workspace are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Breaking

- **`fandwill-vo`** — `ReviewsVO` gains `like_count`, `viewer_liked`, and `created_at` fields (constructors must now supply all three).
- **`fandwill-vo`** — `RootLimits` gains the required `max_image_pixels` field (constructors must now supply it).
- **`fandwill-vo`** — `ListingsVO` gains the required `capabilities` field describing its edit and reply audiences.
- **`fandwill-vo`** — `UsersVO { id, sub }` is replaced by `UserVO { id, display_name, created_at }`; the IAM subject is no longer part of the public profile contract.
- **`fandwill-vo`** — `UpdateListingCapabilitiesVO` fields `edit` and `reply` are now `Option<ListingCapabilityAudience>`; omitted or `null` fields keep the listing's current values, matching the partial-update semantics of `PATCH /listings/{id}/capabilities`.

### Added

- **`fandwill-vo`** — `ReviewsVO` now reports the like count (`like_count`), whether the requesting user has liked the review (`viewer_liked`), and the review creation time (`created_at`).
- **`fandwill-vo`** — `RootLimits` exposes `max_image_pixels` so clients can preflight decoded image dimensions.
- **`fandwill-vo`** — `UserCapabilitiesVO` describes the `GET /users/me/capabilities` response.
- **`fandwill-sdk`** — `get_my_capabilities` covers `GET /users/me/capabilities`.
- **`fandwill-vo`** — listing capability audience, response, and administrator update request contracts.
- **`fandwill-sdk`** — `update_listing_capabilities` covers administrator `PATCH /listings/{id}/capabilities` updates.
- **`fandwill-vo`** — `UpdateUserProfileVO` describes partial profile updates with a validated optional display name.
- **`fandwill-sdk`** — `update_me` covers `PATCH /users/me`; `get_me` and `get_user` now return `UserVO`.

## [0.4.0] - 2026-08-04

### Breaking

- **`fandwill-sdk`** — `add_listing` and `update_listing` now return `ListingsVOWithValidation`, preserving the API's `markdown_validation` results instead of silently discarding them.
- **`fandwill-sdk`** — `get_listing_versions` now returns `PagedResponse<ListingVersionVO>` to match the wire response.
- **`fandwill-sdk`** — `Error` is now non-exhaustive and adds explicit redirect failures for missing or invalid `Location` headers.
- **`fandwill-sdk`** — WASM support has been removed; the client now targets native platforms only.

### Added

- **`fandwill-sdk`** — complete endpoint coverage for the 38-operation public OpenAPI contract, including bookmark lookup, resources, reviews and replies, users and collections, and notifications.
- **`fandwill-sdk`** — native HTTPS through reqwest with rustls.
- **`fandwill-sdk`** — typed `ApiError` access through `Error::api_error()` while preserving the existing status and raw response body.
- **`fandwill-sdk`** — `get_resource` redirect handling that returns the presigned `Location` URL without downloading the object.
- **`fandwill-vo`** — bookmark and notification response/query types, including tagged notification payload and action enums.

### Changed

- **`fandwill-vo`** — validation response wrappers can now be deserialized by API clients; sign-in and sign-up response types tolerate flattened response metadata.
- **`fandwill-sdk`** — `with_api_key` is deprecated because the current OpenAPI contract only declares bearer JWT authentication.
- Updated crate documentation, installation versions, target support claims, API base URL examples, and pre-release package checks for the required VO-before-SDK publication order.

## [0.3.0] - 2026-08-04

### Breaking

- **`fandwill-vo`** — `ListingsQuery` search contract reworked: `mode` is now `Option<SearchMode>` (absent means browse, present selects the search type), and the legacy `by`/`query` pair is unified into a single `q` term whose interpretation depends on `mode`.
- **`fandwill-vo`** — review contracts: `CreateReviewVO.of_listing` renamed to `listing_id` with `rating` removed from both `CreateReviewVO` and `ReviewsVO`; `CreateReplyVO.parent_id` and `ReviewReplyVO.parent_id` renamed to `parent_reply_id`.
- **`fandwill-vo`** — `RootResponse` gains a `limits: RootLimits` field (currently `max_upload_bytes`) and `RootResponse::new` takes the limits as a third argument.

### Added

- **`fandwill-vo`** — `CreateReviewVO.content` and `CreateReplyVO.content` reject blank (whitespace-only) values via garde validation.

## [0.2.0] - 2026-07-27

### Added

- **`fandwill-vo`** — shared collection request/response types.
- **`fandwill-vo`** — shared pagination, listing search query, and review filter types with Serde and OpenAPI support.
- **`fandwill-vo`** — `ResourceUploadVO` describing S3 POST-policy direct uploads (`url`, `method`, `fields`, `max_bytes`, `expires_in_secs`).

### Changed

- **`fandwill-sdk`** — `ListingsQuery`, `SearchMode`, `PageInfo`, and `PagedResponse<T>` now re-export the shared `fandwill-vo` definitions.
- **`fandwill-vo`** — listing search modes serialize as lowercase `fts` / `semantic` while still accepting the legacy `Fts` / `Semantic` spellings.

### Breaking

- **`fandwill-vo`** — `CreateResourceVO.upload_url` is replaced by `CreateResourceVO.upload: ResourceUploadVO`. Clients must upload with `multipart/form-data` POST (not raw PUT to a presigned URL).

## [0.1.1] - 2026-07-11

### Removed

- **`fandwill-sdk`** — removed all FRB (Flutter Rust Bridge) code:
  - Removed `frb` Cargo feature flag and optional `chrono` dependency.
  - Removed `FrbError` enum and `frb_error_from!` macro.
  - Removed `frb_dispatch!` macro; endpoint methods now use direct `impl FandwillClient {}` blocks instead.
  - Updated `AGENTS.md` to reflect the simplified codebase.

### Added

- **`fandwill-vo`** — `UpdateListingVersionStatusVO` for listing version status changes.
- **`fandwill-vo`** — validation wrapper types (`ListingsVOWithValidation`, `ReviewsVOWithValidation`, `ReviewReplyVOWithValidation`) for markdown validation.
- **`fandwill-vo`** — `RootResponse::new()` convenience constructor.
- **`fandwill-sdk`** — async HTTP client built on `reqwest` exposing
  `FandwillClient`, `Auth` (API key / JWT), `send_json`/`send_empty` helpers,
  endpoints (`root`, `sign_up`, `sign_in`, `get_listings`, `get_listing`,
  `add_listing`, `update_listing`, `delete_listing`, `bookmark_listing`,
  `unbookmark_listing`, `get_listing_versions`, `update_listing_version_status`), `ListingsQuery`/`SearchMode`,
  `PagedResponse<T>`/`PageInfo`, `thiserror`-based `Error`, and re-exports
  `fandwill_vo`.
- **`fandwill-vo`** — `meta::RootResponse` with `Deserialize` (matches
  published contract).

### Changed

- **`fandwill-sdk`** — `ListingsQuery` lifetime removed (`&str` → `String`),
  added `Clone` derive.

## [0.1.0] - 2026-07-02

### Added

- **`fandwill-vo`** — initial public release of Fandwill API value objects.
- MIT OR Apache-2.0 licensing, crate README, and release documentation.

### Changed

- **`fandwill-vo`** — `garde` and `utoipa` are optional Cargo features
  (`default = ["garde", "utoipa"]`). Types always derive `serde`; `Validate` /
  `ToSchema` and field attributes apply only when the corresponding feature is
  enabled. Use `default-features = false` for a serde-only dependency.

[Unreleased]: https://github.com/furlink/fandwill-rust-sdk/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/furlink/fandwill-rust-sdk/releases/tag/v0.4.0
[0.3.0]: https://github.com/furlink/fandwill-rust-sdk/releases/tag/v0.3.0
[0.2.0]: https://github.com/furlink/fandwill-rust-sdk/releases/tag/v0.2.0
[0.1.1]: https://github.com/furlink/fandwill-rust-sdk/releases/tag/v0.1.1
[0.1.0]: https://github.com/furlink/fandwill-rust-sdk/releases/tag/v0.1.0
