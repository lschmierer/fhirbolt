use fhirbolt::model::r5::{
    resources::{Encounter, EncounterLocation},
    types::{Code, CodeableConcept, Identifier, Period, Reference},
};

use crate::hl7v2::{
    ComponentAccess, FieldAccess, Message, MessageAccess, Segment, SegmentAccess,
    SubComponentAccess,
};

use super::{build_codeable_concept, build_reference, map_code, map_identifier};

const CLASS_MAP: &[(&str, &str)] = &[("I", "IMP"), ("O", "AMB"), ("E", "EMER")];

/// Map a HL7 v2 message to a FHIR encounter.
pub fn map_encounter(message: &Message, id: &str, patient_id: &str) -> Encounter {
    let pv1_segment = message.segments_by_id("PV1").next();

    Encounter {
        id: Some(id.into()),
        identifier: map_identifier(pv1_segment.repeated(19)),
        status: Code {
            value: Some("finished".into()),
            ..Default::default()
        },
        class: map_class(pv1_segment),
        subject: Some(build_reference(format!("Patient/{}", patient_id))),
        actual_period: map_actual_period(pv1_segment),
        location: map_location(pv1_segment),
        ..Default::default()
    }
}

fn map_class(pv1_segment: Option<&Segment>) -> Vec<Box<CodeableConcept>> {
    pv1_segment
        .field(2)
        .component(1)
        .first_sub()
        .and_then(|c| map_code(c, CLASS_MAP))
        .map(|c| {
            build_codeable_concept("http://terminology.hl7.org/CodeSystem/v3-ActCode", c, None)
        })
        .unwrap_or_default()
}

fn map_actual_period(pv1_segment: Option<&Segment>) -> Option<Box<Period>> {
    let start = pv1_segment
        .field(44)
        .component(1)
        .first_sub()
        .to_fhir_date_time();
    let end = pv1_segment
        .field(45)
        .component(1)
        .first_sub()
        .to_fhir_date_time();

    if start.is_some() || end.is_some() {
        Some(Box::new(Period {
            start,
            end,
            ..Default::default()
        }))
    } else {
        None
    }
}

fn map_location(pv1_segment: Option<&Segment>) -> Vec<EncounterLocation> {
    pv1_segment
        .field(3)
        .component(1)
        .first_sub()
        .to_fhir_string()
        .map(|l| {
            vec![EncounterLocation {
                location: Box::new(Reference {
                    identifier: Some(Box::new(Identifier {
                        value: Some(l),
                        ..Default::default()
                    })),
                    ..Default::default()
                }),
                ..Default::default()
            }]
        })
        .unwrap_or_default()
}
