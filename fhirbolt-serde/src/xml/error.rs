//! Errors when (de)serializing to/from XML.

use std::{
    fmt::{self, Display},
    str,
};

use serde::{de, ser};

mod quick_xml {
    pub use quick_xml::{events::attributes::AttrError, Error};
}

/// Alias for a Result with the error type [`fhirbolt::serde::xml::Error`](Error).
pub type Result<T> = std::result::Result<T, Error>;

/// This type represents all possible errors that can occur when serializing or deserializing XML data.
#[derive(Debug)]
pub enum Error {
    /// Generic error message.
    Message(String),
    /// Error reading or writing XML.
    InvalidXml(quick_xml::Error),
    /// Invalid XML attribute.
    InvalidXmlAttribute(quick_xml::AttrError),
    /// Invalid XML version.
    InvalidXmlVersion(String),
    /// Invalid XML encoding.
    InvalidXmlEncoding(String),
    /// Invalid XML standalone.
    InvalidXmlStandalone(String),
    /// Invalid XML namespace.
    InvalidXmlNamespace(Option<String>, String),
    /// Unsupported XML event.
    InvalidXmlEvent(&'static str),
    /// Unexpected Fhir event.
    InvalidFhirEvent {
        found: &'static str,
        expected: &'static str,
    },
    /// Error reading UTF8.
    Utf8Error(str::Utf8Error),
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => write!(f, "{}", msg),
            Error::InvalidXml(e) => write!(f, "{}", e),
            Error::InvalidXmlAttribute(e) => write!(f, "{}", e),
            Error::InvalidXmlVersion(v) => {
                write!(f, "invalid XML version '{}' (expected '1.0')", v)
            }
            Error::InvalidXmlEncoding(e) => {
                write!(f, "invalid XML encoding '{}' (expected 'UTF-8')", e)
            }
            Error::InvalidXmlStandalone(s) => {
                write!(f, "invalid XML standalone '{}' (expected 'no')", s)
            }
            Error::InvalidXmlNamespace(None, expected) => {
                write!(f, "invalid XML unbound namespace (expected '{}')", expected)
            }
            Error::InvalidXmlNamespace(Some(ns), expected) => {
                write!(f, "invalid XML namespace {} (expected '{}')", ns, expected)
            }
            Error::InvalidXmlEvent(e) => write!(f, "invalid XML event: {}", e),
            Error::InvalidFhirEvent { found, expected } => {
                write!(
                    f,
                    "invalid FHIR event: found {}, expected: {}",
                    found, expected
                )
            }
            Error::Utf8Error(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<quick_xml::Error> for Error {
    fn from(e: quick_xml::Error) -> Self {
        Self::InvalidXml(e)
    }
}

impl From<quick_xml::AttrError> for Error {
    fn from(e: quick_xml::AttrError) -> Self {
        Self::InvalidXmlAttribute(e)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(e: str::Utf8Error) -> Self {
        Self::Utf8Error(e)
    }
}
