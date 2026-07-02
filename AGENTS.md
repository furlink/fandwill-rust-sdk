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
cargo check --target wasm32-unknown-unknown    # SDK only; no wasm feature exists yet
cargo publish --dry-run -p fandwill-sdk        # or -p fandwill-vo
```

`cargo test --all` or `cargo test -p <crate>` are the test commands.

## Key facts

- **fandwill-vo features**: `default = ["garde", "utoipa"]`. Use `--all-features` to test with both. `default-features = false` for serde-only.
- **fandwill-sdk**: `fandwill-vo` dep uses `{ path = "../fandwill-vo", version = "0.1", default-features = false }`. The `version` is required for `cargo publish --dry-run` — without it manifest verification fails.
- **`FandwillClient::new(url)`** auto-appends trailing `/` to the base URL so path joins work correctly.
- **Auth**: `client.with_api_key(key)` or `client.with_jwt(token)` (returns owned `Self`, builder style). `$ref: bearer_auth` → use `with_jwt`.
- **Response format**: success = HTTP 200 with flattened fields (no envelope); error = real HTTP status + `{"code":..., "msg":...}`.
- **Paginated responses** → `PagedResponse<T>` / `PageInfo` in `fandwill_sdk`.
- **`frb_dispatch!` macro** (in `macros.rs`): endpoint implementations use a free-function style with `client` instead of `self`. The macro generates both the `FandwillClient` method and (with `--features frb`) a variant returning `FrbError`. **New endpoints**: write as `frb_dispatch! { pub async fn foo = foo_impl (client, ...) -> ... { ... } }` in `endpoints/<domain>.rs`, register in `endpoints/mod.rs`. Do not write bare `impl FandwillClient {}` blocks — use the macro.
- **FRB feature**: use `cargo clippy -p fandwill-sdk --features frb` to verify FRB-compatible code paths (`FrbError` + `frb_dispatch!` error conversion).
- **`frb_error_from!`** macro auto-generates `From<Error> for FrbError` (String/u16 variants mirroring Error's concrete types).
- **WASM**: the README and doc-comment describe WASM support, but the feature flag (`wasm`) and `cfg(target_arch = "wasm32")` code do not exist yet — do not claim WASM works.
- **`fandwill_vo::meta::RootResponse`** was missing from the local crate and was added to match the published `fandwill-vo` contract.
