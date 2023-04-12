use serde::{de::DeserializeSeed, ser::Error, Serialize};

use fhirbolt_shared::{
    element::{Element, Value},
    FhirRelease,
};

use crate::{
    element::{self, de::Deserializer, error::Result},
    DeserializationConfig, DeserializeResource, Resource, SerializeResource,
};

pub mod de;
pub mod ser;

pub mod error;

impl<const R: FhirRelease> Resource for Element<R> {
    const FHIR_RELEASE: FhirRelease = R;
}

pub fn from_element<'a, const R: FhirRelease, T>(
    element: Element<R>,
    config: Option<DeserializationConfig>,
) -> Result<T>
where
    T: DeserializeResource,
{
    T::context(config.unwrap_or(Default::default()), false, T::FHIR_RELEASE)
        .deserialize(Deserializer(element))
}

pub fn to_element<const R: FhirRelease, T>(resource: T) -> element::error::Result<Element<R>>
where
    T: SerializeResource,
{
    match resource
        .context(false, R)
        .serialize(element::ser::Serializer)?
    {
        Value::Element(e) => Ok(e),
        Value::Sequence(_) => Err(element::error::Error::custom(
            "invalid sequence, expected an element".to_string(),
        )),
        Value::Primitive(_) => Err(element::error::Error::custom(
            "invalid primitive, expected an element".to_string(),
        )),
    }
}
