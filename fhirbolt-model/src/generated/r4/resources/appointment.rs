// Generated on 2023-04-05 by fhirbolt-codegen v0.1.0
#[doc = "List of participants involved in the appointment."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AppointmentParticipant {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Role of participant in the appointment."]
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A Person, Location/HealthcareService or Device that is participating in the appointment."]
    pub r#actor: Option<Box<super::super::types::Reference>>,
    #[doc = "Whether this participant is required to be present at the meeting. This covers a use-case where two doctors need to meet to discuss the results for a specific patient, and the patient is not required to be present."]
    pub r#required: Option<super::super::types::Code>,
    #[doc = "Participation status of the actor."]
    pub r#status: super::super::types::Code,
    #[doc = "Participation period of the actor."]
    pub r#period: Option<Box<super::super::types::Period>>,
}
impl serde::ser::Serialize for AppointmentParticipant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if !self.r#type.is_empty() {
                state.serialize_entry("type", &self.r#type)?;
            }
            if let Some(some) = self.r#actor.as_ref() {
                state.serialize_entry("actor", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#required.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("required", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_required", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#required.as_ref() {
                    state.serialize_entry("required", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#status.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("status", &some)?;
                }
                if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#status.id.as_ref(),
                        extension: &self.r#status.extension,
                    };
                    state.serialize_entry("_status", &primitive_element)?;
                }
            } else {
                state.serialize_entry("status", &self.r#status)?;
            }
            if let Some(some) = self.r#period.as_ref() {
                state.serialize_entry("period", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for AppointmentParticipant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "actor")]
            Actor,
            #[serde(rename = "required")]
            Required,
            #[serde(rename = "_required")]
            RequiredPrimitiveElement,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "period")]
            Period,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AppointmentParticipant;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AppointmentParticipant")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<AppointmentParticipant, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#actor: Option<Box<super::super::types::Reference>> = None;
                let mut r#required: Option<super::super::types::Code> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#period: Option<Box<super::super::types::Period>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Type => {
                                if _ctx.from_json {
                                    if r#type.is_some() {
                                        return Err(serde::de::Error::duplicate_field("type"));
                                    }
                                    r#type = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#type.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Actor => {
                                if r#actor.is_some() {
                                    return Err(serde::de::Error::duplicate_field("actor"));
                                }
                                r#actor = Some(map_access.next_value()?);
                            }
                            Field::Required => {
                                if _ctx.from_json {
                                    let some = r#required.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("required"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#required.is_some() {
                                        return Err(serde::de::Error::duplicate_field("required"));
                                    }
                                    r#required = Some(map_access.next_value()?);
                                }
                            }
                            Field::RequiredPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#required.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_required"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "required",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "actor",
                                            "required",
                                            "status",
                                            "period",
                                        ],
                                    ));
                                }
                            }
                            Field::Status => {
                                if _ctx.from_json {
                                    let some = r#status.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("status"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#status.is_some() {
                                        return Err(serde::de::Error::duplicate_field("status"));
                                    }
                                    r#status = Some(map_access.next_value()?);
                                }
                            }
                            Field::StatusPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#status.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_status"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "status",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "actor",
                                            "required",
                                            "status",
                                            "period",
                                        ],
                                    ));
                                }
                            }
                            Field::Period => {
                                if r#period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("period"));
                                }
                                r#period = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "type",
                                        "actor",
                                        "required",
                                        "status",
                                        "period",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(AppointmentParticipant {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type: r#type.unwrap_or(vec![]),
                        r#actor,
                        r#required,
                        r#status: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#period,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A booking of a healthcare event among patient(s), practitioner(s), related person(s) and/or device(s) for a specific date/time. This may result in one or more Encounter(s)."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Appointment {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<std::string::String>,
    #[doc = "The metadata about the resource. This is content that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource."]
    pub r#meta: Option<Box<super::super::types::Meta>>,
    #[doc = "A reference to a set of rules that were followed when the resource was constructed, and which must be understood when processing the content. Often, this is a reference to an implementation guide that defines the special rules along with other profiles etc."]
    pub r#implicit_rules: Option<super::super::types::Uri>,
    #[doc = "The base language in which the resource is written."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "A human-readable narrative that contains a summary of the resource and can be used to represent the content of the resource to a human. The narrative need not encode all the structured data, but is required to contain sufficient detail to make it \"clinically safe\" for a human to just read the narrative. Resource definitions may define what content should be represented in the narrative to ensure clinical safety."]
    pub r#text: Option<Box<super::super::types::Narrative>>,
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<Box<super::super::Resource>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "This records identifiers associated with this appointment concern that are defined by business processes and/or used to refer to it when a direct URL reference to the resource itself is not appropriate (e.g. in CDA documents, or in written / printed documentation)."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The overall status of the Appointment. Each of the participants has their own participation status which indicates their involvement in the process, however this status indicates the shared status."]
    pub r#status: super::super::types::Code,
    #[doc = "The coded reason for the appointment being cancelled. This is often used in reporting/billing/futher processing to determine if further actions are required, or specific fees apply."]
    pub r#cancelation_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A broad categorization of the service that is to be performed during this appointment."]
    pub r#service_category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The specific service that is to be performed during this appointment."]
    pub r#service_type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The specialty of a practitioner that would be required to perform the service requested in this appointment."]
    pub r#specialty: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The style of appointment or patient that has been booked in the slot (not service type)."]
    pub r#appointment_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The coded reason that this appointment is being scheduled. This is more clinical than administrative."]
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Reason the appointment has been scheduled to take place, as specified using information from another resource. When the patient arrives and the encounter begins it may be used as the admission diagnosis. The indication will typically be a Condition (with other resources referenced in the evidence.detail), or a Procedure."]
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "The priority of the appointment. Can be used to make informed decisions if needing to re-prioritize appointments. (The iCal Standard specifies 0 as undefined, 1 as highest, 9 as lowest priority)."]
    pub r#priority: Option<super::super::types::UnsignedInt>,
    #[doc = "The brief description of the appointment as would be shown on a subject line in a meeting request, or appointment list. Detailed or expanded information should be put in the comment field."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Additional information to support the appointment provided when making the appointment."]
    pub r#supporting_information: Vec<Box<super::super::types::Reference>>,
    #[doc = "Date/Time that the appointment is to take place."]
    pub r#start: Option<super::super::types::Instant>,
    #[doc = "Date/Time that the appointment is to conclude."]
    pub r#end: Option<super::super::types::Instant>,
    #[doc = "Number of minutes that the appointment is to take. This can be less than the duration between the start and end times.  For example, where the actual time of appointment is only an estimate or if a 30 minute appointment is being requested, but any time would work.  Also, if there is, for example, a planned 15 minute break in the middle of a long appointment, the duration may be 15 minutes less than the difference between the start and end."]
    pub r#minutes_duration: Option<super::super::types::PositiveInt>,
    #[doc = "The slots from the participants' schedules that will be filled by the appointment."]
    pub r#slot: Vec<Box<super::super::types::Reference>>,
    #[doc = "The date that this appointment was initially created. This could be different to the meta.lastModified value on the initial entry, as this could have been before the resource was created on the FHIR server, and should remain unchanged over the lifespan of the appointment."]
    pub r#created: Option<super::super::types::DateTime>,
    #[doc = "Additional comments about the appointment."]
    pub r#comment: Option<super::super::types::String>,
    #[doc = "While Appointment.comment contains information for internal use, Appointment.patientInstructions is used to capture patient facing information about the Appointment (e.g. please bring your referral or fast from 8pm night before)."]
    pub r#patient_instruction: Option<super::super::types::String>,
    #[doc = "The service request this appointment is allocated to assess (e.g. incoming referral or procedure request)."]
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    #[doc = "List of participants involved in the appointment."]
    pub r#participant: Vec<AppointmentParticipant>,
    #[doc = "A set of date ranges (potentially including times) that the appointment is preferred to be scheduled within.\n\nThe duration (usually in minutes) could also be provided to indicate the length of the appointment to fill and populate the start/end times for the actual allocated time. However, in other situations the duration may be calculated by the scheduling system."]
    pub r#requested_period: Vec<Box<super::super::types::Period>>,
}
impl crate::AnyResource for Appointment {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4;
}
impl serde::ser::Serialize for Appointment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "Appointment")?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if let Some(some) = self.r#meta.as_ref() {
                state.serialize_entry("meta", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#implicit_rules.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("implicitRules", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_implicitRules", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#implicit_rules.as_ref() {
                    state.serialize_entry("implicitRules", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#language.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("language", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_language", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#language.as_ref() {
                    state.serialize_entry("language", some)?;
                }
            }
            if let Some(some) = self.r#text.as_ref() {
                state.serialize_entry("text", some)?;
            }
            if !self.r#contained.is_empty() {
                state.serialize_entry("contained", &self.r#contained)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if !self.r#identifier.is_empty() {
                state.serialize_entry("identifier", &self.r#identifier)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#status.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("status", &some)?;
                }
                if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#status.id.as_ref(),
                        extension: &self.r#status.extension,
                    };
                    state.serialize_entry("_status", &primitive_element)?;
                }
            } else {
                state.serialize_entry("status", &self.r#status)?;
            }
            if let Some(some) = self.r#cancelation_reason.as_ref() {
                state.serialize_entry("cancelationReason", some)?;
            }
            if !self.r#service_category.is_empty() {
                state.serialize_entry("serviceCategory", &self.r#service_category)?;
            }
            if !self.r#service_type.is_empty() {
                state.serialize_entry("serviceType", &self.r#service_type)?;
            }
            if !self.r#specialty.is_empty() {
                state.serialize_entry("specialty", &self.r#specialty)?;
            }
            if let Some(some) = self.r#appointment_type.as_ref() {
                state.serialize_entry("appointmentType", some)?;
            }
            if !self.r#reason_code.is_empty() {
                state.serialize_entry("reasonCode", &self.r#reason_code)?;
            }
            if !self.r#reason_reference.is_empty() {
                state.serialize_entry("reasonReference", &self.r#reason_reference)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#priority.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("priority", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_priority", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#priority.as_ref() {
                    state.serialize_entry("priority", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#description.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("description", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_description", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#description.as_ref() {
                    state.serialize_entry("description", some)?;
                }
            }
            if !self.r#supporting_information.is_empty() {
                state.serialize_entry("supportingInformation", &self.r#supporting_information)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#start.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("start", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_start", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#start.as_ref() {
                    state.serialize_entry("start", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#end.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("end", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_end", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#end.as_ref() {
                    state.serialize_entry("end", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#minutes_duration.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("minutesDuration", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_minutesDuration", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#minutes_duration.as_ref() {
                    state.serialize_entry("minutesDuration", some)?;
                }
            }
            if !self.r#slot.is_empty() {
                state.serialize_entry("slot", &self.r#slot)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#created.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("created", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_created", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#created.as_ref() {
                    state.serialize_entry("created", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#comment.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("comment", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_comment", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#comment.as_ref() {
                    state.serialize_entry("comment", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#patient_instruction.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("patientInstruction", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_patientInstruction", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#patient_instruction.as_ref() {
                    state.serialize_entry("patientInstruction", some)?;
                }
            }
            if !self.r#based_on.is_empty() {
                state.serialize_entry("basedOn", &self.r#based_on)?;
            }
            if !self.r#participant.is_empty() {
                state.serialize_entry("participant", &self.r#participant)?;
            }
            if !self.r#requested_period.is_empty() {
                state.serialize_entry("requestedPeriod", &self.r#requested_period)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for Appointment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "resourceType")]
            ResourceType,
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "meta")]
            Meta,
            #[serde(rename = "implicitRules")]
            ImplicitRules,
            #[serde(rename = "_implicitRules")]
            ImplicitRulesPrimitiveElement,
            #[serde(rename = "language")]
            Language,
            #[serde(rename = "_language")]
            LanguagePrimitiveElement,
            #[serde(rename = "text")]
            Text,
            #[serde(rename = "contained")]
            Contained,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "cancelationReason")]
            CancelationReason,
            #[serde(rename = "serviceCategory")]
            ServiceCategory,
            #[serde(rename = "serviceType")]
            ServiceType,
            #[serde(rename = "specialty")]
            Specialty,
            #[serde(rename = "appointmentType")]
            AppointmentType,
            #[serde(rename = "reasonCode")]
            ReasonCode,
            #[serde(rename = "reasonReference")]
            ReasonReference,
            #[serde(rename = "priority")]
            Priority,
            #[serde(rename = "_priority")]
            PriorityPrimitiveElement,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "supportingInformation")]
            SupportingInformation,
            #[serde(rename = "start")]
            Start,
            #[serde(rename = "_start")]
            StartPrimitiveElement,
            #[serde(rename = "end")]
            End,
            #[serde(rename = "_end")]
            EndPrimitiveElement,
            #[serde(rename = "minutesDuration")]
            MinutesDuration,
            #[serde(rename = "_minutesDuration")]
            MinutesDurationPrimitiveElement,
            #[serde(rename = "slot")]
            Slot,
            #[serde(rename = "created")]
            Created,
            #[serde(rename = "_created")]
            CreatedPrimitiveElement,
            #[serde(rename = "comment")]
            Comment,
            #[serde(rename = "_comment")]
            CommentPrimitiveElement,
            #[serde(rename = "patientInstruction")]
            PatientInstruction,
            #[serde(rename = "_patientInstruction")]
            PatientInstructionPrimitiveElement,
            #[serde(rename = "basedOn")]
            BasedOn,
            #[serde(rename = "participant")]
            Participant,
            #[serde(rename = "requestedPeriod")]
            RequestedPeriod,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Appointment;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Appointment")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Appointment, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<super::super::types::Meta>> = None;
                let mut r#implicit_rules: Option<super::super::types::Uri> = None;
                let mut r#language: Option<super::super::types::Code> = None;
                let mut r#text: Option<Box<super::super::types::Narrative>> = None;
                let mut r#contained: Option<Vec<Box<super::super::Resource>>> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#cancelation_reason: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#service_category: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#service_type: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#specialty: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#appointment_type: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#reason_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#reason_reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#priority: Option<super::super::types::UnsignedInt> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#supporting_information: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#start: Option<super::super::types::Instant> = None;
                let mut r#end: Option<super::super::types::Instant> = None;
                let mut r#minutes_duration: Option<super::super::types::PositiveInt> = None;
                let mut r#slot: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#created: Option<super::super::types::DateTime> = None;
                let mut r#comment: Option<super::super::types::String> = None;
                let mut r#patient_instruction: Option<super::super::types::String> = None;
                let mut r#based_on: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#participant: Option<Vec<AppointmentParticipant>> = None;
                let mut r#requested_period: Option<Vec<Box<super::super::types::Period>>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "Appointment" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"Appointment",
                                    ));
                                }
                            }
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Meta => {
                                if r#meta.is_some() {
                                    return Err(serde::de::Error::duplicate_field("meta"));
                                }
                                r#meta = Some(map_access.next_value()?);
                            }
                            Field::ImplicitRules => {
                                if _ctx.from_json {
                                    let some = r#implicit_rules.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "implicitRules",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#implicit_rules.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "implicitRules",
                                        ));
                                    }
                                    r#implicit_rules = Some(map_access.next_value()?);
                                }
                            }
                            Field::ImplicitRulesPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#implicit_rules.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_implicitRules",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "implicitRules",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "status",
                                            "cancelationReason",
                                            "serviceCategory",
                                            "serviceType",
                                            "specialty",
                                            "appointmentType",
                                            "reasonCode",
                                            "reasonReference",
                                            "priority",
                                            "description",
                                            "supportingInformation",
                                            "start",
                                            "end",
                                            "minutesDuration",
                                            "slot",
                                            "created",
                                            "comment",
                                            "patientInstruction",
                                            "basedOn",
                                            "participant",
                                            "requestedPeriod",
                                        ],
                                    ));
                                }
                            }
                            Field::Language => {
                                if _ctx.from_json {
                                    let some = r#language.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("language"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#language.is_some() {
                                        return Err(serde::de::Error::duplicate_field("language"));
                                    }
                                    r#language = Some(map_access.next_value()?);
                                }
                            }
                            Field::LanguagePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#language.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_language"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "language",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "status",
                                            "cancelationReason",
                                            "serviceCategory",
                                            "serviceType",
                                            "specialty",
                                            "appointmentType",
                                            "reasonCode",
                                            "reasonReference",
                                            "priority",
                                            "description",
                                            "supportingInformation",
                                            "start",
                                            "end",
                                            "minutesDuration",
                                            "slot",
                                            "created",
                                            "comment",
                                            "patientInstruction",
                                            "basedOn",
                                            "participant",
                                            "requestedPeriod",
                                        ],
                                    ));
                                }
                            }
                            Field::Text => {
                                if r#text.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                r#text = Some(map_access.next_value()?);
                            }
                            Field::Contained => {
                                if _ctx.from_json {
                                    if r#contained.is_some() {
                                        return Err(serde::de::Error::duplicate_field("contained"));
                                    }
                                    r#contained = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#contained.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Identifier => {
                                if _ctx.from_json {
                                    if r#identifier.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "identifier",
                                        ));
                                    }
                                    r#identifier = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#identifier.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Status => {
                                if _ctx.from_json {
                                    let some = r#status.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("status"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#status.is_some() {
                                        return Err(serde::de::Error::duplicate_field("status"));
                                    }
                                    r#status = Some(map_access.next_value()?);
                                }
                            }
                            Field::StatusPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#status.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_status"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "status",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "status",
                                            "cancelationReason",
                                            "serviceCategory",
                                            "serviceType",
                                            "specialty",
                                            "appointmentType",
                                            "reasonCode",
                                            "reasonReference",
                                            "priority",
                                            "description",
                                            "supportingInformation",
                                            "start",
                                            "end",
                                            "minutesDuration",
                                            "slot",
                                            "created",
                                            "comment",
                                            "patientInstruction",
                                            "basedOn",
                                            "participant",
                                            "requestedPeriod",
                                        ],
                                    ));
                                }
                            }
                            Field::CancelationReason => {
                                if r#cancelation_reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "cancelationReason",
                                    ));
                                }
                                r#cancelation_reason = Some(map_access.next_value()?);
                            }
                            Field::ServiceCategory => {
                                if _ctx.from_json {
                                    if r#service_category.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "serviceCategory",
                                        ));
                                    }
                                    r#service_category = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#service_category.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ServiceType => {
                                if _ctx.from_json {
                                    if r#service_type.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "serviceType",
                                        ));
                                    }
                                    r#service_type = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#service_type.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Specialty => {
                                if _ctx.from_json {
                                    if r#specialty.is_some() {
                                        return Err(serde::de::Error::duplicate_field("specialty"));
                                    }
                                    r#specialty = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#specialty.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::AppointmentType => {
                                if r#appointment_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "appointmentType",
                                    ));
                                }
                                r#appointment_type = Some(map_access.next_value()?);
                            }
                            Field::ReasonCode => {
                                if _ctx.from_json {
                                    if r#reason_code.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "reasonCode",
                                        ));
                                    }
                                    r#reason_code = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#reason_code.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ReasonReference => {
                                if _ctx.from_json {
                                    if r#reason_reference.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "reasonReference",
                                        ));
                                    }
                                    r#reason_reference = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#reason_reference.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Priority => {
                                if _ctx.from_json {
                                    let some = r#priority.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("priority"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#priority.is_some() {
                                        return Err(serde::de::Error::duplicate_field("priority"));
                                    }
                                    r#priority = Some(map_access.next_value()?);
                                }
                            }
                            Field::PriorityPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#priority.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_priority"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "priority",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "status",
                                            "cancelationReason",
                                            "serviceCategory",
                                            "serviceType",
                                            "specialty",
                                            "appointmentType",
                                            "reasonCode",
                                            "reasonReference",
                                            "priority",
                                            "description",
                                            "supportingInformation",
                                            "start",
                                            "end",
                                            "minutesDuration",
                                            "slot",
                                            "created",
                                            "comment",
                                            "patientInstruction",
                                            "basedOn",
                                            "participant",
                                            "requestedPeriod",
                                        ],
                                    ));
                                }
                            }
                            Field::Description => {
                                if _ctx.from_json {
                                    let some = r#description.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "description",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#description.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "description",
                                        ));
                                    }
                                    r#description = Some(map_access.next_value()?);
                                }
                            }
                            Field::DescriptionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#description.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_description",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "description",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "status",
                                            "cancelationReason",
                                            "serviceCategory",
                                            "serviceType",
                                            "specialty",
                                            "appointmentType",
                                            "reasonCode",
                                            "reasonReference",
                                            "priority",
                                            "description",
                                            "supportingInformation",
                                            "start",
                                            "end",
                                            "minutesDuration",
                                            "slot",
                                            "created",
                                            "comment",
                                            "patientInstruction",
                                            "basedOn",
                                            "participant",
                                            "requestedPeriod",
                                        ],
                                    ));
                                }
                            }
                            Field::SupportingInformation => {
                                if _ctx.from_json {
                                    if r#supporting_information.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "supportingInformation",
                                        ));
                                    }
                                    r#supporting_information = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#supporting_information.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Start => {
                                if _ctx.from_json {
                                    let some = r#start.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("start"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#start.is_some() {
                                        return Err(serde::de::Error::duplicate_field("start"));
                                    }
                                    r#start = Some(map_access.next_value()?);
                                }
                            }
                            Field::StartPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#start.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_start"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "start",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "status",
                                            "cancelationReason",
                                            "serviceCategory",
                                            "serviceType",
                                            "specialty",
                                            "appointmentType",
                                            "reasonCode",
                                            "reasonReference",
                                            "priority",
                                            "description",
                                            "supportingInformation",
                                            "start",
                                            "end",
                                            "minutesDuration",
                                            "slot",
                                            "created",
                                            "comment",
                                            "patientInstruction",
                                            "basedOn",
                                            "participant",
                                            "requestedPeriod",
                                        ],
                                    ));
                                }
                            }
                            Field::End => {
                                if _ctx.from_json {
                                    let some = r#end.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("end"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#end.is_some() {
                                        return Err(serde::de::Error::duplicate_field("end"));
                                    }
                                    r#end = Some(map_access.next_value()?);
                                }
                            }
                            Field::EndPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#end.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_end"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "end",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "status",
                                            "cancelationReason",
                                            "serviceCategory",
                                            "serviceType",
                                            "specialty",
                                            "appointmentType",
                                            "reasonCode",
                                            "reasonReference",
                                            "priority",
                                            "description",
                                            "supportingInformation",
                                            "start",
                                            "end",
                                            "minutesDuration",
                                            "slot",
                                            "created",
                                            "comment",
                                            "patientInstruction",
                                            "basedOn",
                                            "participant",
                                            "requestedPeriod",
                                        ],
                                    ));
                                }
                            }
                            Field::MinutesDuration => {
                                if _ctx.from_json {
                                    let some = r#minutes_duration.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "minutesDuration",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#minutes_duration.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "minutesDuration",
                                        ));
                                    }
                                    r#minutes_duration = Some(map_access.next_value()?);
                                }
                            }
                            Field::MinutesDurationPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#minutes_duration.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_minutesDuration",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "minutesDuration",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "status",
                                            "cancelationReason",
                                            "serviceCategory",
                                            "serviceType",
                                            "specialty",
                                            "appointmentType",
                                            "reasonCode",
                                            "reasonReference",
                                            "priority",
                                            "description",
                                            "supportingInformation",
                                            "start",
                                            "end",
                                            "minutesDuration",
                                            "slot",
                                            "created",
                                            "comment",
                                            "patientInstruction",
                                            "basedOn",
                                            "participant",
                                            "requestedPeriod",
                                        ],
                                    ));
                                }
                            }
                            Field::Slot => {
                                if _ctx.from_json {
                                    if r#slot.is_some() {
                                        return Err(serde::de::Error::duplicate_field("slot"));
                                    }
                                    r#slot = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#slot.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Created => {
                                if _ctx.from_json {
                                    let some = r#created.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("created"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#created.is_some() {
                                        return Err(serde::de::Error::duplicate_field("created"));
                                    }
                                    r#created = Some(map_access.next_value()?);
                                }
                            }
                            Field::CreatedPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#created.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_created"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "created",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "status",
                                            "cancelationReason",
                                            "serviceCategory",
                                            "serviceType",
                                            "specialty",
                                            "appointmentType",
                                            "reasonCode",
                                            "reasonReference",
                                            "priority",
                                            "description",
                                            "supportingInformation",
                                            "start",
                                            "end",
                                            "minutesDuration",
                                            "slot",
                                            "created",
                                            "comment",
                                            "patientInstruction",
                                            "basedOn",
                                            "participant",
                                            "requestedPeriod",
                                        ],
                                    ));
                                }
                            }
                            Field::Comment => {
                                if _ctx.from_json {
                                    let some = r#comment.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("comment"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#comment.is_some() {
                                        return Err(serde::de::Error::duplicate_field("comment"));
                                    }
                                    r#comment = Some(map_access.next_value()?);
                                }
                            }
                            Field::CommentPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#comment.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_comment"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "comment",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "status",
                                            "cancelationReason",
                                            "serviceCategory",
                                            "serviceType",
                                            "specialty",
                                            "appointmentType",
                                            "reasonCode",
                                            "reasonReference",
                                            "priority",
                                            "description",
                                            "supportingInformation",
                                            "start",
                                            "end",
                                            "minutesDuration",
                                            "slot",
                                            "created",
                                            "comment",
                                            "patientInstruction",
                                            "basedOn",
                                            "participant",
                                            "requestedPeriod",
                                        ],
                                    ));
                                }
                            }
                            Field::PatientInstruction => {
                                if _ctx.from_json {
                                    let some =
                                        r#patient_instruction.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patientInstruction",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#patient_instruction.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patientInstruction",
                                        ));
                                    }
                                    r#patient_instruction = Some(map_access.next_value()?);
                                }
                            }
                            Field::PatientInstructionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#patient_instruction.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patientInstruction",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "patientInstruction",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "status",
                                            "cancelationReason",
                                            "serviceCategory",
                                            "serviceType",
                                            "specialty",
                                            "appointmentType",
                                            "reasonCode",
                                            "reasonReference",
                                            "priority",
                                            "description",
                                            "supportingInformation",
                                            "start",
                                            "end",
                                            "minutesDuration",
                                            "slot",
                                            "created",
                                            "comment",
                                            "patientInstruction",
                                            "basedOn",
                                            "participant",
                                            "requestedPeriod",
                                        ],
                                    ));
                                }
                            }
                            Field::BasedOn => {
                                if _ctx.from_json {
                                    if r#based_on.is_some() {
                                        return Err(serde::de::Error::duplicate_field("basedOn"));
                                    }
                                    r#based_on = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#based_on.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Participant => {
                                if _ctx.from_json {
                                    if r#participant.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "participant",
                                        ));
                                    }
                                    r#participant = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#participant.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::RequestedPeriod => {
                                if _ctx.from_json {
                                    if r#requested_period.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "requestedPeriod",
                                        ));
                                    }
                                    r#requested_period = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#requested_period.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "meta",
                                        "implicitRules",
                                        "language",
                                        "text",
                                        "contained",
                                        "extension",
                                        "modifierExtension",
                                        "identifier",
                                        "status",
                                        "cancelationReason",
                                        "serviceCategory",
                                        "serviceType",
                                        "specialty",
                                        "appointmentType",
                                        "reasonCode",
                                        "reasonReference",
                                        "priority",
                                        "description",
                                        "supportingInformation",
                                        "start",
                                        "end",
                                        "minutesDuration",
                                        "slot",
                                        "created",
                                        "comment",
                                        "patientInstruction",
                                        "basedOn",
                                        "participant",
                                        "requestedPeriod",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(Appointment {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#status: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#cancelation_reason,
                        r#service_category: r#service_category.unwrap_or(vec![]),
                        r#service_type: r#service_type.unwrap_or(vec![]),
                        r#specialty: r#specialty.unwrap_or(vec![]),
                        r#appointment_type,
                        r#reason_code: r#reason_code.unwrap_or(vec![]),
                        r#reason_reference: r#reason_reference.unwrap_or(vec![]),
                        r#priority,
                        r#description,
                        r#supporting_information: r#supporting_information.unwrap_or(vec![]),
                        r#start,
                        r#end,
                        r#minutes_duration,
                        r#slot: r#slot.unwrap_or(vec![]),
                        r#created,
                        r#comment,
                        r#patient_instruction,
                        r#based_on: r#based_on.unwrap_or(vec![]),
                        r#participant: r#participant.unwrap_or(vec![]),
                        r#requested_period: r#requested_period.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
