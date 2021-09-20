// Copyright (c) The secret-loader Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::convert::Infallible;
use std::convert::TryFrom;
use std::env;
use std::fs;
use std::str::FromStr;

use camino::Utf8PathBuf;
use secrecy::Secret;

use crate::error::LoadError;

/// A type that can load secrets from multiple locations
///
/// This enum is the main type of this crate and represents a secret
/// that can be loaded from multiple locations. This type can be converted
/// to a [`SecretString`](secrecy::SecretString) by using [`SecretLoader::into_secret`]
/// or one of the `TryFrom<_>`/`TryInto<_>` implementations.
#[derive(Debug, Clone)]
pub enum SecretLoader {
    /// A secret that will be loaded from an environment variable
    Env(String),
    /// A secret that will be loaded from a file
    File(Utf8PathBuf),
    /// A plaintext secret
    Plain(Secret<String>),
}

impl SecretLoader {
    /// Constructs a new `SecretLoader` from a provided str.
    ///
    /// # Examples
    ///
    /// ```
    /// use secret_loader::SecretLoader;
    ///
    /// let env_cred = SecretLoader::new("env:SECRET");
    /// let file_cred = SecretLoader::new("file:/some/file/path");
    /// let plain_cred = SecretLoader::new("plaintextpasswordsarebad");
    /// ```
    pub fn new<S: AsRef<str>>(val: S) -> Self {
        val.as_ref().parse().unwrap()
    }

    /// Converts a `SecretLoader` into a [`SecretString`](secrecy::SecretString)
    ///
    /// Use this method to actually 'load' or 'resolve' a usable `Secret`
    pub fn into_secret(self) -> Result<Secret<String>, LoadError> {
        let secret = match self {
            Self::Env(env_var) => env::var(env_var)?.parse().expect("Infallible"),
            Self::File(path) => fs::read_to_string(path)?.parse().expect("Infallible"),
            Self::Plain(secret) => secret,
        };
        Ok(secret)
    }

    /// Returns true if the secret will be loaded from an environment variable.
    ///
    /// ```
    /// # use secret_loader::SecretLoader;
    /// assert!(SecretLoader::new("env:SECRET").is_env());
    /// ```
    pub fn is_env(&self) -> bool {
        matches!(self, Self::Env(_))
    }

    /// Returns true if the secret will be loaded from a file.
    ///
    /// ```
    /// # use secret_loader::SecretLoader;
    /// assert!(SecretLoader::new("file:/some/file/path").is_file());
    /// ```
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File(_))
    }

    /// Returns true if the secret is in plaintext.
    ///
    /// ```
    /// # use secret_loader::SecretLoader;
    /// assert!(SecretLoader::new("plaintextpasswordsarebad").is_plain());
    /// ```
    pub fn is_plain(&self) -> bool {
        matches!(self, Self::Plain(_))
    }
}

impl FromStr for SecretLoader {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cred = match s {
            val if val.starts_with("env:") => Self::Env(val[4..].to_owned()),
            val if val.starts_with("file:") => Self::File(val[5..].parse()?),
            val => Self::Plain(val.parse()?),
        };
        Ok(cred)
    }
}

impl From<String> for SecretLoader {
    fn from(s: String) -> Self {
        s.parse().expect("Infallible")
    }
}

impl TryFrom<SecretLoader> for Secret<String> {
    type Error = LoadError;

    fn try_from(value: SecretLoader) -> Result<Self, Self::Error> {
        value.into_secret()
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;
    use std::env;
    use std::io::Write;

    use secrecy::ExposeSecret;
    use serial_test::serial;
    use tempfile::NamedTempFile;

    use super::*;

    fn setup_env(value: Option<&str>) {
        match value {
            Some(value) => env::set_var("SECRET", value),
            None => env::remove_var("SECRET"),
        }
    }

    fn env_is_set() -> bool {
        env::var("SECRET").is_ok()
    }

    #[test]
    fn parse_env() {
        let cred = "env:SECRET".parse().unwrap();
        match cred {
            SecretLoader::Env(env_var) => {
                assert_eq!(env_var, "SECRET");
            }
            _ => panic!("Wrong loader type"),
        }
    }

    #[test]
    fn parse_file() {
        let cred = "file:/home/user/.secrets".parse().unwrap();
        match cred {
            SecretLoader::File(path) => {
                assert_eq!(path, "/home/user/.secrets");
            }
            _ => panic!("Wrong loader type"),
        }
    }

    #[test]
    fn parse_plain() {
        let cred = "plaincredentialstorageisbad".parse().unwrap();
        match cred {
            SecretLoader::Plain(secret) => {
                assert_eq!(secret.expose_secret(), "plaincredentialstorageisbad");
            }
            _ => panic!("Wrong loader type"),
        }
    }

    #[test]
    #[serial(Env)]
    fn secret_from_env_present() {
        let cred: SecretLoader = "env:SECRET".parse().unwrap();

        setup_env(Some("superenvsecret"));
        assert!(env_is_set());

        let secret: Secret<String> = cred.try_into().unwrap();
        assert_eq!(secret.expose_secret(), "superenvsecret");
    }

    #[test]
    #[serial(Env)]
    fn secret_from_env_missing() {
        let cred: SecretLoader = "env:SECRET".parse().unwrap();

        setup_env(None);
        assert!(!env_is_set());

        let secret: Result<Secret<String>, _> = cred.try_into();

        assert!(matches!(secret.unwrap_err(), LoadError::Env(_)));
    }

    #[test]
    fn secret_from_file_present() {
        let mut tempfile = NamedTempFile::new().unwrap();
        write!(tempfile, "superfilesecret").unwrap();
        let tempfile = tempfile.into_temp_path();

        let cred: SecretLoader = format!("file:{}", tempfile.display()).parse().unwrap();
        let secret: Secret<String> = cred.try_into().unwrap();

        assert_eq!(secret.expose_secret(), "superfilesecret");
        tempfile.close().unwrap();
    }

    #[test]
    fn secret_from_file_missing() {
        let cred: SecretLoader = "file:/does/not/exist".parse().unwrap();

        let secret: Result<Secret<String>, _> = cred.try_into();

        assert!(matches!(secret.unwrap_err(), LoadError::Io(_)));
    }

    #[test]
    fn secret_from_plain() {
        let cred: SecretLoader = "plaincredentialstorageisbad".parse().unwrap();
        let secret: Secret<String> = cred.try_into().unwrap();

        assert_eq!(secret.expose_secret(), "plaincredentialstorageisbad");
    }
}
