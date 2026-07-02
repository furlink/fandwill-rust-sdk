# Changelog

All notable changes to published crates in this workspace are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

