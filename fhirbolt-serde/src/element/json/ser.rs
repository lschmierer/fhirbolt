use fhirbolt_shared::{element::Element, serde_context::ser::SerializationContext, FhirRelease};

use serde::Serialize;
use serde_json::{error::Result, Serializer};

/// Serialize the given resource as JSON into the IO stream.
pub fn to_writer<W, const FR: FhirRelease>(writer: W, value: &Element<FR>) -> Result<()>
where
    W: std::io::Write,
{
    SerializationContext::with_path_tracking(value, true).serialize(&mut Serializer::new(writer))
}

/// Serialize the given resource as pretty-printed JSON into the IO stream.
pub fn to_writer_pretty<W, const FR: FhirRelease>(writer: W, value: &Element<FR>) -> Result<()>
where
    W: std::io::Write,
{
    SerializationContext::with_path_tracking(value, true).serialize(&mut Serializer::pretty(writer))
}

/// Serialize the given resource as a JSON byte vector.
pub fn to_vec<const FR: FhirRelease>(value: &Element<FR>) -> Result<Vec<u8>> {
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value)?;
    Ok(writer)
}

/// Serialize the given resource as a pretty-printed JSON byte vector.
pub fn to_vec_pretty<const FR: FhirRelease>(value: &Element<FR>) -> Result<Vec<u8>> {
    let mut writer = Vec::with_capacity(128);
    to_writer_pretty(&mut writer, value)?;
    Ok(writer)
}

/// Serialize the given resource as a String of JSON.
pub fn to_string<const FR: FhirRelease>(value: &Element<FR>) -> Result<String> {
    let vec = to_vec(value)?;
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}

/// Serialize the given resource as a pretty-printed String of JSON.
pub fn to_string_pretty<const FR: FhirRelease>(value: &Element<FR>) -> Result<String> {
    let vec = to_vec_pretty(value)?;
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}

/// Serialize the given resource as a [`serde_json::Value`].
pub fn to_json_value<const FR: FhirRelease>(value: Element<FR>) -> Result<serde_json::Value> {
    SerializationContext::with_path_tracking(&value, true).serialize(serde_json::value::Serializer)
}
