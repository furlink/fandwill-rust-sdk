# Fandwill Rust SDK — Agent Guide

## Workspace

Two crates, `resolver = "3"`, `edition = "2024"`:

| Crate          | Path                   | Purpose                                                                                      |
| -------------- | ---------------------- | -------------------------------------------------------------------------------------------- |
| `fandwill-vo`  | `crates/fandwill-vo/`  | Serde value objects (shared with backend). `garde` + `utoipa` are optional default features. |
| `fandwill-sdk` | `crates/fandwill-sdk/` | Async HTTP client. Re-exports `fandwill_vo` as `fandwill_sdk::fandwill_vo`.                  |

## Commands (CI order)

```sh
cargo fmt --all -- --check
cargo clippy -- -D warnings
cargo test --all
cargo publish --dry-run -p fandwill-vo
cargo package -p fandwill-sdk --list                # before matching VO is published
cargo publish --dry-run -p fandwill-sdk             # after matching VO is published
```

`cargo test --all` or `cargo test -p <crate>` are the test commands.

## Key facts

- **fandwill-vo features**: `default = ["garde", "utoipa"]`. Use `--all-features` to test with both. `default-features = false` for serde-only.
- **fandwill-sdk**: `fandwill-vo` uses a path plus the matching published version with `default-features = false`. The `version` is required for `cargo publish --dry-run` — without it manifest verification fails.
- **`FandwillClient::new(url)`** auto-appends trailing `/` to the base URL so path joins work correctly.
- **Auth**: `client.with_api_key(key)` or `client.with_jwt(token)` (returns owned `Self`, builder style). `$ref: bearer_auth` → use `with_jwt`.
- **Response format**: success = HTTP 200 with flattened fields (no envelope); error = real HTTP status + `{"code":..., "msg":...}`.
- **Paginated responses** → `PagedResponse<T>` / `PageInfo` in `fandwill_sdk`.
- **New endpoints**: add `impl FandwillClient {}` blocks in `endpoints/<domain>.rs`, register in `endpoints/mod.rs`.
- **Targets**: `fandwill-sdk` is native-only and uses reqwest with rustls; WASM is not supported.
- **`fandwill_vo::meta::RootResponse`** was missing from the local crate and was added to match the published `fandwill-vo` contract.
