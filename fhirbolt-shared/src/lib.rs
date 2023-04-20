pub mod element_map;
pub mod path;
pub mod type_hints;

/// Generic FHIR Release.
///
/// Refer to  [`fhirbolt::FhirReleases`](FhirReleases).
pub type FhirRelease = usize;

/// Supported FHIR Releases.
///
/// This is a workaround to mimic a const generic enum until more complex const parameter types are stable.
/// See [https://github.com/rust-lang/rust/issues/95174](https://github.com/rust-lang/rust/issues/95174) for more information.\
///
/// Refer to  [`fhirbolt::FhirRelease`](FhirRelease).
#[allow(non_snake_case)]
pub mod FhirReleases {
    use crate::FhirRelease;

    /// Constant for FHIR release R4.
    pub const R4: FhirRelease = 4_0;
    /// Constant for FHIR release R4B.
    pub const R4B: FhirRelease = 4_3;
    /// Constant for FHIR release R5.
    pub const R5: FhirRelease = 5_0;
}
