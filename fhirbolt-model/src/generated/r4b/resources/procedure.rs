// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "Estimated or actual date, date-time, period, or age when the procedure was performed.  Allows a period to support complex procedures that span more than one date, and also allows for the length of the procedure to be captured."]
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Default, Debug, Clone, PartialEq)]
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
#[doc = "A device that is implanted, removed or otherwise manipulated (calibration, battery replacement, fitting a prosthesis, attaching a wound-vac, etc.) as a focal portion of the Procedure."]
#[derive(Default, Debug, Clone, PartialEq)]
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
#[doc = "An action that is or was performed on or for a patient. This can be a physical intervention like an operation, or less invasive like long term services, counseling, or hypnotherapy."]
#[derive(Default, Debug, Clone, PartialEq)]
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
