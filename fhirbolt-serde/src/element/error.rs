//! Errors when converting between element model and structs.
use std::{error, fmt, result};

use serde::{de, ser};

/// Error that can happen when converting between element model and structs.
#[derive(Debug)]
pub struct Error(pub(crate) String);

/// Alias for a Result with the error type [`fhirbolt::serde::element::Error`](Error).
pub type Result<T> = result::Result<T, Error>;

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error(msg.to_string())
    }
}
