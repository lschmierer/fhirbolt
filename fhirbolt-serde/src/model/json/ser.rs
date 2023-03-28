//! Serialize FHIR resources to JSON.

use std::io;

use serde::ser::Serialize;

use fhirbolt_shared::{
    serde_context::ser::{with_context, SerializationContext},
    AnyResource,
};
use serde_json::Serializer;

use serde_json::error::Result;

/// Serialize the given resource as JSON into the IO stream.
pub fn to_writer<W, T>(writer: W, value: &T) -> Result<()>
where
    W: io::Write,
    T: ?Sized + Serialize + AnyResource,
{
    with_context(SerializationContext::without_path_tracking(true), || {
        value.serialize(&mut Serializer::new(writer))
    })
}

/// Serialize the given resource as pretty-printed JSON into the IO stream.
pub fn to_writer_pretty<W, T>(writer: W, value: &T) -> Result<()>
where
    W: std::io::Write,
    T: ?Sized + serde::ser::Serialize + AnyResource,
{
    with_context(SerializationContext::without_path_tracking(true), || {
        value.serialize(&mut Serializer::pretty(writer))
    })
}

/// Serialize the given resource as a JSON byte vector.
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + serde::ser::Serialize + AnyResource,
{
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value)?;
    Ok(writer)
}

/// Serialize the given resource as a pretty-printed JSON byte vector.
pub fn to_vec_pretty<T>(value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + serde::ser::Serialize + AnyResource,
{
    let mut writer = Vec::with_capacity(128);
    to_writer_pretty(&mut writer, value)?;
    Ok(writer)
}

/// Serialize the given resource as a String of JSON.
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: ?Sized + serde::ser::Serialize + AnyResource,
{
    let vec = to_vec(value)?;
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}

/// Serialize the given resource as a pretty-printed String of JSON.
pub fn to_string_pretty<T>(value: &T) -> Result<String>
where
    T: ?Sized + serde::ser::Serialize + AnyResource,
{
    let vec = to_vec_pretty(value)?;
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}

/// Serialize the given resource as a [`serde_json::Value`].
pub fn to_json_value<T>(value: T) -> Result<serde_json::Value>
where
    T: serde::ser::Serialize + AnyResource,
{
    with_context(SerializationContext::without_path_tracking(true), || {
        value.serialize(serde_json::value::Serializer)
    })
}
