# secret-loader
### Load secrets from multiple locations
[![Github CI](https://github.com/e-nomem/secret-loader/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/e-nomem/secret-loader/actions/workflows/ci.yml)
[![secret-loader on crates.io](https://img.shields.io/crates/v/secret-loader)](https://crates.io/crates/secret-loader)
[![Documentation (latest release)](https://docs.rs/secret-loader/badge.svg)](https://docs.rs/secret-loader)
[![License](https://img.shields.io/badge/license-Apache-green.svg)](LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE-MIT)

`secret-loader` provides a `SecretLoader` type that can load a `SecretString`
from an environment variable, a file, or directly as a String. The intended use case is to remove
hard-coded credentials in configuration files and replace them with hints on how an application should
load the secret instead. E.g. updating the following TOML configuration file:
```toml
[user.alice]
username = "alice"
key = "somecrazypassword"

[user.bob]
username = "bob"
key = "hello123"
```
With the following configuration file instead:
```toml
[user.alice]
username = "alice"
key = "env:ALICE_SECRET_KEY"

[user.bob]
username = "bob"
key = "file:/home/bob/.auth_token"
```

## Optional Features
`secret-loader` currently implements the following feature flags:

| Feature Name | Description |
| --- | --- |
| serde | Enable automatic deserialization of a `SecretLoader` |

## License

This project is available under the terms of either the [Apache 2.0 license](LICENSE-APACHE) or the [MIT
license](LICENSE-MIT).

This project's documentation is adapted from [The Rust Programming Language](https://github.com/rust-lang/rust/), which is
available under the terms of either the [Apache 2.0 license](https://github.com/rust-lang/rust/blob/master/LICENSE-APACHE)
or the [MIT license](https://github.com/rust-lang/rust/blob/master/LICENSE-MIT).
