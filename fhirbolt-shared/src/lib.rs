#![feature(adt_const_params)]
#![feature(generic_const_exprs)]

use std::fmt;

use element::Element;

pub mod element;
pub mod serde_context;

pub mod path;
pub mod serde_helpers;

/// Marker trait for all types representing FHIR resources.
pub trait AnyResource: Sized {
    const FHIR_RELEASE: FhirRelease;

    fn to_element(self) -> Element<{ Self::FHIR_RELEASE }> {
        unimplemented!()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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
