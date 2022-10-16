//! Generated Rust structures for FHIR resources.
//!
//! You can select which FHIR release to include by speficifying them as Cargo features.
//! ```toml
//! [dependencies]
//! fhirbolt = { version = "0.1", features = ["r4", "r4b"] }
//! ```
//! By default, no FHIR release is included.

#[warn(variant_size_differences)]
mod generated;

pub use fhirbolt_shared::{AnyResource, FhirRelease};

pub use generated::*;
