use serde::Deserialize;

use fhirbolt_shared::{
    element::{self, Element},
    serde_context::de::{with_context, DeserializationConfig, DeserializationContext},
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
    with_context(
        DeserializationContext::without_path_tracking(config.unwrap_or_default(), false),
        || T::deserialize(element),
    )
}
