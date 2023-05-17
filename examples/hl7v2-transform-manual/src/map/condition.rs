use fhirbolt::model::r5::{
    resources::Condition,
    types::{CodeableConcept, Coding, Uri},
};

use crate::hl7v2::{
    ComponentAccess, Field, FieldAccess, Message, MessageAccess, Segment, SegmentAccess,
    SubComponentAccess,
};

use super::{build_codeable_concept, build_reference};

/// Map a HL7 v2 message to a list of FHIR conditions.
///
/// This function tries to remove duplicates.
pub fn map_conditions(message: &Message, patient_id: &str, encounter_id: &str) -> Vec<Condition> {
    let mut conditions = vec![];

    for segment in message.segments_by_id("DG1") {
        let condition = map_condition(segment, patient_id, encounter_id);

        if !conditions.contains(&condition) {
            conditions.push(condition);
        }
    }

    // we skipped assigngin ids earlier, to make it easier to compre for duplicates
    let mut condition_id = -1;

    for condition in &mut conditions {
        condition_id += 1;
        condition.id = Some(format!("{}condition{}", encounter_id, condition_id).into());
    }

    conditions
}

fn map_condition(dg1_segment: &Segment, patient_id: &str, encounter_id: &str) -> Condition {
    Condition {
        clinical_status: build_clinical_status_active(),
        code: map_condition_code(dg1_segment.field(3)),
        subject: build_reference(format!("Patient/{}", patient_id)),
        encounter: Some(build_reference(format!("Encounter/{}", encounter_id))),
        ..Default::default()
    }
}

fn build_clinical_status_active() -> Box<CodeableConcept> {
    Box::new(CodeableConcept {
        coding: vec![Coding {
            system: Some(Uri {
                value: Some("http://terminology.hl7.org/CodeSystem/condition-clinical".to_string()),
                ..Default::default()
            }),
            code: Some("active".into()),
            ..Default::default()
        }],
        ..Default::default()
    })
}

fn map_condition_code(field: Option<&Field>) -> Option<Box<CodeableConcept>> {
    let code = field.component(1).first_sub().to_fhir_code();
    let system = match field.component(3).first_sub().as_str() {
        Some("I10") => Some("http://hl7.org/fhir/sid/icd-10"),
        Some(s) => {
            println!("Warning: unknown code system: `{}`", s);
            None
        }
        _ => None,
    };

    if let Some((code, system)) = code.zip(system) {
        build_codeable_concept(system, code, field.component(2).first_sub().as_str())
            .into_iter()
            .next()
            .map(Box::new)
    } else {
        None
    }
}
