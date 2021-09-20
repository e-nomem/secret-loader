// Copyright (c) The secret-loader Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Load secrets from multiple locations
//!
//! `secret-loader` provides a [`SecretLoader`] type that can load a [`SecretString`](secrecy::SecretString)
//! from an environment variable, a file, or directly as a String. The intended use case is to remove
//! hard-coded credentials in configuration files and replace them with hints on how an application should
//! load the secret instead. E.g. updating the following TOML configuration file:
//! ```toml
//! [user.alice]
//! username = "alice"
//! key = "somecrazypassword"
//!
//! [user.bob]
//! username = "bob"
//! key = "hello123"
//! ```
//! With the following configuration file instead:
//! ```toml
//! [user.alice]
//! username = "alice"
//! key = "env:ALICE_SECRET_KEY"
//!
//! [user.bob]
//! username = "bob"
//! key = "file:/home/bob/.auth_token"
//! ```
//!
//! # Basic Usage
//! Continuing with our configuration file above, here is how we could deserialize that TOML
//!
//! ```
//! use std::collections::HashMap;
//!
//! use secrecy::ExposeSecret;
//! use secrecy::SecretString;
//! use secret_loader::SecretLoader;
//! use serde::Deserialize;
//! # use std::env;
//! #
//! # env::set_var("ALICE_SECRET_KEY", "somecrazypassword");
//!
//! // Somewhere outside this program, the env var `ALICE_SECRET_KEY` has been set
//!
//! #[derive(Deserialize)]
//! pub struct UserConfig {
//!        username: String,
//!        key: SecretLoader,
//! }
//!
//! #[derive(Deserialize)]
//! pub struct Configuration {
//!     user: HashMap<String, UserConfig>,
//! }
//!
//! let config: Configuration = toml::from_str(r#"
//! [user.alice]
//! username = "alice"
//! key = "env:ALICE_SECRET_KEY"
//!
//! [user.bob]
//! username = "bob"
//! key = "file:/home/bob/.auth_token"
//! "#).unwrap();
//!
//! let alice_key: SecretString = config.user.get("alice")
//!        .unwrap()
//!        .key.clone()
//!        .into_secret()
//!        .unwrap();
//! assert_eq!(alice_key.expose_secret(), "somecrazypassword");
//! ```
//!
//! # Deserializing directly to `SecretString`
//! If you wish to deserialize directly to a `SecretString`, the `#[serde(deserialize_with = "..")]` attribute
//! can be used to help.
//!
//! ```
//! use std::collections::HashMap;
//!
//! use secrecy::ExposeSecret;
//! use secrecy::SecretString;
//! use secret_loader::SecretLoader;
//! use serde::Deserialize;
//! use serde::Deserializer;
//! use serde::de::Error as DeError;
//! # use std::env;
//! #
//! # env::set_var("ALICE_SECRET_KEY", "somecrazypassword");
//!
//! // Somewhere outside this program, the env var `ALICE_SECRET_KEY` has been set
//!
//! #[derive(Deserialize)]
//! pub struct UserConfig {
//!        username: String,
//!        #[serde(deserialize_with = "deserialize_secret")] // <-- Points at the fn defined below
//!        key: SecretString,
//! }
//!
//! #[derive(Deserialize)]
//! pub struct Configuration {
//!     user: HashMap<String, UserConfig>,
//! }
//!
//! // New deserialization fn HERE
//! pub fn deserialize_secret<'de, D>(deserializer: D) -> Result<SecretString, D::Error>
//! where
//!        D: Deserializer<'de>,
//! {
//!        SecretLoader::deserialize(deserializer)?.into_secret().map_err(DeError::custom)
//!    }
//!
//! let config: Configuration = toml::from_str(r#"
//! [user.alice]
//! username = "alice"
//! key = "env:ALICE_SECRET_KEY"
//! "#).unwrap();
//!
//! let alice_key: SecretString = config.user.get("alice")
//!        .unwrap()
//!        .key.clone();
//! assert_eq!(alice_key.expose_secret(), "somecrazypassword");
//! ```

#![warn(missing_docs)]
// only enables the `doc_cfg` feature when
// the `docsrs` configuration attribute is defined
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_root_url = "https://docs.rs/secret-loader/0.0.1")]

mod error;
mod loader;
#[cfg(feature = "serde")]
mod serde;

pub use crate::error::LoadError;
pub use crate::loader::SecretLoader;
