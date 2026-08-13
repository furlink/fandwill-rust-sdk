# Fandwill Rust SDK

Rust value objects and an async HTTP client for the **Fandwill** platform API.

## Crates

| Crate | crates.io | Description |
| --- | --- | --- |
| [`fandwill-vo`](./crates/fandwill-vo/README.md) | `fandwill-vo` | Shared Serde, garde, and utoipa value objects |
| [`fandwill-sdk`](./crates/fandwill-sdk/README.md) | `fandwill-sdk` | Async client for the complete public API surface |

```toml
[dependencies]
fandwill-sdk = "0.5"
```

The backend and SDK should use the same `fandwill-vo` version when a payload contract changes. The SDK re-exports the value-object crate as `fandwill_sdk::fandwill_vo`.

The SDK targets native platforms and uses rustls for HTTPS. WASM is not supported.

See [CHANGELOG.md](./CHANGELOG.md) for release history and the crate READMEs for usage.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](./LICENSE-MIT))

at your option.

## Trademark

**Fandwill** is a brand of Furlink. This license does not grant permission to use Fandwill trademarks, logos, or branding except as needed to describe that your software works with the Fandwill platform. Do not imply endorsement or an official relationship without written permission from Furlink.
