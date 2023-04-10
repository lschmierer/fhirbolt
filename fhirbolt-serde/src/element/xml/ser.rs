use std::io;

use serde::ser::Serialize;

use fhirbolt_shared::{element::Element, FhirRelease};

use crate::{
    context::ser::SerializationContext,
    xml::{error::Result, ser::Serializer},
};

/// Serialize the given resource as XML into the IO stream.
pub fn to_writer<W, const F: FhirRelease>(writer: W, value: &Element<F>) -> Result<()>
where
    W: io::Write,
{
    SerializationContext::new(value, false, F).serialize(&mut Serializer::new(writer))
}

/// Serialize the given resource as a XML byte vector.
pub fn to_vec<const F: FhirRelease>(value: &Element<F>) -> Result<Vec<u8>> {
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value)?;
    Ok(writer)
}

/// Serialize the given resource as a String of JSON.
pub fn to_string<const F: FhirRelease>(value: &Element<F>) -> Result<String> {
    let vec = to_vec(value)?;
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}
