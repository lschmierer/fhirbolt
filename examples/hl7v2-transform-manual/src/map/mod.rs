//! This module provides conversion functions (data mappers)
//! to translate a HL7 v2 message to FHIR resources.

use fhirbolt::model::r5::types::{
    Code, CodeableConcept, Coding, Identifier, Reference, String as FhirString, Uri,
};

pub use condition::map_conditions;
pub use encounter::map_encounter;
pub use patient::map_patient;

use crate::hl7v2::{ComponentAccess, FieldAccess, RepeatedField, SubComponent, SubComponentAccess};

mod condition;
mod encounter;
mod patient;

/// Utility function to map an owned `String` or a string reference `&str` to a FHIR String.
fn build_fhir_string<S: Into<String>>(str: S) -> FhirString {
    FhirString {
        value: Some(str.into()),
        ..Default::default()
    }
}

/// Utility function to build a FHIR reference.
fn build_reference<S: Into<String>>(reference: S) -> Box<Reference> {
    Box::new(Reference {
        reference: Some(build_fhir_string(reference)),
        ..Default::default()
    })
}

/// Utility function to build a FHIR CodeableConcept.
///
/// The `CodeableConcept` is alreadu wrapped in a `Vec` and a `Box`,
/// because this is the most common useage.
fn build_codeable_concept(
    system: &str,
    code: Code,
    display: Option<&str>,
) -> Vec<Box<CodeableConcept>> {
    vec![Box::new(CodeableConcept {
        coding: vec![Box::new(Coding {
            system: Some(Uri {
                value: Some(system.to_string()),
                ..Default::default()
            }),
            code: Some(code),
            display: display.map(build_fhir_string),
            ..Default::default()
        })],
        ..Default::default()
    })]
}

/// Map FHIR identifiers from a repeated field.
fn map_identifier(fields: &RepeatedField) -> Vec<Box<Identifier>> {
    fields
        .iter()
        .flat_map(|f| {
            if let Some(id) = f.component(1).first_sub().to_fhir_string() {
                Some(Box::new(Identifier {
                    value: Some(id),
                    ..Default::default()
                }))
            } else {
                None
            }
        })
        .collect()
}

/// Map a FHIR Code from a sub-component.
fn map_code(from: &SubComponent, map: &[(&str, &str)]) -> Option<Code> {
    map.iter()
        .find(|(key, _value)| Some(*key) == from.as_str())
        .map(|(_key, value)| Code {
            value: Some(value.to_string()),
            ..Default::default()
        })
}
