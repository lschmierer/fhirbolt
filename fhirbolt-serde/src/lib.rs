//! (De)serialize FHIR resources from and to JSON and XML.

pub mod element;

pub mod json;
pub mod xml;

mod context;
mod decimal;
mod model;

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
