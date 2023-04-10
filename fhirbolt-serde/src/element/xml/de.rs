use serde::de::DeserializeSeed;
use std::io;

use fhirbolt_shared::{element::Element, FhirRelease};

use crate::{
    serde_context::de::DeserializationContext,
    xml::{de::Deserializer, error::Result, read::Read},
};

fn from_deserializer<R, const F: FhirRelease>(de: &mut Deserializer<R>) -> Result<Element<F>>
where
    R: Read,
{
    DeserializationContext::<Element<F>>::new(Default::default(), true, F).deserialize(de)
}

pub fn from_reader<R: io::Read, const F: FhirRelease>(rdr: R) -> Result<Element<F>> {
    from_deserializer(&mut Deserializer::from_reader(rdr, F)?)
}

pub fn from_slice<const F: FhirRelease>(v: &[u8]) -> Result<Element<F>>
where
{
    from_deserializer(&mut Deserializer::from_slice(v, F)?)
}

pub fn from_str<'a, const F: FhirRelease>(s: &'a str) -> Result<Element<F>> {
    from_deserializer(&mut Deserializer::from_str(s, F)?)
}
