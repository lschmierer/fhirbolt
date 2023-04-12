use serde::{de::DeserializeSeed, ser::Error, Serialize};

use fhirbolt_shared::{
    element::{Element, Value},
    serde_context::ser::SerializationContext,
    FhirRelease,
};

use crate::{
    element::{self, de::Deserializer, error::Result},
    DeserializationConfig, DeserializeResource, Resource,
};

pub mod json;
pub mod xml;

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
    T::new_context(config.unwrap_or(Default::default()), false, T::FHIR_RELEASE)
        .deserialize(Deserializer(element))
}

pub fn to_element<const R: FhirRelease, T>(resource: T) -> element::error::Result<Element<R>>
where
    T: Resource + Serialize,
{
    fhirbolt_shared::serde_context::ser::with_context(
        SerializationContext::without_path_tracking(false),
        || match resource.serialize(element::ser::Serializer)? {
            Value::Element(e) => Ok(e),
            Value::Sequence(_) => Err(element::error::Error::custom(
                "invalid sequence, expected an element".to_string(),
            )),
            Value::Primitive(_) => Err(element::error::Error::custom(
                "invalid primitive, expected an element".to_string(),
            )),
        },
    )
}
