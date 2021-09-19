// Copyright (c) The secret-loader Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::env::VarError;
use std::error::Error as StdError;
use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;
use std::io::Error as IoError;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Io(IoError),
    Env(VarError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            Error::Io(_) => write!(f, "Io Error"),
            Error::Env(_) => write!(f, "Env Error"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::Env(e) => Some(e),
        }
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Error::Io(e)
    }
}

impl From<VarError> for Error {
    fn from(e: VarError) -> Self {
        Error::Env(e)
    }
}
