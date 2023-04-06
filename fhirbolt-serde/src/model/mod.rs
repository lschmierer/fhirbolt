use serde::{ser::Error, Deserialize, Serialize};

use fhirbolt_shared::{
    element::{self, Element, Value},
    serde_context::{
        de::{self, DeserializationConfig, DeserializationContext},
        ser::{self, SerializationContext},
    },
    AnyResource, FhirRelease,
};

pub mod json;
pub mod xml;

pub fn from_element<'a, const R: FhirRelease, T>(
    element: Element<R>,
    config: Option<DeserializationConfig>,
) -> element::error::Result<T>
where
    T: AnyResource + Deserialize<'a>,
{
    de::with_context(
        DeserializationContext::without_path_tracking(config.unwrap_or_default(), false),
        || T::deserialize(element),
    )
}

pub fn to_element<const R: FhirRelease, T>(resource: T) -> element::error::Result<Element<R>>
where
    T: AnyResource + Serialize,
{
    ser::with_context(
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
