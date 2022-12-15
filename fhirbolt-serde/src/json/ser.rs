/// Serialize FHIR resources to JSON.
use serde::ser::Serialize;

use fhirbolt_shared::{
    serde_context::ser::{with_context, SerializationContext},
    AnyResource,
};

use crate::json::error::Result;

/// Serialize the given resource as JSON into the IO stream.
pub fn to_writer<W, T>(writer: W, value: &T) -> Result<()>
where
    W: std::io::Write,
    T: ?Sized + Serialize + AnyResource,
{
    with_context(SerializationContext { output_json: true }, || {
        serde_json::to_writer(writer, value)
    })
}

/// Serialize the given resource as pretty-printed JSON into the IO stream.
pub fn to_writer_pretty<W, T>(writer: W, value: &T) -> Result<()>
where
    W: std::io::Write,
    T: ?Sized + serde::ser::Serialize + AnyResource,
{
    with_context(SerializationContext { output_json: true }, || {
        serde_json::to_writer_pretty(writer, value)
    })
}

/// Serialize the given resource as a JSON byte vector.
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + serde::ser::Serialize + AnyResource,
{
    with_context(SerializationContext { output_json: true }, || {
        serde_json::to_vec(value)
    })
}

/// Serialize the given resource as a pretty-printed JSON byte vector.
pub fn to_vec_pretty<T>(value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + serde::ser::Serialize + AnyResource,
{
    with_context(SerializationContext { output_json: true }, || {
        serde_json::to_vec_pretty(value)
    })
}

/// Serialize the given resource as a String of JSON.
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: ?Sized + serde::ser::Serialize + AnyResource,
{
    with_context(SerializationContext { output_json: true }, || {
        serde_json::to_string(value)
    })
}

/// Serialize the given resource as a pretty-printed String of JSON.
pub fn to_string_pretty<T>(value: &T) -> Result<String>
where
    T: ?Sized + serde::ser::Serialize + AnyResource,
{
    with_context(SerializationContext { output_json: true }, || {
        serde_json::to_string_pretty(value)
    })
}

/// Serialize the given resource as a [`serde_json::Value`].
pub fn to_json_value<T>(value: T) -> Result<serde_json::Value>
where
    T: serde::ser::Serialize + AnyResource,
{
    with_context(SerializationContext { output_json: true }, || {
        serde_json::to_value(value)
    })
}
