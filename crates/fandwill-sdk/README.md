# fandwill-sdk

Async HTTP client for the **Fandwill** API. Request and response types are re-exported through [`fandwill_vo`](https://docs.rs/fandwill-vo), so consumers do not need a separate direct dependency.

The client covers the complete public OpenAPI surface: metadata, authentication, listings and bookmarks, listing capability administration, resources, reviews and replies, users and collections, and notifications.

## Installation

```toml
[dependencies]
fandwill-sdk = "0.4"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

The SDK targets native platforms and enables HTTPS through `reqwest` and rustls automatically. WASM is not supported, and there are no target-selection Cargo features.

## Example

The API base URL includes the `/api/` prefix. A missing trailing slash is accepted and normalized by `FandwillClient::new`.

```rust
use fandwill_sdk::fandwill_vo::auth::{SignInVO, SignUpVO};
use fandwill_sdk::FandwillClient;

#[tokio::main]
async fn main() -> Result<(), fandwill_sdk::Error> {
    let client = FandwillClient::new("https://dev.fandwill.com/api/")?;
    let meta = client.root().await?;
    println!("{} {}", meta.version, meta.start_at);

    let _ = client
        .sign_up(&SignUpVO {
            email: "user@example.com".into(),
            password: "password123".into(),
            nickname: None,
        })
        .await?;

    let session = client
        .sign_in(&SignInVO::Email {
            email: "user@example.com".into(),
            password: "password123".into(),
        })
        .await?;
    let authenticated = client.with_jwt(session.token);
    let _ = authenticated.get_me().await?;
    Ok(())
}
```

The current OpenAPI contract uses bearer JWT authentication. `with_api_key` remains only for source compatibility with older deployments and is deprecated.

## Responses and errors

Successful JSON responses are deserialized from the API's flattened payload shape. Paginated operations return `PagedResponse<T>` with `items` and `page_info`.

Non-success responses remain available as `Error::Status { status, body }`. For the documented JSON error shape, `Error::api_error()` returns a typed `ApiError { code, msg }` while preserving the original status and body.

`get_resource` returns the presigned target from the API's `302 Location` header without downloading the object.

## License

MIT OR Apache-2.0, same as the workspace.
