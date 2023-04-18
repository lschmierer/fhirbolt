//! Conversion between element model and model structs.
//!
//! # Example
//! ```
//! // The `Resource` type is an enum that contains all possible FHIR resources.
//! // If the resource type is known in advance, you could also use a concrete resource type
//! // (like e.g. `fhirbolt::model::r4b::resources::Observation`).
//! use fhirbolt::FhirReleases;
//! use fhirbolt::element::Element;
//! use fhirbolt::model::r4b::Resource;
//! use fhirbolt::serde::{DeserializationConfig, DeserializationMode};
//!
//! // The type of `s` is `&str`
//! let s = "{
//!     \"resourceType\": \"Observation\",
//!     \"status\": \"final\",
//!     \"code\": {
//!         \"text\": \"some code\"
//!     },
//!     \"valueString\": \"some value\"
//! }";
//!
//! let e: Element<{ FhirReleases::R4B }> =  fhirbolt::json::from_str(s, None).unwrap();
//! let r: Resource = fhirbolt::element::from_element(e, None).unwrap();
//!
//! println!("{:?}", r);
//! ```
//!
//! See [`DeserializationMode`](crate::DeserializationMode) for different supported deserialization modes.

use fhirbolt_element::{Element, Value};
use fhirbolt_shared::FhirRelease;

use serde::{de::DeserializeSeed, Serialize};

use crate::{
    element::de::Deserializer, element::ser::Serializer, DeserializationConfig,
    DeserializeResourceOwned, Resource, SerializeResource,
};

pub mod de;
pub mod ser;

pub mod error;

pub(crate) mod internal;

impl<const R: FhirRelease> Resource for Element<R> {
    const FHIR_RELEASE: FhirRelease = R;
}

pub use error::{Error, Result};

/// Convert an element to an instance of resource type `T`.
///
/// # Example
/// ```
/// // The `Resource` type is an enum that contains all possible FHIR resources.
/// // If the resource type is known in advance, you could also use a concrete resource type
/// // (like e.g. `fhirbolt::model::r4b::resources::Observation`).
/// use fhirbolt::FhirReleases;
/// use fhirbolt::element::Element;
/// use fhirbolt::model::r4b::Resource;
/// use fhirbolt::serde::{DeserializationConfig, DeserializationMode};
///
/// // The type of `s` is `&str`
/// let s = "{
///     \"resourceType\": \"Observation\",
///     \"status\": \"final\",
///     \"code\": {
///         \"text\": \"some code\"
///     },
///     \"valueString\": \"some value\"
/// }";
///
/// let e: Element<{ FhirReleases::R4B }> =  fhirbolt::json::from_str(s, None).unwrap();
/// let r: Resource = fhirbolt::element::from_element(e, None).unwrap();
///
/// println!("{:?}", r);
/// ```
///
/// # Errors
/// The conversion can fail if the structure of the input does not match the FHIR resource `T`.
/// This behavior can be modified by passing a [`DeserializationConfig`](crate::DeserializationConfig).
pub fn from_element<'a, const R: FhirRelease, T>(
    element: Element<R>,
    config: Option<DeserializationConfig>,
) -> Result<T>
where
    T: DeserializeResourceOwned,
{
    T::deserialization_context(config.unwrap_or(Default::default()), false)
        .deserialize(Deserializer(element))
}

/// Convert the given resource as an element.
pub fn to_element<const R: FhirRelease, T>(resource: T) -> Result<Element<R>>
where
    T: SerializeResource,
{
    match resource
        .serialization_context(false)
        .serialize(Serializer)?
    {
        Value::Element(e) => Ok(e),
        Value::Sequence(_) => Err(serde::ser::Error::custom(
            "invalid sequence, expected an element".to_string(),
        )),
        Value::Primitive(_) => Err(serde::ser::Error::custom(
            "invalid primitive, expected an element".to_string(),
        )),
    }
}
