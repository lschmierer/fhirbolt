//! (De)serialize FHIR resources from and to JSON and XML.
//!
//! # Example
//! ```
//! # fn main() {
//! // The `Resource` type is an enum that contains all possible FHIR resources.
//! // If the resource type is known in advance, you could also use a concrete resource type
//! // (like e.g. `fhirbolt::model::r4b::resources::Observation`).
//! use fhirbolt::model::r4b::Resource as R4BResource;
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

#![feature(adt_const_params)]

pub mod element;
// TODO: remove after migrating model to SerializationContext
pub mod model;

pub mod json;
pub mod xml;

mod context;
mod generated;
mod number;

pub use context::{de::*, ser::*};
pub use generated::*;
