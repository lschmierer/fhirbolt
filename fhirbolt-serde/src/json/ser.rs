//! Serialize FHIR resources to JSON.

use std::io;

use serde::ser::Serialize;
use serde_json::Serializer;

use crate::{
    context::{ser::SerializationConfig, Format},
    json::Result,
    SerializeResource,
};

/// Serialize the given resource as JSON into the IO stream.
pub fn to_writer<W, T>(writer: W, value: &T, config: Option<SerializationConfig>) -> Result<()>
where
    W: io::Write,
    T: SerializeResource,
{
    value
        .serialization_context(config.unwrap_or(Default::default()), Format::Json)
        .serialize(&mut Serializer::new(writer))
}

/// Serialize the given resource as pretty-printed JSON into the IO stream.
pub fn to_writer_pretty<W, T>(
    writer: W,
    value: &T,
    config: Option<SerializationConfig>,
) -> Result<()>
where
    W: std::io::Write,
    T: SerializeResource,
{
    value
        .serialization_context(config.unwrap_or(Default::default()), Format::Json)
        .serialize(&mut Serializer::pretty(writer))
}

/// Serialize the given resource as a JSON byte vector.
pub fn to_vec<T>(value: &T, config: Option<SerializationConfig>) -> Result<Vec<u8>>
where
    T: SerializeResource,
{
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value, config)?;
    Ok(writer)
}

/// Serialize the given resource as a pretty-printed JSON byte vector.
pub fn to_vec_pretty<T>(value: &T, config: Option<SerializationConfig>) -> Result<Vec<u8>>
where
    T: SerializeResource,
{
    let mut writer = Vec::with_capacity(128);
    to_writer_pretty(&mut writer, value, config)?;
    Ok(writer)
}

/// Serialize the given resource as a String of JSON.
pub fn to_string<T>(value: &T, config: Option<SerializationConfig>) -> Result<String>
where
    T: SerializeResource,
{
    let vec = to_vec(value, config)?;
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}

/// Serialize the given resource as a pretty-printed String of JSON.
pub fn to_string_pretty<T>(value: &T, config: Option<SerializationConfig>) -> Result<String>
where
    T: SerializeResource,
{
    let vec = to_vec_pretty(value, config)?;
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}

/// Serialize the given resource as a [`serde_json::Value`].
pub fn to_json_value<T>(value: T, config: Option<SerializationConfig>) -> Result<serde_json::Value>
where
    T: SerializeResource,
{
    value
        .serialization_context(config.unwrap_or(Default::default()), Format::Json)
        .serialize(serde_json::value::Serializer)
}
