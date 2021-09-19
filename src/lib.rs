// Copyright (c) The secret-loader Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

mod credential;
mod error;
#[cfg(feature = "serde")]
mod serde;

pub use crate::credential::Credential;
pub use crate::error::Error;
#[cfg(feature = "serde")]
pub use crate::serde::deserialize_secret;
