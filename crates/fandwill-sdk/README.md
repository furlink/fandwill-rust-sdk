# fandwill-sdk

Async HTTP client for the **Fandwill** API. Payload types are re-exported as [`fandwill_vo`](https://docs.rs/fandwill-vo) (no separate `fandwill-vo` dependency needed).

- **Native** (iOS, Android, desktop): default feature `native` enables HTTPS via `reqwest` + rustls.
- **WASM** (Flutter Web): build with feature `wasm` (browser `fetch`, no rustls).

## Installation

```toml
[dependencies]
fandwill-sdk = "0.1"
```

WASM build:

```bash
cargo build -p fandwill-sdk --target wasm32-unknown-unknown --no-default-features --features wasm
```

## Example

Types: `fandwill_sdk::fandwill_vo::auth::SignUpVO`, etc.

```rust
use fandwill_sdk::fandwill_vo::auth::{SignInVO, SignUpVO};
use fandwill_sdk::FandwillClient;

#[tokio::main]
async fn main() -> Result<(), fandwill_sdk::SdkError> {
    let client = FandwillClient::new("***")?;
    let meta = client.root().await?;
    println!("{} {}", meta.version, meta.start_at);
    let _ = client.sign_up(&SignUpVO {
        email: "user@example.com".into(),
        password: "password123".into(),
        nickname: None,
    }).await?;
    let session = client.sign_in(&SignInVO::Email {
        email: "user@example.com".into(),
        password: "password123".into(),
    }).await?;
    let client = client.with_jwt(session.token);
    let _ = client;
    Ok(())
}
```

## License

MIT OR Apache-2.0, same as the workspace.
