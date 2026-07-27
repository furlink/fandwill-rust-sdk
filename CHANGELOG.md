# Changelog

All notable changes to published crates in this workspace are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/furlink/fandwil-rust-sdk/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/furlink/fandwil-rust-sdk/releases/tag/v0.2.0
[0.1.1]: https://github.com/furlink/fandwil-rust-sdk/releases/tag/v0.1.1
[0.1.0]: https://github.com/furlink/fandwil-rust-sdk/releases/tag/fandwill-vo-v0.1.0
