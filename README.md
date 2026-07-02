# Fandwill Rust SDK

Monorepo for **Fandwill** API client and shared types. The first published crate is the JSON contract layer; a higher-level Rust SDK crate is planned.

## Who uses what

| Consumer                           | Role                                                                           |
| ---------------------------------- | ------------------------------------------------------------------------------ |
| **Fandwill backend** (proprietary) | Depends on `fandwill-vo` for request/response shapes and validation            |
| **Open-source Rust SDK** (planned) | Will depend on the same `fandwill-vo` version from crates.io                   |
| **Dart SDK** (planned)             | Generated from the same API contract; align releases with `fandwill-vo` semver |

Treat **`fandwill-vo` as the source of truth for payload shapes** in Rust. Keep the same version pinned everywhere when you ship an API change.

## Crates

| Crate                               | crates.io     | Description                            |
| ----------------------------------- | ------------- | -------------------------------------- |
| [`fandwill-vo`](crates/fandwill-vo) | `fandwill-vo` | Serde, garde, and utoipa value objects |

```toml
[dependencies]
fandwill-vo = "0.1"
```

See [CHANGELOG.md](./CHANGELOG.md) and [docs/RELEASE.md](./docs/RELEASE.md) for releases.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](./LICENSE-MIT))

at your option.

## Trademark

**Fandwill** is a brand of Furlink. This license does not grant permission to use Fandwill trademarks, logos, or branding except as needed to describe that your software works with the Fandwill platform. Do not imply endorsement or an official relationship without written permission from Furlink.

