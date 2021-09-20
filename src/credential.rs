// Copyright (c) The secret-loader Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::convert::Infallible;
use std::convert::TryFrom;
use std::env;
use std::fs;
use std::str::FromStr;

use camino::Utf8PathBuf;
use secrecy::Secret;

use crate::error::Error;

#[derive(Debug, Clone)]
pub enum Credential {
    Env(String),
    File(Utf8PathBuf),
    Plain(Secret<String>),
}

impl Credential {
    pub fn is_env(&self) -> bool {
        matches!(self, Credential::Env(_))
    }

    pub fn is_file(&self) -> bool {
        matches!(self, Credential::File(_))
    }

    pub fn is_plain(&self) -> bool {
        matches!(self, Credential::Plain(_))
    }
}

impl FromStr for Credential {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cred = match s {
            val if val.starts_with("env:") => Credential::Env(val[4..].to_owned()),
            val if val.starts_with("file:") => Credential::File(val[5..].parse()?),
            val => Credential::Plain(val.parse()?),
        };
        Ok(cred)
    }
}

impl From<String> for Credential {
    fn from(s: String) -> Self {
        s.parse().expect("Infallible")
    }
}

impl TryFrom<Credential> for Secret<String> {
    type Error = Error;

    fn try_from(value: Credential) -> Result<Self, Self::Error> {
        let secret = match value {
            Credential::Env(env_var) => env::var(env_var)?.parse().expect("Infallible"),
            Credential::File(path) => fs::read_to_string(path)?.parse().expect("Infallible"),
            Credential::Plain(secret) => secret,
        };
        Ok(secret)
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
            Credential::Env(env_var) => {
                assert_eq!(env_var, "SECRET");
            }
            _ => panic!("Wrong credential type"),
        }
    }

    #[test]
    fn parse_file() {
        let cred = "file:/home/user/.secrets".parse().unwrap();
        match cred {
            Credential::File(path) => {
                assert_eq!(path, "/home/user/.secrets");
            }
            _ => panic!("Wrong credential type"),
        }
    }

    #[test]
    fn parse_plain() {
        let cred = "plaincredentialstorageisbad".parse().unwrap();
        match cred {
            Credential::Plain(secret) => {
                assert_eq!(secret.expose_secret(), "plaincredentialstorageisbad");
            }
            _ => panic!("Wrong credential type"),
        }
    }

    #[test]
    #[serial(Env)]
    fn secret_from_env_present() {
        let cred: Credential = "env:SECRET".parse().unwrap();

        setup_env(Some("superenvsecret"));
        assert!(env_is_set());

        let secret: Secret<String> = cred.try_into().unwrap();
        assert_eq!(secret.expose_secret(), "superenvsecret");
    }

    #[test]
    #[serial(Env)]
    fn secret_from_env_missing() {
        let cred: Credential = "env:SECRET".parse().unwrap();

        setup_env(None);
        assert!(!env_is_set());

        let secret: Result<Secret<String>, _> = cred.try_into();

        assert!(matches!(secret.unwrap_err(), Error::Env(_)));
    }

    #[test]
    fn secret_from_file_present() {
        let mut tempfile = NamedTempFile::new().unwrap();
        write!(tempfile, "superfilesecret").unwrap();
        let tempfile = tempfile.into_temp_path();

        let cred: Credential = format!("file:{}", tempfile.display()).parse().unwrap();
        let secret: Secret<String> = cred.try_into().unwrap();

        assert_eq!(secret.expose_secret(), "superfilesecret");
        tempfile.close().unwrap();
    }

    #[test]
    fn secret_from_file_missing() {
        let cred: Credential = "file:/does/not/exist".parse().unwrap();

        let secret: Result<Secret<String>, _> = cred.try_into();

        assert!(matches!(secret.unwrap_err(), Error::Io(_)));
    }

    #[test]
    fn secret_from_plain() {
        let cred: Credential = "plaincredentialstorageisbad".parse().unwrap();
        let secret: Secret<String> = cred.try_into().unwrap();

        assert_eq!(secret.expose_secret(), "plaincredentialstorageisbad");
    }
}
