use serde::de::DeserializeSeed;
use std::io;

use fhirbolt_shared::{element::Element, serde_context::de::DeserializationContext, FhirRelease};

use crate::xml::{de::Deserializer, error::Result, read::Read};

fn from_deserializer<R, const FR: FhirRelease>(de: &mut Deserializer<R>) -> Result<Element<FR>>
where
    R: Read,
{
    DeserializationContext::<Element<FR>>::with_path_tracking(false).deserialize(de)
}

pub fn from_reader<R: io::Read, const FR: FhirRelease>(rdr: R) -> Result<Element<FR>> {
    from_deserializer(&mut Deserializer::from_reader(rdr, FR)?)
}

pub fn from_slice<const FR: FhirRelease>(v: &[u8]) -> Result<Element<FR>>
where
{
    from_deserializer(&mut Deserializer::from_slice(v, FR)?)
}

pub fn from_str<'a, const FR: FhirRelease>(s: &'a str) -> Result<Element<FR>> {
    from_deserializer(&mut Deserializer::from_str(s, FR)?)
}
