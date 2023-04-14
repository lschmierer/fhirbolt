pub mod element_map;
pub mod path;
pub mod type_hints;

pub type FhirRelease = usize;

// This is a workaround to mimic a const generic enum until more complex const parameter types are stable.
// See https://github.com/rust-lang/rust/issues/95174 for more information.
#[allow(non_snake_case)]
pub mod FhirReleases {
    use crate::FhirRelease;

    pub const R4: FhirRelease = 4_0;
    pub const R4B: FhirRelease = 4_3;
}
