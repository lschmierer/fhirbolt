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
//!
//! # Installation
//! Add `fhirbolt` to your Cargo.toml.
//! You can select which FHIR release to include by speficifying them as Cargo features.
//! ```toml
//! [dependencies]
//! fhirbolt = { version = "0.1", features = ["r4b"] }
//! ```
//! By default, no FHIR release is included.
//!
//! # Example
//! ```
//! # fn main() {
//! // The `Resource` type is an enum that contains all possible FHIR resources.
//! // If the resource type is known in advance, you could also use a concrete resource type
//! // (like e.g. `fhirbolt::model::r4b::resources::Observation`).
//! use fhirbolt::model::r4b::Resource as R4BResource;
//! use fhirbolt::serde::{DeserializationConfig, DeserializationMode};
//!
//! // The type of `s` is `&str`
//! let s = "{
//!         \"resourceType\": \"Observation\",
//!         \"status\": \"final\",
//!         \"code\": {
//!             \"text\": \"some code\"
//!         },
//!         \"valueString\": \"some value\"
//!     }";
//!
//! let r: R4BResource = fhirbolt::json::from_str(s, None).unwrap();
//! println!("{:?}", r);
//! # }
//! ```
//!
//! You can pass a [`DeserializationConfig`](crate::DeserializationConfig) to configure the deserialization behavior.

#[doc(inline)]
pub use fhirbolt_model as model;
#[doc(inline)]
pub use fhirbolt_serde as serde;

#[doc(no_inline)]
pub use serde::json;
#[doc(no_inline)]
pub use serde::xml;
