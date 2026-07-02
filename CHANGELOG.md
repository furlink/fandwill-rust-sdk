# Changelog

All notable changes to published crates in this workspace are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **`fandwill-sdk`** (workspace) — async HTTP client crate built on `reqwest`.
  - `FandwillClient` with configurable base URL and shared `request` / JSON response helpers.
  - `Auth` for **API key** (`X-Api-Key`) and **JWT** (`Authorization: Bearer`), plus
    `with_auth`, `with_api_key`, and `with_jwt` on the client.
  - `SdkError` (`thiserror`) for transport, URL, non-success HTTP status, and JSON errors.
  - Re-exports `fandwill_vo` so apps can depend on `fandwill-sdk` only for types and client.
  - Crate README and workspace membership; root README lists the SDK crate.

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
