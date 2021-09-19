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
