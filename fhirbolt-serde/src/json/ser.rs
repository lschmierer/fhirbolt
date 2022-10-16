/// Serialize FHIR resources to JSON.
use serde::ser::Serialize;

pub use serde_json::ser::*;

use fhirbolt_shared::AnyResource;

use crate::json::error::Result;

/// Serialize the given resource as JSON into the IO stream.
pub fn to_writer<W, T>(writer: W, value: &T) -> Result<()>
where
    W: std::io::Write,
    T: ?Sized + Serialize + AnyResource,
{
    serde_json::to_writer(writer, value)
}

/// Serialize the given resource as pretty=printed JSON into the IO stream.
pub fn to_writer_pretty<W, T>(writer: W, value: &T) -> Result<()>
where
    W: std::io::Write,
    T: ?Sized + serde::ser::Serialize + AnyResource,
{
    serde_json::to_writer_pretty(writer, value)
}

/// Serialize the given resource as a JSON byte vector.
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + serde::ser::Serialize + AnyResource,
{
    serde_json::to_vec(value)
}

/// Serialize the given resource as a pretty-printed JSON byte vector.
pub fn to_vec_pretty<T>(value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + serde::ser::Serialize + AnyResource,
{
    serde_json::to_vec_pretty(value)
}

/// Serialize the given resource as a String of JSON.
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: ?Sized + serde::ser::Serialize + AnyResource,
{
    serde_json::to_string(value)
}

/// Serialize the given resource as a pretty-printed String of JSON.
pub fn to_string_pretty<T>(value: &T) -> Result<String>
where
    T: ?Sized + serde::ser::Serialize + AnyResource,
{
    serde_json::to_string_pretty(value)
}

pub fn to_value<T>(value: T) -> Result<serde_json::Value>
where
    T: serde::ser::Serialize + AnyResource,
{
    serde_json::to_value(value)
}
