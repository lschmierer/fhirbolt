//! Deserialize and serialize FHIR resources to and from JSON.
//!
//! This module uses `serde_json` under the hood and reexports its [`Error`] type.
//!
//! # Example
//! ```
//! # fn main() {
//! // The `Resource` type is an enum that contains all possible FHIR resources.
//! // If the resource type is known in advance, you could also use a concrete resource type
//! // (like e.g. `fhirbolt::model::r4b::resources::Observation`).
//! use fhirbolt::model::r4b::Resource as R4BResource;
//! use fhirbolt::{DeserializationConfig, DeserializationMode};
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
mod de;
mod error;
mod ser;

pub use de::{from_json_value, from_reader, from_slice, from_str};
pub use ser::{
    to_json_value, to_string, to_string_pretty, to_vec, to_vec_pretty, to_writer, to_writer_pretty,
};

pub use error::{Error, Result};
