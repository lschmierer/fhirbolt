pub mod serde_config;

use std::fmt;

pub trait AnyResource {
    fn fhir_release() -> FhirRelease;
}

#[derive(Copy, Clone, Debug)]
pub enum FhirRelease {
    R4,
    R4B,
}

impl fmt::Display for FhirRelease {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FhirRelease::R4 => write!(f, "R4"),
            FhirRelease::R4B => write!(f, "R4B"),
        }
    }
}
