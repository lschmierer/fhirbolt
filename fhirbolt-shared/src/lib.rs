#![feature(adt_const_params)]

use std::fmt;

pub mod element;
pub mod serde_context;

pub mod path;
pub mod serde_helpers;

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
