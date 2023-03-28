// Generated on 2023-03-28 by fhirbolt-codegen v0.1.0
#[doc = "Estimated or actual date, date-time, period, or age when the procedure was performed.  Allows a period to support complex procedures that span more than one date, and also allows for the length of the procedure to be captured."]
#[derive(Debug, Clone)]
pub enum ProcedurePerformed {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    String(Box<super::super::types::String>),
    Age(Box<super::super::types::Age>),
    Range(Box<super::super::types::Range>),
    Invalid,
}
impl Default for ProcedurePerformed {
    fn default() -> ProcedurePerformed {
        ProcedurePerformed::Invalid
    }
}
#[doc = "Limited to \"real\" people rather than equipment."]
#[derive(Default, Debug, Clone)]
pub struct ProcedurePerformer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Distinguishes the type of involvement of the performer in the procedure. For example, surgeon, anaesthetist, endoscopist."]
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The practitioner who was involved in the procedure."]
    pub r#actor: Box<super::super::types::Reference>,
    #[doc = "The organization the device or practitioner was acting on behalf of."]
    pub r#on_behalf_of: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for ProcedurePerformer {
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
            if let Some(some) = self.r#function.as_ref() {
                state.serialize_entry("function", some)?;
            }
            state.serialize_entry("actor", &self.r#actor)?;
            if let Some(some) = self.r#on_behalf_of.as_ref() {
                state.serialize_entry("onBehalfOf", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ProcedurePerformer {
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
            #[serde(rename = "function")]
            Function,
            #[serde(rename = "actor")]
            Actor,
            #[serde(rename = "onBehalfOf")]
            OnBehalfOf,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ProcedurePerformer;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ProcedurePerformer")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ProcedurePerformer, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#function: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#actor: Option<Box<super::super::types::Reference>> = None;
                let mut r#on_behalf_of: Option<Box<super::super::types::Reference>> = None;
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
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Function => {
                                if r#function.is_some() {
                                    return Err(serde::de::Error::duplicate_field("function"));
                                }
                                r#function = Some(map_access.next_value()?);
                            }
                            Field::Actor => {
                                if r#actor.is_some() {
                                    return Err(serde::de::Error::duplicate_field("actor"));
                                }
                                r#actor = Some(map_access.next_value()?);
                            }
                            Field::OnBehalfOf => {
                                if r#on_behalf_of.is_some() {
                                    return Err(serde::de::Error::duplicate_field("onBehalfOf"));
                                }
                                r#on_behalf_of = Some(map_access.next_value()?);
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
                                        "function",
                                        "actor",
                                        "onBehalfOf",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ProcedurePerformer {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#function,
                        r#actor: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#actor.unwrap_or(Default::default())
                        } else {
                            r#actor.ok_or(serde::de::Error::missing_field("actor"))?
                        },
                        r#on_behalf_of,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A device that is implanted, removed or otherwise manipulated (calibration, battery replacement, fitting a prosthesis, attaching a wound-vac, etc.) as a focal portion of the Procedure."]
#[derive(Default, Debug, Clone)]
pub struct ProcedureFocalDevice {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The kind of change that happened to the device during the procedure."]
    pub r#action: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The device that was manipulated (changed) during the procedure."]
    pub r#manipulated: Box<super::super::types::Reference>,
}
impl serde::ser::Serialize for ProcedureFocalDevice {
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
            if let Some(some) = self.r#action.as_ref() {
                state.serialize_entry("action", some)?;
            }
            state.serialize_entry("manipulated", &self.r#manipulated)?;
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ProcedureFocalDevice {
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
            #[serde(rename = "action")]
            Action,
            #[serde(rename = "manipulated")]
            Manipulated,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ProcedureFocalDevice;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ProcedureFocalDevice")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ProcedureFocalDevice, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#action: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#manipulated: Option<Box<super::super::types::Reference>> = None;
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
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Action => {
                                if r#action.is_some() {
                                    return Err(serde::de::Error::duplicate_field("action"));
                                }
                                r#action = Some(map_access.next_value()?);
                            }
                            Field::Manipulated => {
                                if r#manipulated.is_some() {
                                    return Err(serde::de::Error::duplicate_field("manipulated"));
                                }
                                r#manipulated = Some(map_access.next_value()?);
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
                                        "action",
                                        "manipulated",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ProcedureFocalDevice {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#action,
                        r#manipulated: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#manipulated.unwrap_or(Default::default())
                        } else {
                            r#manipulated.ok_or(serde::de::Error::missing_field("manipulated"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "An action that is or was performed on or for a patient. This can be a physical intervention like an operation, or less invasive like long term services, counseling, or hypnotherapy."]
#[derive(Default, Debug, Clone)]
pub struct Procedure {
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
    #[doc = "Business identifiers assigned to this procedure by the performer or other systems which remain constant as the resource is updated and is propagated from server to server."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The URL pointing to a FHIR-defined protocol, guideline, order set or other definition that is adhered to in whole or in part by this Procedure."]
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    #[doc = "The URL pointing to an externally maintained protocol, guideline, order set or other definition that is adhered to in whole or in part by this Procedure."]
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    #[doc = "A reference to a resource that contains details of the request for this procedure."]
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    #[doc = "A larger event of which this particular procedure is a component or step."]
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    #[doc = "A code specifying the state of the procedure. Generally, this will be the in-progress or completed state."]
    pub r#status: super::super::types::Code,
    #[doc = "Captures the reason for the current state of the procedure."]
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A code that classifies the procedure for searching, sorting and display purposes (e.g. \"Surgical Procedure\")."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The specific procedure that is performed. Use text if the exact nature of the procedure cannot be coded (e.g. \"Laparoscopic Appendectomy\")."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The person, animal or group on which the procedure was performed."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The Encounter during which this Procedure was created or performed or to which the creation of this record is tightly associated."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Estimated or actual date, date-time, period, or age when the procedure was performed.  Allows a period to support complex procedures that span more than one date, and also allows for the length of the procedure to be captured."]
    pub r#performed: Option<ProcedurePerformed>,
    #[doc = "Individual who recorded the record and takes responsibility for its content."]
    pub r#recorder: Option<Box<super::super::types::Reference>>,
    #[doc = "Individual who is making the procedure statement."]
    pub r#asserter: Option<Box<super::super::types::Reference>>,
    #[doc = "Limited to \"real\" people rather than equipment."]
    pub r#performer: Vec<ProcedurePerformer>,
    #[doc = "The location where the procedure actually happened.  E.g. a newborn at home, a tracheostomy at a restaurant."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "The coded reason why the procedure was performed. This may be a coded entity of some type, or may simply be present as text."]
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The justification of why the procedure was performed."]
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "Detailed and structured anatomical location information. Multiple locations are allowed - e.g. multiple punch biopsies of a lesion."]
    pub r#body_site: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The outcome of the procedure - did it resolve the reasons for the procedure being performed?"]
    pub r#outcome: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "This could be a histology result, pathology report, surgical report, etc."]
    pub r#report: Vec<Box<super::super::types::Reference>>,
    #[doc = "Any complications that occurred during the procedure, or in the immediate post-performance period. These are generally tracked separately from the notes, which will typically describe the procedure itself rather than any 'post procedure' issues."]
    pub r#complication: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Any complications that occurred during the procedure, or in the immediate post-performance period."]
    pub r#complication_detail: Vec<Box<super::super::types::Reference>>,
    #[doc = "If the procedure required specific follow up - e.g. removal of sutures. The follow up may be represented as a simple note or could potentially be more complex, in which case the CarePlan resource can be used."]
    pub r#follow_up: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Any other notes and comments about the procedure."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "A device that is implanted, removed or otherwise manipulated (calibration, battery replacement, fitting a prosthesis, attaching a wound-vac, etc.) as a focal portion of the Procedure."]
    pub r#focal_device: Vec<ProcedureFocalDevice>,
    #[doc = "Identifies medications, devices and any other substance used as part of the procedure."]
    pub r#used_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "Identifies coded items that were used as part of the procedure."]
    pub r#used_code: Vec<Box<super::super::types::CodeableConcept>>,
}
impl crate::AnyResource for Procedure {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R4;
}
impl serde::ser::Serialize for Procedure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "Procedure")?;
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
                if !self.r#instantiates_canonical.is_empty() {
                    let values = self
                        .r#instantiates_canonical
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("instantiatesCanonical", &values)?;
                    }
                    let requires_elements = self
                        .r#instantiates_canonical
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#instantiates_canonical
                            .iter()
                            .map(|e| {
                                if e.id.is_some() || !e.extension.is_empty() {
                                    Some(super::super::serde_helpers::PrimitiveElement {
                                        id: e.id.as_ref(),
                                        extension: &e.extension,
                                    })
                                } else {
                                    None
                                }
                            })
                            .collect();
                        state.serialize_entry("_instantiatesCanonical", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#instantiates_canonical.is_empty() {
                    state
                        .serialize_entry("instantiatesCanonical", &self.r#instantiates_canonical)?;
                }
            }
            if _ctx.output_json {
                if !self.r#instantiates_uri.is_empty() {
                    let values = self
                        .r#instantiates_uri
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("instantiatesUri", &values)?;
                    }
                    let requires_elements = self
                        .r#instantiates_uri
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#instantiates_uri
                            .iter()
                            .map(|e| {
                                if e.id.is_some() || !e.extension.is_empty() {
                                    Some(super::super::serde_helpers::PrimitiveElement {
                                        id: e.id.as_ref(),
                                        extension: &e.extension,
                                    })
                                } else {
                                    None
                                }
                            })
                            .collect();
                        state.serialize_entry("_instantiatesUri", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#instantiates_uri.is_empty() {
                    state.serialize_entry("instantiatesUri", &self.r#instantiates_uri)?;
                }
            }
            if !self.r#based_on.is_empty() {
                state.serialize_entry("basedOn", &self.r#based_on)?;
            }
            if !self.r#part_of.is_empty() {
                state.serialize_entry("partOf", &self.r#part_of)?;
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
            if let Some(some) = self.r#status_reason.as_ref() {
                state.serialize_entry("statusReason", some)?;
            }
            if let Some(some) = self.r#category.as_ref() {
                state.serialize_entry("category", some)?;
            }
            if let Some(some) = self.r#code.as_ref() {
                state.serialize_entry("code", some)?;
            }
            state.serialize_entry("subject", &self.r#subject)?;
            if let Some(some) = self.r#encounter.as_ref() {
                state.serialize_entry("encounter", some)?;
            }
            if let Some(some) = self.r#performed.as_ref() {
                match some {
                    ProcedurePerformed::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("performedDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_performedDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("performedDateTime", value)?;
                        }
                    }
                    ProcedurePerformed::Period(ref value) => {
                        state.serialize_entry("performedPeriod", value)?;
                    }
                    ProcedurePerformed::String(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("performedString", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_performedString", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("performedString", value)?;
                        }
                    }
                    ProcedurePerformed::Age(ref value) => {
                        state.serialize_entry("performedAge", value)?;
                    }
                    ProcedurePerformed::Range(ref value) => {
                        state.serialize_entry("performedRange", value)?;
                    }
                    ProcedurePerformed::Invalid => {
                        return Err(serde::ser::Error::custom("performed is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#recorder.as_ref() {
                state.serialize_entry("recorder", some)?;
            }
            if let Some(some) = self.r#asserter.as_ref() {
                state.serialize_entry("asserter", some)?;
            }
            if !self.r#performer.is_empty() {
                state.serialize_entry("performer", &self.r#performer)?;
            }
            if let Some(some) = self.r#location.as_ref() {
                state.serialize_entry("location", some)?;
            }
            if !self.r#reason_code.is_empty() {
                state.serialize_entry("reasonCode", &self.r#reason_code)?;
            }
            if !self.r#reason_reference.is_empty() {
                state.serialize_entry("reasonReference", &self.r#reason_reference)?;
            }
            if !self.r#body_site.is_empty() {
                state.serialize_entry("bodySite", &self.r#body_site)?;
            }
            if let Some(some) = self.r#outcome.as_ref() {
                state.serialize_entry("outcome", some)?;
            }
            if !self.r#report.is_empty() {
                state.serialize_entry("report", &self.r#report)?;
            }
            if !self.r#complication.is_empty() {
                state.serialize_entry("complication", &self.r#complication)?;
            }
            if !self.r#complication_detail.is_empty() {
                state.serialize_entry("complicationDetail", &self.r#complication_detail)?;
            }
            if !self.r#follow_up.is_empty() {
                state.serialize_entry("followUp", &self.r#follow_up)?;
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            if !self.r#focal_device.is_empty() {
                state.serialize_entry("focalDevice", &self.r#focal_device)?;
            }
            if !self.r#used_reference.is_empty() {
                state.serialize_entry("usedReference", &self.r#used_reference)?;
            }
            if !self.r#used_code.is_empty() {
                state.serialize_entry("usedCode", &self.r#used_code)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for Procedure {
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
            #[serde(rename = "instantiatesCanonical")]
            InstantiatesCanonical,
            #[serde(rename = "_instantiatesCanonical")]
            InstantiatesCanonicalPrimitiveElement,
            #[serde(rename = "instantiatesUri")]
            InstantiatesUri,
            #[serde(rename = "_instantiatesUri")]
            InstantiatesUriPrimitiveElement,
            #[serde(rename = "basedOn")]
            BasedOn,
            #[serde(rename = "partOf")]
            PartOf,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "statusReason")]
            StatusReason,
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "encounter")]
            Encounter,
            #[serde(rename = "performedDateTime")]
            PerformedDateTime,
            #[serde(rename = "_performedDateTime")]
            PerformedDateTimePrimitiveElement,
            #[serde(rename = "performedPeriod")]
            PerformedPeriod,
            #[serde(rename = "performedString")]
            PerformedString,
            #[serde(rename = "_performedString")]
            PerformedStringPrimitiveElement,
            #[serde(rename = "performedAge")]
            PerformedAge,
            #[serde(rename = "performedRange")]
            PerformedRange,
            #[serde(rename = "recorder")]
            Recorder,
            #[serde(rename = "asserter")]
            Asserter,
            #[serde(rename = "performer")]
            Performer,
            #[serde(rename = "location")]
            Location,
            #[serde(rename = "reasonCode")]
            ReasonCode,
            #[serde(rename = "reasonReference")]
            ReasonReference,
            #[serde(rename = "bodySite")]
            BodySite,
            #[serde(rename = "outcome")]
            Outcome,
            #[serde(rename = "report")]
            Report,
            #[serde(rename = "complication")]
            Complication,
            #[serde(rename = "complicationDetail")]
            ComplicationDetail,
            #[serde(rename = "followUp")]
            FollowUp,
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "focalDevice")]
            FocalDevice,
            #[serde(rename = "usedReference")]
            UsedReference,
            #[serde(rename = "usedCode")]
            UsedCode,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Procedure;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Procedure")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Procedure, V::Error>
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
                let mut r#instantiates_canonical: Option<Vec<super::super::types::Canonical>> =
                    None;
                let mut r#instantiates_uri: Option<Vec<super::super::types::Uri>> = None;
                let mut r#based_on: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#part_of: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#status_reason: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#category: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#encounter: Option<Box<super::super::types::Reference>> = None;
                let mut r#performed: Option<ProcedurePerformed> = None;
                let mut r#recorder: Option<Box<super::super::types::Reference>> = None;
                let mut r#asserter: Option<Box<super::super::types::Reference>> = None;
                let mut r#performer: Option<Vec<ProcedurePerformer>> = None;
                let mut r#location: Option<Box<super::super::types::Reference>> = None;
                let mut r#reason_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#reason_reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#body_site: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#outcome: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#report: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#complication: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#complication_detail: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#follow_up: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#focal_device: Option<Vec<ProcedureFocalDevice>> = None;
                let mut r#used_reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#used_code: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "Procedure" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"Procedure",
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
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "basedOn",
                                            "partOf",
                                            "status",
                                            "statusReason",
                                            "category",
                                            "code",
                                            "subject",
                                            "encounter",
                                            "performedDateTime",
                                            "performedPeriod",
                                            "performedString",
                                            "performedAge",
                                            "performedRange",
                                            "recorder",
                                            "asserter",
                                            "performer",
                                            "location",
                                            "reasonCode",
                                            "reasonReference",
                                            "bodySite",
                                            "outcome",
                                            "report",
                                            "complication",
                                            "complicationDetail",
                                            "followUp",
                                            "note",
                                            "focalDevice",
                                            "usedReference",
                                            "usedCode",
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
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "basedOn",
                                            "partOf",
                                            "status",
                                            "statusReason",
                                            "category",
                                            "code",
                                            "subject",
                                            "encounter",
                                            "performedDateTime",
                                            "performedPeriod",
                                            "performedString",
                                            "performedAge",
                                            "performedRange",
                                            "recorder",
                                            "asserter",
                                            "performer",
                                            "location",
                                            "reasonCode",
                                            "reasonReference",
                                            "bodySite",
                                            "outcome",
                                            "report",
                                            "complication",
                                            "complicationDetail",
                                            "followUp",
                                            "note",
                                            "focalDevice",
                                            "usedReference",
                                            "usedCode",
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
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                r#contained = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
                            }
                            Field::InstantiatesCanonical => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#instantiates_canonical.get_or_insert(
                                        std::iter::repeat(Default::default())
                                            .take(values.len())
                                            .collect::<Vec<_>>(),
                                    );
                                    if vec.len() != values.len() {
                                        return Err(serde::de::Error::invalid_length(
                                            values.len(),
                                            &"primitive elements length",
                                        ));
                                    }
                                    if vec.iter().any(|v| v.value.is_some()) {
                                        return Err(serde::de::Error::duplicate_field(
                                            "instantiatesCanonical",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#instantiates_canonical.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "instantiatesCanonical",
                                        ));
                                    }
                                    r#instantiates_canonical = Some(map_access.next_value()?);
                                }
                            }
                            Field::InstantiatesCanonicalPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#instantiates_canonical.get_or_insert(
                                        std::iter::repeat(Default::default())
                                            .take(elements.len())
                                            .collect::<Vec<_>>(),
                                    );
                                    if vec.len() != elements.len() {
                                        return Err(serde::de::Error::invalid_length(
                                            elements.len(),
                                            &"primitive values length",
                                        ));
                                    }
                                    if vec
                                        .iter()
                                        .any(|e| e.id.is_some() || !e.extension.is_empty())
                                    {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_instantiatesCanonical",
                                        ));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "instantiatesCanonical",
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
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "basedOn",
                                            "partOf",
                                            "status",
                                            "statusReason",
                                            "category",
                                            "code",
                                            "subject",
                                            "encounter",
                                            "performedDateTime",
                                            "performedPeriod",
                                            "performedString",
                                            "performedAge",
                                            "performedRange",
                                            "recorder",
                                            "asserter",
                                            "performer",
                                            "location",
                                            "reasonCode",
                                            "reasonReference",
                                            "bodySite",
                                            "outcome",
                                            "report",
                                            "complication",
                                            "complicationDetail",
                                            "followUp",
                                            "note",
                                            "focalDevice",
                                            "usedReference",
                                            "usedCode",
                                        ],
                                    ));
                                }
                            }
                            Field::InstantiatesUri => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#instantiates_uri.get_or_insert(
                                        std::iter::repeat(Default::default())
                                            .take(values.len())
                                            .collect::<Vec<_>>(),
                                    );
                                    if vec.len() != values.len() {
                                        return Err(serde::de::Error::invalid_length(
                                            values.len(),
                                            &"primitive elements length",
                                        ));
                                    }
                                    if vec.iter().any(|v| v.value.is_some()) {
                                        return Err(serde::de::Error::duplicate_field(
                                            "instantiatesUri",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#instantiates_uri.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "instantiatesUri",
                                        ));
                                    }
                                    r#instantiates_uri = Some(map_access.next_value()?);
                                }
                            }
                            Field::InstantiatesUriPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#instantiates_uri.get_or_insert(
                                        std::iter::repeat(Default::default())
                                            .take(elements.len())
                                            .collect::<Vec<_>>(),
                                    );
                                    if vec.len() != elements.len() {
                                        return Err(serde::de::Error::invalid_length(
                                            elements.len(),
                                            &"primitive values length",
                                        ));
                                    }
                                    if vec
                                        .iter()
                                        .any(|e| e.id.is_some() || !e.extension.is_empty())
                                    {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_instantiatesUri",
                                        ));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "instantiatesUri",
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
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "basedOn",
                                            "partOf",
                                            "status",
                                            "statusReason",
                                            "category",
                                            "code",
                                            "subject",
                                            "encounter",
                                            "performedDateTime",
                                            "performedPeriod",
                                            "performedString",
                                            "performedAge",
                                            "performedRange",
                                            "recorder",
                                            "asserter",
                                            "performer",
                                            "location",
                                            "reasonCode",
                                            "reasonReference",
                                            "bodySite",
                                            "outcome",
                                            "report",
                                            "complication",
                                            "complicationDetail",
                                            "followUp",
                                            "note",
                                            "focalDevice",
                                            "usedReference",
                                            "usedCode",
                                        ],
                                    ));
                                }
                            }
                            Field::BasedOn => {
                                if r#based_on.is_some() {
                                    return Err(serde::de::Error::duplicate_field("basedOn"));
                                }
                                r#based_on = Some(map_access.next_value()?);
                            }
                            Field::PartOf => {
                                if r#part_of.is_some() {
                                    return Err(serde::de::Error::duplicate_field("partOf"));
                                }
                                r#part_of = Some(map_access.next_value()?);
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
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "basedOn",
                                            "partOf",
                                            "status",
                                            "statusReason",
                                            "category",
                                            "code",
                                            "subject",
                                            "encounter",
                                            "performedDateTime",
                                            "performedPeriod",
                                            "performedString",
                                            "performedAge",
                                            "performedRange",
                                            "recorder",
                                            "asserter",
                                            "performer",
                                            "location",
                                            "reasonCode",
                                            "reasonReference",
                                            "bodySite",
                                            "outcome",
                                            "report",
                                            "complication",
                                            "complicationDetail",
                                            "followUp",
                                            "note",
                                            "focalDevice",
                                            "usedReference",
                                            "usedCode",
                                        ],
                                    ));
                                }
                            }
                            Field::StatusReason => {
                                if r#status_reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statusReason"));
                                }
                                r#status_reason = Some(map_access.next_value()?);
                            }
                            Field::Category => {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                r#category = Some(map_access.next_value()?);
                            }
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
                            }
                            Field::Subject => {
                                if r#subject.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subject"));
                                }
                                r#subject = Some(map_access.next_value()?);
                            }
                            Field::Encounter => {
                                if r#encounter.is_some() {
                                    return Err(serde::de::Error::duplicate_field("encounter"));
                                }
                                r#encounter = Some(map_access.next_value()?);
                            }
                            Field::PerformedDateTime => {
                                if _ctx.from_json {
                                    let r#enum = r#performed.get_or_insert(
                                        ProcedurePerformed::DateTime(Default::default()),
                                    );
                                    if let ProcedurePerformed::DateTime(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "performedDateTime",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "performed[x]",
                                        ));
                                    }
                                } else {
                                    if r#performed.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "performedDateTime",
                                        ));
                                    }
                                    r#performed = Some(ProcedurePerformed::DateTime(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::PerformedDateTimePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#performed.get_or_insert(
                                        ProcedurePerformed::DateTime(Default::default()),
                                    );
                                    if let ProcedurePerformed::DateTime(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_performedDateTime",
                                            ));
                                        }
                                        let super::super::serde_helpers::PrimitiveElementOwned {
                                            id,
                                            extension,
                                        } = map_access.next_value()?;
                                        variant.id = id;
                                        variant.extension = extension;
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_performed[x]",
                                        ));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "performedDateTime",
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
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "basedOn",
                                            "partOf",
                                            "status",
                                            "statusReason",
                                            "category",
                                            "code",
                                            "subject",
                                            "encounter",
                                            "performedDateTime",
                                            "performedPeriod",
                                            "performedString",
                                            "performedAge",
                                            "performedRange",
                                            "recorder",
                                            "asserter",
                                            "performer",
                                            "location",
                                            "reasonCode",
                                            "reasonReference",
                                            "bodySite",
                                            "outcome",
                                            "report",
                                            "complication",
                                            "complicationDetail",
                                            "followUp",
                                            "note",
                                            "focalDevice",
                                            "usedReference",
                                            "usedCode",
                                        ],
                                    ));
                                }
                            }
                            Field::PerformedPeriod => {
                                if r#performed.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "performedPeriod",
                                    ));
                                }
                                r#performed =
                                    Some(ProcedurePerformed::Period(map_access.next_value()?));
                            }
                            Field::PerformedString => {
                                if _ctx.from_json {
                                    let r#enum = r#performed.get_or_insert(
                                        ProcedurePerformed::String(Default::default()),
                                    );
                                    if let ProcedurePerformed::String(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "performedString",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "performed[x]",
                                        ));
                                    }
                                } else {
                                    if r#performed.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "performedString",
                                        ));
                                    }
                                    r#performed =
                                        Some(ProcedurePerformed::String(map_access.next_value()?));
                                }
                            }
                            Field::PerformedStringPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#performed.get_or_insert(
                                        ProcedurePerformed::String(Default::default()),
                                    );
                                    if let ProcedurePerformed::String(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_performedString",
                                            ));
                                        }
                                        let super::super::serde_helpers::PrimitiveElementOwned {
                                            id,
                                            extension,
                                        } = map_access.next_value()?;
                                        variant.id = id;
                                        variant.extension = extension;
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_performed[x]",
                                        ));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "performedString",
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
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "basedOn",
                                            "partOf",
                                            "status",
                                            "statusReason",
                                            "category",
                                            "code",
                                            "subject",
                                            "encounter",
                                            "performedDateTime",
                                            "performedPeriod",
                                            "performedString",
                                            "performedAge",
                                            "performedRange",
                                            "recorder",
                                            "asserter",
                                            "performer",
                                            "location",
                                            "reasonCode",
                                            "reasonReference",
                                            "bodySite",
                                            "outcome",
                                            "report",
                                            "complication",
                                            "complicationDetail",
                                            "followUp",
                                            "note",
                                            "focalDevice",
                                            "usedReference",
                                            "usedCode",
                                        ],
                                    ));
                                }
                            }
                            Field::PerformedAge => {
                                if r#performed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("performedAge"));
                                }
                                r#performed =
                                    Some(ProcedurePerformed::Age(map_access.next_value()?));
                            }
                            Field::PerformedRange => {
                                if r#performed.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "performedRange",
                                    ));
                                }
                                r#performed =
                                    Some(ProcedurePerformed::Range(map_access.next_value()?));
                            }
                            Field::Recorder => {
                                if r#recorder.is_some() {
                                    return Err(serde::de::Error::duplicate_field("recorder"));
                                }
                                r#recorder = Some(map_access.next_value()?);
                            }
                            Field::Asserter => {
                                if r#asserter.is_some() {
                                    return Err(serde::de::Error::duplicate_field("asserter"));
                                }
                                r#asserter = Some(map_access.next_value()?);
                            }
                            Field::Performer => {
                                if r#performer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("performer"));
                                }
                                r#performer = Some(map_access.next_value()?);
                            }
                            Field::Location => {
                                if r#location.is_some() {
                                    return Err(serde::de::Error::duplicate_field("location"));
                                }
                                r#location = Some(map_access.next_value()?);
                            }
                            Field::ReasonCode => {
                                if r#reason_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reasonCode"));
                                }
                                r#reason_code = Some(map_access.next_value()?);
                            }
                            Field::ReasonReference => {
                                if r#reason_reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "reasonReference",
                                    ));
                                }
                                r#reason_reference = Some(map_access.next_value()?);
                            }
                            Field::BodySite => {
                                if r#body_site.is_some() {
                                    return Err(serde::de::Error::duplicate_field("bodySite"));
                                }
                                r#body_site = Some(map_access.next_value()?);
                            }
                            Field::Outcome => {
                                if r#outcome.is_some() {
                                    return Err(serde::de::Error::duplicate_field("outcome"));
                                }
                                r#outcome = Some(map_access.next_value()?);
                            }
                            Field::Report => {
                                if r#report.is_some() {
                                    return Err(serde::de::Error::duplicate_field("report"));
                                }
                                r#report = Some(map_access.next_value()?);
                            }
                            Field::Complication => {
                                if r#complication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("complication"));
                                }
                                r#complication = Some(map_access.next_value()?);
                            }
                            Field::ComplicationDetail => {
                                if r#complication_detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "complicationDetail",
                                    ));
                                }
                                r#complication_detail = Some(map_access.next_value()?);
                            }
                            Field::FollowUp => {
                                if r#follow_up.is_some() {
                                    return Err(serde::de::Error::duplicate_field("followUp"));
                                }
                                r#follow_up = Some(map_access.next_value()?);
                            }
                            Field::Note => {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some(map_access.next_value()?);
                            }
                            Field::FocalDevice => {
                                if r#focal_device.is_some() {
                                    return Err(serde::de::Error::duplicate_field("focalDevice"));
                                }
                                r#focal_device = Some(map_access.next_value()?);
                            }
                            Field::UsedReference => {
                                if r#used_reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field("usedReference"));
                                }
                                r#used_reference = Some(map_access.next_value()?);
                            }
                            Field::UsedCode => {
                                if r#used_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("usedCode"));
                                }
                                r#used_code = Some(map_access.next_value()?);
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
                                        "instantiatesCanonical",
                                        "instantiatesUri",
                                        "basedOn",
                                        "partOf",
                                        "status",
                                        "statusReason",
                                        "category",
                                        "code",
                                        "subject",
                                        "encounter",
                                        "performedDateTime",
                                        "performedPeriod",
                                        "performedString",
                                        "performedAge",
                                        "performedRange",
                                        "recorder",
                                        "asserter",
                                        "performer",
                                        "location",
                                        "reasonCode",
                                        "reasonReference",
                                        "bodySite",
                                        "outcome",
                                        "report",
                                        "complication",
                                        "complicationDetail",
                                        "followUp",
                                        "note",
                                        "focalDevice",
                                        "usedReference",
                                        "usedCode",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(Procedure {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#instantiates_canonical: r#instantiates_canonical.unwrap_or(vec![]),
                        r#instantiates_uri: r#instantiates_uri.unwrap_or(vec![]),
                        r#based_on: r#based_on.unwrap_or(vec![]),
                        r#part_of: r#part_of.unwrap_or(vec![]),
                        r#status: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#status_reason,
                        r#category,
                        r#code,
                        r#subject: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#subject.unwrap_or(Default::default())
                        } else {
                            r#subject.ok_or(serde::de::Error::missing_field("subject"))?
                        },
                        r#encounter,
                        r#performed,
                        r#recorder,
                        r#asserter,
                        r#performer: r#performer.unwrap_or(vec![]),
                        r#location,
                        r#reason_code: r#reason_code.unwrap_or(vec![]),
                        r#reason_reference: r#reason_reference.unwrap_or(vec![]),
                        r#body_site: r#body_site.unwrap_or(vec![]),
                        r#outcome,
                        r#report: r#report.unwrap_or(vec![]),
                        r#complication: r#complication.unwrap_or(vec![]),
                        r#complication_detail: r#complication_detail.unwrap_or(vec![]),
                        r#follow_up: r#follow_up.unwrap_or(vec![]),
                        r#note: r#note.unwrap_or(vec![]),
                        r#focal_device: r#focal_device.unwrap_or(vec![]),
                        r#used_reference: r#used_reference.unwrap_or(vec![]),
                        r#used_code: r#used_code.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
