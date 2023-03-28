use std::io;

use serde::ser::Serialize;

use fhirbolt_shared::{element::Element, serde_context::ser::SerializationContext, FhirRelease};

use crate::xml::{error::Result, ser::Serializer};

/// Serialize the given resource as XML into the IO stream.
pub fn to_writer<W, const FR: FhirRelease>(writer: W, value: &Element<FR>) -> Result<()>
where
    W: io::Write,
{
    SerializationContext::with_path_tracking(value, false).serialize(&mut Serializer::new(writer))
}

/// Serialize the given resource as a XML byte vector.
pub fn to_vec<const FR: FhirRelease>(value: &Element<FR>) -> Result<Vec<u8>> {
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value)?;
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
