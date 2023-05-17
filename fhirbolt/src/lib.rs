//! # Fhirbolt
//! Fhirbolt is a library that enables you to work with FHIR resources in Rust.
//! This library includes FHIR data types and methods for (de)serializing these from and to JSON and XML.
//!
//! More elaborate features like validation (including cardinality and slicing) or full FHIRPath evaluation
//! might be added eventually.
//!
//! Currenlty supported FHIR releases:
//!   * R4
//!   * R4B
//!   * R5
//!
//! The Rust crate supports two working modes:
//! 1. a generic [element] model
//! 2. working with fully typed model structs.
//!
//! ## Installation
//! Add `fhirbolt` to your Cargo.toml.
//! You can select which FHIR release to include model structs for by speficifying them as Cargo features.
//!
//! ```toml
//! [dependencies]
//! fhirbolt = { version = "0.2", features = ["r4b"] }
//! ```
//! By default, no FHIR release is included.
//!
//! ## Example
//! ```
//! // The `Resource` type is an enum that contains all possible FHIR resources.
//! // If the resource type is known in advance, you could also use a concrete resource type
//! // (like e.g. `fhirbolt::model::r4b::resources::Observation`).
//! use fhirbolt::model::r4b::{
//!     Resource,
//!     resources::{Observation, ObservationValue},
//!     types::{Code, CodeableConcept, Coding, String as FhirString},
//! };
//! use fhirbolt::serde::{DeserializationConfig, DeserializationMode};
//!
//! // The type of `s` is `&str`
//! let s = r#"{
//!     "resourceType": "Observation",
//!     "status": "final",
//!     "code": {
//!         "text": "some code"
//!     },
//!     "valueString": "some value"
//! }"#;
//!
//! let r: Resource = fhirbolt::json::from_str(s, None).unwrap();
//!
//! match r {
//!     Resource::Observation(ref o) => println!("deserialized observation: {:?}", r),
//!     _ => (),
//! }
//!
//! // Use Default::default() or constructing new resources by yourself
//! let o = Observation {
//!     status: Code {
//!         value: Some("final".to_string()),
//!         ..Default::default()
//!     },
//!     code: Box::new(CodeableConcept {
//!         text: Some(FhirString {
//!             value: Some("some code".to_string()),
//!             ..Default::default()
//!         }),
//!         ..Default::default()
//!     }),
//!     value: Some(ObservationValue::String(Box::new(FhirString {
//!         value: Some("some value".to_string()),
//!         ..Default::default()
//!     }))),
//!     ..Default::default()
//! };
//! ```
//!
//! You can pass a [`DeserializationConfig`](crate::serde::DeserializationConfig) to configure the deserialization behavior.

pub use fhirbolt_element::{FhirRelease, FhirReleases};

#[cfg(feature = "fhirbolt-model")]
pub mod model {
    //! Generated structs for FHIR resources.
    //!
    //! You can select which FHIR release to include by speficifying them as Cargo features.
    //! ```toml
    //! [dependencies]
    //! fhirbolt = { version = "0.1", features = ["r4", "r4b"] }
    //! ```
    //! By default, no FHIR release is included.

    pub use fhirbolt_model::*;
}

pub mod element {
    //! Generic element model.
    //!
    //! As deserialization differs slightly between FHIR releases,
    //! `Element` is generic over a FHIR release.
    //!
    //! # Example
    //! ```
    //! use fhirbolt::FhirReleases;
    //! use fhirbolt::element::{Element, Value, Primitive};
    //!
    //! let mut element = Element::<{ FhirReleases:: R4B }>::new();
    //! element.insert(
    //!     "resourceType".to_string(),
    //!     Value::Primitive(
    //!         Primitive::String("Observation".to_string())
    //!     )
    //! );
    //!
    //! // This can be simplified by using the Element! macro provided
    //! let element: Element::<{ FhirReleases:: R4B }> = Element! {
    //!     "resourceType" => Value::Primitive(Primitive::String("Observation".to_string()))
    //! };
    //! ```

    pub use fhirbolt_element::*;

    #[doc(no_inline)]
    pub use crate::serde::element::{from_element, to_element, Error, Result};
}

pub mod serde {
    //! (De)serialize FHIR resources from and to JSON and XML.
    //!
    //! # Example
    //! ```
    //! // The `Resource` type is an enum that contains all possible FHIR resources.
    //! // If the resource type is known in advance, you could also use a concrete resource type
    //! // (like e.g. `fhirbolt::model::r4b::resources::Observation`).
    //! use fhirbolt::model::r4b::Resource;
    //!
    //! // The type of `s` is `&str`
    //! let s = r#"{
    //!     "resourceType": "Observation",
    //!     "status": "final",
    //!     "code": {
    //!         "text": "some code"
    //!     },
    //!     "valueString": "some value"
    //! }"#;
    //!
    //! let r: Resource = fhirbolt::json::from_str(s, None).unwrap();
    //! println!("{:?}", r);
    //! ```

    pub use fhirbolt_serde::*;
}

#[doc(no_inline)]
pub use serde::{json, xml};
