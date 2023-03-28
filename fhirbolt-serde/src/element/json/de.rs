use std::io;

use serde::de::DeserializeSeed;

use serde_json::{error::Result, Deserializer};

use fhirbolt_shared::{element::Element, serde_context::de::DeserializationContext, FhirRelease};

fn from_deserializer<'a, R, const FR: FhirRelease>(de: &mut Deserializer<R>) -> Result<Element<FR>>
where
    R: serde_json::de::Read<'a>,
{
    DeserializationContext::<Element<FR>>::with_path_tracking(true).deserialize(de)
}

pub fn from_reader<R, const FR: FhirRelease>(rdr: R) -> Result<Element<FR>>
where
    R: io::Read,
{
    from_deserializer(&mut Deserializer::from_reader(rdr))
}

pub fn from_slice<'a, const FR: FhirRelease>(v: &'a [u8]) -> Result<Element<FR>> {
    from_deserializer(&mut Deserializer::from_slice(v))
}

pub fn from_str<'a, const FR: FhirRelease>(s: &'a str) -> Result<Element<FR>> {
    from_deserializer(&mut Deserializer::from_str(s))
}

pub fn from_json_value<const FR: FhirRelease>(value: serde_json::Value) -> Result<Element<FR>> {
    DeserializationContext::<Element<FR>>::with_path_tracking(true).deserialize(value)
}
