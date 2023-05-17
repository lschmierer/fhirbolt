//! This module provides conversion functions (data mappers)
//! to translate a HL7 v2 message to FHIR resources.

use fhirbolt::model::r5::types::{Code, CodeableConcept, Coding, Identifier, Reference};

pub use condition::map_conditions;
pub use encounter::map_encounter;
pub use patient::map_patient;

use crate::hl7v2::{ComponentAccess, FieldAccess, RepeatedField, SubComponent, SubComponentAccess};

mod condition;
mod encounter;
mod patient;

/// Utility function to build a FHIR reference.
fn build_reference<S: Into<String>>(reference: S) -> Box<Reference> {
    let reference_str = reference.into();
    Box::new(Reference {
        reference: Some(reference_str.into()),
        ..Default::default()
    })
}

/// Utility function to build a FHIR CodeableConcept.
///
/// The `CodeableConcept` is alreadu wrapped in a `Vec` and a `Box`,
/// because this is the most common useage.
fn build_codeable_concept(system: &str, code: Code, display: Option<&str>) -> Vec<CodeableConcept> {
    vec![CodeableConcept {
        coding: vec![Coding {
            system: Some(system.into()),
            code: Some(code),
            display: display.map(|d| d.into()),
            ..Default::default()
        }],
        ..Default::default()
    }]
}

/// Map FHIR identifiers from a repeated field.
fn map_identifier(fields: &RepeatedField) -> Vec<Identifier> {
    fields
        .iter()
        .flat_map(|f| {
            f.component(1)
                .first_sub()
                .to_fhir_string()
                .map(|id| Identifier {
                    value: Some(id),
                    ..Default::default()
                })
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
