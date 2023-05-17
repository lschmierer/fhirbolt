//! (De)serialize FHIR resources from and to JSON and XML.

// This was taken from serde_json to reduce code size by avoiding going
// through From (as stdlib try! or ? does).
// This reduces the size of the rlib by a few percent
macro_rules! tri {
    ($e:expr $(,)?) => {
        match $e {
            core::result::Result::Ok(val) => val,
            core::result::Result::Err(err) => return core::result::Result::Err(err),
        }
    };
}

pub mod element;

pub mod json;
pub mod xml;

mod context;
mod decimal;
mod model;
mod utils;

use std::fmt::Debug;

use fhirbolt_shared::FhirRelease;

pub use context::{
    de::{
        DeserializationConfig, DeserializationContext, DeserializationMode, DeserializeResource,
        DeserializeResourceOwned,
    },
    ser::{SerializationConfig, SerializationContext, SerializeResource},
};

/// Marker trait for all types representing FHIR resources.
pub trait Resource: Sized + Clone + PartialEq + Debug {
    const FHIR_RELEASE: FhirRelease;
}
