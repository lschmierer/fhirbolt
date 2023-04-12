// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "Identifies the medication being administered. This is either a link to a resource representing the details of the medication or a simple attribute carrying a code that identifies the medication from a known list of medications."]
#[derive(Debug, Clone, PartialEq)]
pub enum MedicationStatementMedication {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for MedicationStatementMedication {
    fn default() -> MedicationStatementMedication {
        MedicationStatementMedication::Invalid
    }
}
#[doc = "The interval of time during which it is being asserted that the patient is/was/will be taking the medication (or was not taking, when the MedicationStatement.taken element is No)."]
#[derive(Debug, Clone, PartialEq)]
pub enum MedicationStatementEffective {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for MedicationStatementEffective {
    fn default() -> MedicationStatementEffective {
        MedicationStatementEffective::Invalid
    }
}
#[doc = "A record of a medication that is being consumed by a patient.   A MedicationStatement may indicate that the patient may be taking the medication now or has taken the medication in the past or will be taking the medication in the future.  The source of this information can be the patient, significant other (such as a family member or spouse), or a clinician.  A common scenario where this information is captured is during the history taking process during a patient visit or stay.   The medication information may come from sources such as the patient's memory, from a prescription bottle,  or from a list of medications the patient, clinician or other party maintains. \n\nThe primary difference between a medication statement and a medication administration is that the medication administration has complete administration information and is based on actual administration information from the person who administered the medication.  A medication statement is often, if not always, less specific.  There is no required date/time when the medication was administered, in fact we only know that a source has reported the patient is taking this medication, where details such as time, quantity, or rate or even medication product may be incomplete or missing or less precise.  As stated earlier, the medication statement information may come from the patient's memory, from a prescription bottle or from a list of medications the patient, clinician or other party maintains.  Medication administration is more formal and is not missing detailed information."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicationStatement {
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
    #[doc = "Identifiers associated with this Medication Statement that are defined by business processes and/or used to refer to it when a direct URL reference to the resource itself is not appropriate. They are business identifiers assigned to this resource by the performer or other systems and remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "A plan, proposal or order that is fulfilled in whole or in part by this event."]
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    #[doc = "A larger event of which this particular event is a component or step."]
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    #[doc = "A code representing the patient or other source's judgment about the state of the medication used that this statement is about.  Generally, this will be active or completed."]
    pub r#status: super::super::types::Code,
    #[doc = "Captures the reason for the current state of the MedicationStatement."]
    pub r#status_reason: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates where the medication is expected to be consumed or administered."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identifies the medication being administered. This is either a link to a resource representing the details of the medication or a simple attribute carrying a code that identifies the medication from a known list of medications."]
    pub r#medication: MedicationStatementMedication,
    #[doc = "The person, animal or group who is/was taking the medication."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The encounter or episode of care that establishes the context for this MedicationStatement."]
    pub r#context: Option<Box<super::super::types::Reference>>,
    #[doc = "The interval of time during which it is being asserted that the patient is/was/will be taking the medication (or was not taking, when the MedicationStatement.taken element is No)."]
    pub r#effective: Option<MedicationStatementEffective>,
    #[doc = "The date when the medication statement was asserted by the information source."]
    pub r#date_asserted: Option<super::super::types::DateTime>,
    #[doc = "The person or organization that provided the information about the taking of this medication. Note: Use derivedFrom when a MedicationStatement is derived from other resources, e.g. Claim or MedicationRequest."]
    pub r#information_source: Option<Box<super::super::types::Reference>>,
    #[doc = "Allows linking the MedicationStatement to the underlying MedicationRequest, or to other information that supports or is used to derive the MedicationStatement."]
    pub r#derived_from: Vec<Box<super::super::types::Reference>>,
    #[doc = "A reason for why the medication is being/was taken."]
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Condition or observation that supports why the medication is being/was taken."]
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "Provides extra information about the medication statement that is not conveyed by the other attributes."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Indicates how the medication is/was or should be taken by the patient."]
    pub r#dosage: Vec<Box<super::super::types::Dosage>>,
}
impl serde::ser::Serialize for MedicationStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "MedicationStatement")?;
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
            if !self.r#status_reason.is_empty() {
                state.serialize_entry("statusReason", &self.r#status_reason)?;
            }
            if let Some(some) = self.r#category.as_ref() {
                state.serialize_entry("category", some)?;
            }
            match self.r#medication {
                MedicationStatementMedication::CodeableConcept(ref value) => {
                    state.serialize_entry("medicationCodeableConcept", value)?;
                }
                MedicationStatementMedication::Reference(ref value) => {
                    state.serialize_entry("medicationReference", value)?;
                }
                MedicationStatementMedication::Invalid => {
                    return Err(serde::ser::Error::custom("medication is a required field"))
                }
            }
            state.serialize_entry("subject", &self.r#subject)?;
            if let Some(some) = self.r#context.as_ref() {
                state.serialize_entry("context", some)?;
            }
            if let Some(some) = self.r#effective.as_ref() {
                match some {
                    MedicationStatementEffective::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("effectiveDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_effectiveDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("effectiveDateTime", value)?;
                        }
                    }
                    MedicationStatementEffective::Period(ref value) => {
                        state.serialize_entry("effectivePeriod", value)?;
                    }
                    MedicationStatementEffective::Invalid => {
                        return Err(serde::ser::Error::custom("effective is invalid"))
                    }
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#date_asserted.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("dateAsserted", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_dateAsserted", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#date_asserted.as_ref() {
                    state.serialize_entry("dateAsserted", some)?;
                }
            }
            if let Some(some) = self.r#information_source.as_ref() {
                state.serialize_entry("informationSource", some)?;
            }
            if !self.r#derived_from.is_empty() {
                state.serialize_entry("derivedFrom", &self.r#derived_from)?;
            }
            if !self.r#reason_code.is_empty() {
                state.serialize_entry("reasonCode", &self.r#reason_code)?;
            }
            if !self.r#reason_reference.is_empty() {
                state.serialize_entry("reasonReference", &self.r#reason_reference)?;
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            if !self.r#dosage.is_empty() {
                state.serialize_entry("dosage", &self.r#dosage)?;
            }
            state.end()
        })
    }
}
