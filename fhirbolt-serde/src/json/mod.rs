//! Deserialize and serialize FHIR resources to and from JSON.
//!
//! This module uses `serde_json` for (de)serialization and directly reexports some of its types (like [`Error`] and [`Deserializer`]).
//! While you could use `serde_json`s `from_` and `to_` directly,
//! the functions in this module allow to pass a [`DeserializationConfig`](crate::DeserializationConfig)
//! to configure (de)serialization behavior.
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
//! let r: R4BResource = fhirbolt::json::from_str(s, Some(DeserializationConfig {
//!     mode: DeserializationMode::Lax,
//! })).unwrap();
//! println!("{:?}", r);
//! # }
//! ```
//!
//! See [`DeserializationMode`](crate::DeserializationMode) for different supported deserialization modes.
pub mod de;
pub mod error;
pub mod ser;

pub use de::{from_reader, from_slice, from_str, from_value, Deserializer};
pub use ser::{
    to_string, to_string_pretty, to_value, to_vec, to_vec_pretty, to_writer, to_writer_pretty,
    Serializer,
};

pub use error::{Error, Result};
