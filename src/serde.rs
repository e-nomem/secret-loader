// Copyright (c) The secret-loader Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::convert::TryInto;

use camino::Utf8Path;
use secrecy::Secret;
use serde::de::Error as DeError;
use serde::Deserialize;
use serde::Deserializer;

use crate::Credential;

pub fn deserialize_secret<'de, D>(deserializer: D) -> Result<Option<Secret<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    match <Option<Credential>>::deserialize(deserializer)? {
        Some(cred) => cred.try_into().map(Option::Some).map_err(DeError::custom),
        None => Ok(None),
    }
}

impl<'de> Deserialize<'de> for Credential {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let cred = match s {
            val if val.starts_with("env:") => Credential::Env((&val[4..]).into()),
            val if val.starts_with("file:") => Credential::File(Utf8Path::new(&val[5..]).into()),
            val => Credential::Plain(val.parse().unwrap()),
        };
        Ok(cred)
    }
}
