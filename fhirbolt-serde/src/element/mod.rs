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
    context::Format,
    element::{de::Deserializer, ser::Serializer},
    DeserializationConfig, DeserializeResourceOwned, Resource, SerializeResource,
};

pub mod de;
pub mod ser;

pub mod error;

pub(crate) mod internal;

mod sort;

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
pub fn from_element<const R: FhirRelease, T>(
    element: Element<R>,
    config: Option<DeserializationConfig>,
) -> Result<T>
where
    T: DeserializeResourceOwned,
{
    T::deserialization_context(
        config.unwrap_or(Default::default()),
        Format::InternalElement,
    )
    .deserialize(Deserializer(element))
}

/// Convert the given resource as an element.
pub fn to_element<const R: FhirRelease, T>(resource: T) -> Result<Element<R>>
where
    T: SerializeResource,
{
    match resource
        .serialization_context(Default::default(), Format::InternalElement)
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

#[cfg(test)]
mod tests {
    use assert_json_diff::assert_json_eq;
    use test_utils::assert_xml_eq;

    use fhirbolt_element::{Element, Primitive, Value};
    use fhirbolt_shared::FhirReleases;

    fn assert(element: Element<{ FhirReleases::R4 }>, json: &str, xml: &str) {
        assert_json_eq!(
            crate::json::to_json_value(element.clone(), None).unwrap(),
            serde_json::from_str::<serde_json::Value>(json).unwrap()
        );
        assert_eq!(
            crate::json::from_str::<Element<{ FhirReleases::R4 }>>(json, None).unwrap(),
            element
        );

        assert_xml_eq(&crate::xml::to_string(&element, None).unwrap(), xml, false);
        assert_eq!(
            crate::xml::from_str::<Element<{ FhirReleases::R4 }>>(xml, None).unwrap(),
            element
        );

        assert_eq!(
            super::from_element::<{ FhirReleases::R4 }, Element<{ FhirReleases::R4 }>>(
                element.clone(),
                None
            )
            .unwrap(),
            element
        );
        assert_eq!(super::to_element(element.clone()).unwrap(), element);
    }

    #[test]
    fn test_resource_id() {
        assert(
            Element! {
                "resourceType" => Value::Primitive(Primitive::String("Observation".into())),
                "id" => Value::Element(Element! {
                    "value" => Value::Primitive(Primitive::String("test_id".into())),
                })
            },
            r#"{
                "resourceType": "Observation",
                "id": "test_id"
            }"#,
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <Observation xmlns="http://hl7.org/fhir">
                <id value="test_id"/>
            </Observation>"#,
        );
    }

    #[test]
    fn test_resource_contained() {
        assert(
            Element! {
                "resourceType" => Value::Primitive(Primitive::String("Observation".into())),
                "contained" =>  Value::Sequence(vec![Element! {
                    "resourceType" => Value::Primitive(Primitive::String("Observation".into())),
                    "identifier" => Value::Sequence(vec![Element! {
                        "value" => Value::Element(Element! { "value" => Value::Primitive(Primitive::String("123".into())) }),
                    }])
                }]),
                "identifier" => Value::Sequence(vec![Element! {
                    "value" => Value::Element(Element! { "value" => Value::Primitive(Primitive::String("123".into())) }),
                }])
            },
            r#"{
                "resourceType": "Observation",
                "contained": [
                    {
                        "resourceType": "Observation",
                        "identifier": [{ "value": "123" }]
                    }
                ],
                "identifier": [{ "value": "123" }]
            }"#,
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <Observation xmlns="http://hl7.org/fhir">
                <contained>
                    <Observation>
                        <identifier>
                            <value value="123"/>
                        </identifier>
                    </Observation>
                </contained>
                <identifier>
                    <value value="123"/>
                </identifier>
            </Observation>"#,
        );
    }

    #[test]
    fn test_element_id() {
        assert(
            Element! {
                "resourceType" => Value::Primitive(Primitive::String("Observation".into())),
                "valueString" => Value::Element(Element! {
                    "id" => Value::Primitive(Primitive::String("test_id".into())),
                })
            },
            r#"{
                "resourceType": "Observation",
                "_valueString": {
                    "id": "test_id"
                }
            }"#,
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <Observation xmlns="http://hl7.org/fhir">
                <valueString id="test_id"/>
            </Observation>"#,
        );
    }

    #[test]
    fn test_element_value() {
        assert(
            Element! {
                "resourceType" => Value::Primitive(Primitive::String("Observation".into())),
                "valueString" => Value::Element(Element! {
                    "value" => Value::Primitive(Primitive::String("test_value".into())),
                })
            },
            r#"{
                "resourceType": "Observation",
                "valueString": "test_value"
            }"#,
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <Observation xmlns="http://hl7.org/fhir">
                <valueString value="test_value"/>
            </Observation>"#,
        );
    }

    #[test]
    fn test_extension_url() {
        assert(
            Element! {
                "resourceType" => Value::Primitive(Primitive::String("Observation".into())),
                "extension" => Value::Sequence(vec![Element! {
                    "url" => Value::Primitive(Primitive::String("test_url".into())),
                }])
            },
            r#"{
                "resourceType": "Observation",
                "extension": [{ "url": "test_url" }]
            }"#,
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <Observation xmlns="http://hl7.org/fhir">
                <extension url="test_url"/>
            </Observation>"#,
        );
    }
}
