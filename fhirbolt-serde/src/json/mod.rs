//! Deserialize and serialize FHIR resources to and from JSON.
//!
//! # Example
//! ```
//! // The `Resource` type is an enum that contains all possible FHIR resources.
//! // If the resource type is known in advance, you could also use a concrete resource type
//! // (like e.g. `fhirbolt::model::r4b::resources::Observation`).
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
//! let r: Resource = fhirbolt::json::from_str(s, Some(DeserializationConfig {
//!     mode: DeserializationMode::Lax,
//! })).unwrap();
//! println!("{:?}", r);
//! ```
//!
//! See [`DeserializationMode`](crate::DeserializationMode) for different supported deserialization modes.

pub mod de;
pub mod ser;

pub mod error;

pub use de::{from_json_value, from_reader, from_slice, from_str};
pub use ser::{
    to_json_value, to_string, to_string_pretty, to_vec, to_vec_pretty, to_writer, to_writer_pretty,
};

pub use error::{Error, Result};
