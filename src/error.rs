// Copyright (c) The secret-loader Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::env::VarError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;
use std::io::Error as IoError;

/// A possible error value while loading a `Secret` from a [`SecretLoader`](crate::SecretLoader)
///
/// Produced by the `TryFrom<SecretLoader>` implementation for [`SecretString`](secrecy::SecretString)
///
/// # Examples
///
/// ```
/// use std::convert::TryFrom;
/// use std::convert::TryInto;
///
/// use secrecy::SecretString;
/// use secret_loader::LoadError;
/// use secret_loader::SecretLoader;
///
/// let plain_cred = SecretLoader::new("insecurepassword");
/// let secret: SecretString = plain_cred.try_into().expect("Plaintext credentials always convert");
///
/// // Env var may be missing
/// let env_cred = SecretLoader::new("env:MISSING_KEY");
/// let env_error = SecretString::try_from(env_cred).expect_err("Env var is not set");
/// assert!(matches!(env_error, LoadError::Env(_)));
///
/// // File may not available
/// let file_cred = SecretLoader::new("file:/does/not/exist");
/// let file_error = SecretString::try_from(file_cred).expect_err("File is missing");
/// assert!(matches!(file_error, LoadError::Io(_)));
/// ```
#[derive(Debug)]
#[non_exhaustive]
pub enum LoadError {
    /// An IO error was encountered while attempting to read from a file
    Io(IoError),
    /// A `VarError` was encountered while attempting to read from the environment
    Env(VarError),
}

impl Display for LoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            Self::Io(_) => write!(f, "Io Error"),
            Self::Env(_) => write!(f, "Env Error"),
        }
    }
}

impl Error for LoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::Env(e) => Some(e),
        }
    }
}

impl From<IoError> for LoadError {
    fn from(e: IoError) -> Self {
        Self::Io(e)
    }
}

impl From<VarError> for LoadError {
    fn from(e: VarError) -> Self {
        Self::Env(e)
    }
}
