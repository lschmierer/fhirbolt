use std::any::Any;

pub mod element;
pub mod serde_context;

pub mod path;
pub mod serde_helpers;

/// Marker trait for all types representing FHIR resources.
pub trait AnyResource: Any {
    const FHIR_RELEASE: FhirRelease;
}

#[allow(non_snake_case)]
pub mod FhirReleases {
    use crate::FhirRelease;

    pub const R4: FhirRelease = 4_0;
    pub const R4B: FhirRelease = 4_3;
}

pub type FhirRelease = u8;

pub trait FhirReleaseExt {
    fn name(&self) -> &str;
}

impl FhirReleaseExt for FhirRelease {
    fn name(&self) -> &str {
        match *self {
            FhirReleases::R4 => "R4",
            FhirReleases::R4B => "R4B",
            _ => unreachable!(),
        }
    }
}
