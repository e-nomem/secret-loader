// Copyright (c) The secret-loader Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

#![doc(html_root_url = "https://docs.rs/secret-loader/0.0.1")]

mod credential;
mod error;
#[cfg(feature = "serde")]
mod serde;

pub use crate::credential::Credential;
pub use crate::error::Error;
#[cfg(feature = "serde")]
pub use crate::serde::deserialize_secret;
