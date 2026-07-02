# Changelog

All notable changes to published crates in this workspace are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **`fandwill-sdk`** (workspace) — async HTTP client crate built on `reqwest`.
  - `FandwillClient` with configurable base URL, trailing `/` auto-append.
  - `Auth` for **API key** (`X-Api-Key`) and **JWT** (`Authorization: Bearer`).
  - Endpoint groups: `root`, `sign_up`, `sign_in` (auth); `get_listings`,
    `get_listing`, `add_listing`, `update_listing`, `delete_listing`,
    `bookmark_listing`, `unbookmark_listing`, `get_listing_versions` (listings).
  - `send_json<T>()` / `send_empty()` helpers for flattened API response handling.
  - `Error` (`thiserror`) for `Request`, `InvalidUrl`, `Status`, `Json` errors.
  - `ListingsQuery` / `SearchMode` for paginated and search query parameters.
  - `PagedResponse<T>` / `PageInfo` for paginated endpoint responses.
  - `frb` feature flag (empty, reserved for Flutter Rust Bridge integration).
  - Re-exports `fandwill_vo` so apps can depend on `fandwill-sdk` only.
  - `AGENTS.md` agent guide with CI commands and key development facts.

- **`fandwill-vo`** — `meta::RootResponse` with `Deserialize` (matches published contract).

### Changed

- **`fandwill-sdk`** — `Error` variants now use `String`/`u16` instead of
  `reqwest::StatusCode`, `url::ParseError`, `serde_json::Error`, etc., making
  the error type compatible with Flutter Rust Bridge codegen.
- **`fandwill-sdk`** — `ListingsQuery` lifetime removed (`&str` → `String`),
  added `Clone` derive for FRB compatibility.

## [0.1.0] - 2026-07-02

### Added

- **`fandwill-vo`** — initial public release of Fandwill API value objects.
- MIT OR Apache-2.0 licensing, crate README, and release documentation.

### Changed

- **`fandwill-vo`** — `garde` and `utoipa` are optional Cargo features
  (`default = ["garde", "utoipa"]`). Types always derive `serde`; `Validate` /
  `ToSchema` and field attributes apply only when the corresponding feature is
  enabled. Use `default-features = false` for a serde-only dependency.

[Unreleased]: https://github.com/furlink/fandwil-rust-sdk/compare/fandwill-vo-v0.1.0...HEAD
[0.1.0]: https://github.com/furlink/fandwil-rust-sdk/releases/tag/fandwill-vo-v0.1.0
