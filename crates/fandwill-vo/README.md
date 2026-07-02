# fandwill-vo

Value objects for the **Fandwill** platform HTTP API: shared request/response types with [`serde`](https://serde.rs/), validation via [`garde`](https://docs.rs/garde), and OpenAPI schemas via [`utoipa`](https://docs.rs/utoipa).

Use the **same crate version** in the Fandwill backend, future Rust SDK, and any Dart (or other) clients generated from the API contract.

## Installation

```toml
[dependencies]
fandwill-vo = "0.1"
```

Serde only (no validation or OpenAPI derives):

```toml
fandwill-vo = { version = "0.1", default-features = false }
```

| Feature  | Default | Description                            |
| -------- | ------- | -------------------------------------- |
| `garde`  | yes     | Field validation via `garde::Validate` |
| `utoipa` | yes     | OpenAPI schemas via `utoipa::ToSchema` |

## Modules

| Module      | Types (examples)                                                       |
| ----------- | ---------------------------------------------------------------------- |
| `auth`      | `SignUpVO`, `SignInVO`, `SignUpResponseVO`, `SignInResponseVO`         |
| `listings`  | `CreateListingVO`, `UpdateListingVO`, `ListingsVO`, `ListingVersionVO` |
| `users`     | `UsersVO`                                                              |
| `resources` | `ResourceVO`, `CreateResourceVO`                                       |
| `reviews`   | `CreateReviewVO`, `CreateReplyVO`, `ReviewReplyVO`, `ReviewsVO`        |
| `meta`      | `RootResponse`                                                         |

Request types may derive `garde::Validate`; API types used in OpenAPI derive `utoipa::ToSchema`.

## Example

```rust
use fandwill_vo::auth::SignUpVO;
use garde::Validate;

fn main() -> Result<(), garde::Report> {
    let body = SignUpVO {
        email: "user@example.com".into(),
        password: "password123".into(),
        nickname: Some("TestUser".into()),
    };
    body.validate()?;
    Ok(())
}
```

## Stability

**0.1.0** is the initial public release. While the HTTP API stabilizes, minor releases may add fields; read the [changelog](https://github.com/furlink/fandwil-rust-sdk/blob/main/CHANGELOG.md) before upgrading.

## Workspace

Part of the [Fandwill Rust SDK](https://github.com/furlink/fandwil-rust-sdk) repository. Maintainer release steps: [docs/RELEASE.md](https://github.com/furlink/fandwil-rust-sdk/blob/main/docs/RELEASE.md).

## License

Licensed under either of Apache License, Version 2.0 or MIT license, at your option. See [LICENSE-APACHE](https://github.com/furlink/fandwil-rust-sdk/blob/main/LICENSE-APACHE) and [LICENSE-MIT](https://github.com/furlink/fandwil-rust-sdk/blob/main/LICENSE-MIT) in the repository.

## Trademark

**Fandwill** is a brand of Furlink. This license does not grant permission to use Fandwill trademarks, logos, or branding except as needed to describe interoperability with the Fandwill platform. Do not imply endorsement or an official relationship without written permission from Furlink.
