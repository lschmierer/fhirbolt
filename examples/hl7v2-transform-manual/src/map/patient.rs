use fhirbolt::model::r5::{
    resources::{Patient, PatientCommunication, PatientContact, PatientDeceased},
    types::{Address, Boolean, Code, CodeableConcept, ContactPoint, HumanName},
};

use crate::hl7v2::{
    ComponentAccess, Field, FieldAccess, Message, MessageAccess, RepeatedField, Segment,
    SegmentAccess, SubComponentAccess,
};

use super::{build_codeable_concept, map_code, map_identifier};

const NAME_USE_MAP: &[(&str, &str)] = &[
    ("D", "ususal"),
    ("L", "official"),
    ("N", "nickname"),
    ("S", "anonymous"),
    ("M", "maiden"),
];

const GENDER_MAP: &[(&str, &str)] = &[
    ("A", "other"),
    ("F", "female"),
    ("M", "male"),
    ("N", "usual"),
    ("O", "other"),
    ("U", "unknown"),
];

/// Map a HL7 v2 message to a FHIR patient.
pub fn map_patient(message: &Message, id: &str) -> Patient {
    let pid_segment = message.segments_by_id("PID").next();

    Patient {
        id: Some(id.into()),
        identifier: map_identifier(pid_segment.repeated(3)),
        name: map_name(pid_segment.repeated(5)),
        telecom: map_telecoms(pid_segment.repeated(13), pid_segment.repeated(14)),
        gender: map_gender(pid_segment),
        birth_date: pid_segment.field(7).component(1).first_sub().to_fhir_date(),
        deceased: map_deceased(pid_segment),
        address: map_address(pid_segment.repeated(11)),
        marital_status: map_marital_status(pid_segment),
        contact: map_contacts(message),
        communication: map_communicatiion(pid_segment),
        ..Default::default()
    }
}

fn map_name(fields: &RepeatedField) -> Vec<HumanName> {
    fields
        .iter()
        .flat_map(|f| {
            let family_name = f.component(1).first_sub().to_fhir_string();
            let given_names: Vec<_> = f
                .components()
                .iter()
                .skip(1)
                .take(2)
                .filter_map(|c| c.first_sub().to_fhir_string())
                .collect();
            let r#use = f
                .component(7)
                .first_sub()
                .and_then(|c| map_code(c, NAME_USE_MAP));

            if family_name.is_some() || !given_names.is_empty() {
                Some(HumanName {
                    family: family_name,
                    given: given_names,
                    r#use,
                    ..Default::default()
                })
            } else {
                None
            }
        })
        .collect()
}

fn map_telecoms(home_fields: &RepeatedField, work_fields: &RepeatedField) -> Vec<ContactPoint> {
    let home_telecoms_iter = home_fields.iter().map(|f| map_telecom(f, "home"));
    let work_telecoms_iter = work_fields.iter().map(|f| map_telecom(f, "work"));

    home_telecoms_iter
        .chain(work_telecoms_iter)
        .flat_map(|f| f)
        .collect()
}

fn map_telecom(telecom_field: &Field, r#use: &str) -> Option<ContactPoint> {
    if let Some(number_string) = telecom_field.component(1).first_sub().to_fhir_string() {
        Some(ContactPoint {
            system: Some(Code {
                value: Some("phone".into()),
                ..Default::default()
            }),
            value: Some(number_string),
            r#use: Some(Code {
                value: Some(r#use.into()),
                ..Default::default()
            }),
            ..Default::default()
        })
    } else {
        None
    }
}

fn map_gender(pid_segment: Option<&Segment>) -> Option<Code> {
    pid_segment
        .field(8)
        .component(1)
        .first_sub()
        .and_then(|c| map_code(c, GENDER_MAP))
}

fn map_deceased(pid_segment: Option<&Segment>) -> Option<PatientDeceased> {
    let deceased_date_time = pid_segment
        .field(29)
        .component(1)
        .first_sub()
        .to_fhir_date_time();

    if pid_segment
        .field(30)
        .component(1)
        .first_sub()
        .as_bool()
        .unwrap_or(false)
    {
        if let Some(date_time) = deceased_date_time {
            Some(PatientDeceased::DateTime(Box::new(date_time)))
        } else {
            Some(PatientDeceased::Boolean(Box::new(Boolean {
                value: Some(true),
                ..Default::default()
            })))
        }
    } else {
        if deceased_date_time.is_some() {
            println!("\x1B[33mWarning\x1B[39m: Patient has a deceased date although still alive")
        }

        Some(PatientDeceased::Boolean(Box::new(Boolean {
            value: Some(false),
            ..Default::default()
        })))
    }
}

fn map_address(fields: &RepeatedField) -> Vec<Address> {
    fields
        .iter()
        .map(|f| Address {
            line: f
                .components()
                .iter()
                .take(2)
                .flat_map(|c| c.first_sub().to_fhir_string())
                .collect(),
            city: f.component(3).first_sub().to_fhir_string(),
            state: f.component(4).first_sub().to_fhir_string(),
            postal_code: f.component(5).first_sub().to_fhir_string(),
            country: f.component(6).first_sub().to_fhir_string(),
            ..Default::default()
        })
        .collect()
}

fn map_marital_status(pid_segment: Option<&Segment>) -> Option<Box<CodeableConcept>> {
    pid_segment
        .field(16)
        .component(1)
        .first_sub()
        .to_fhir_code()
        .and_then(|c| {
            build_codeable_concept(
                "http://terminology.hl7.org/CodeSystem/v3-MaritalStatus",
                c,
                None,
            )
            .into_iter()
            .next()
        })
        .map(Box::new)
}

fn map_contacts(message: &Message) -> Vec<PatientContact> {
    message.segments_by_id("NK1").map(map_contact).collect()
}

fn map_contact(nk1_segment: &Segment) -> PatientContact {
    PatientContact {
        name: map_name(nk1_segment.repeated(2))
            .into_iter()
            .next()
            .map(Box::new),
        telecom: map_telecoms(nk1_segment.repeated(5), nk1_segment.repeated(6)),
        address: map_address(nk1_segment.repeated(4))
            .into_iter()
            .next()
            .map(Box::new),
        ..Default::default()
    }
}

fn map_communicatiion(pid_segment: Option<&Segment>) -> Vec<PatientCommunication> {
    pid_segment
        .field(15)
        .component(1)
        .first_sub()
        .as_str()
        .and_then(|s| match s {
            "D" => Some("de"),
            s => {
                println!("\x1B[33mWarning:\x1B[39m: unknown language `{}`", s);
                None
            }
        })
        .and_then(|c| {
            build_codeable_concept(
                "urn:ietf:bcp:47",
                Code {
                    value: Some(c.into()),
                    ..Default::default()
                },
                None,
            )
            .into_iter()
            .next()
        })
        .map(|c| {
            vec![PatientCommunication {
                language: Box::new(c),
                preferred: Some(Boolean {
                    value: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            }]
        })
        .unwrap_or_default()
}
