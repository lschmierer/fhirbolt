// Generated on 2023-04-10 by fhirbolt-codegen v0.1.0
#[doc = "Information on the possible cause of the event."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AdverseEventSuspectEntityCausality {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Assessment of if the entity caused the event."]
    pub r#assessment: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "AdverseEvent.suspectEntity.causalityProductRelatedness."]
    pub r#product_relatedness: Option<super::super::types::String>,
    #[doc = "AdverseEvent.suspectEntity.causalityAuthor."]
    pub r#author: Option<Box<super::super::types::Reference>>,
    #[doc = "ProbabilityScale | Bayesian | Checklist."]
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for AdverseEventSuspectEntityCausality {
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
            if let Some(some) = self.r#assessment.as_ref() {
                state.serialize_entry("assessment", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#product_relatedness.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("productRelatedness", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_productRelatedness", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#product_relatedness.as_ref() {
                    state.serialize_entry("productRelatedness", some)?;
                }
            }
            if let Some(some) = self.r#author.as_ref() {
                state.serialize_entry("author", some)?;
            }
            if let Some(some) = self.r#method.as_ref() {
                state.serialize_entry("method", some)?;
            }
            state.end()
        })
    }
}
#[doc = "Describes the entity that is suspected to have caused the adverse event."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AdverseEventSuspectEntity {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identifies the actual instance of what caused the adverse event.  May be a substance, medication, medication administration, medication statement or a device."]
    pub r#instance: Box<super::super::types::Reference>,
    #[doc = "Information on the possible cause of the event."]
    pub r#causality: Vec<AdverseEventSuspectEntityCausality>,
}
impl serde::ser::Serialize for AdverseEventSuspectEntity {
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
            state.serialize_entry("instance", &self.r#instance)?;
            if !self.r#causality.is_empty() {
                state.serialize_entry("causality", &self.r#causality)?;
            }
            state.end()
        })
    }
}
#[doc = "Actual or  potential/avoided event causing unintended physical injury resulting from or contributed to by medical care, a research study or other healthcare setting factors that requires additional monitoring, treatment, or hospitalization, or that results in death."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AdverseEvent {
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
    #[doc = "Business identifiers assigned to this adverse event by the performer or other systems which remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Whether the event actually happened, or just had the potential to. Note that this is independent of whether anyone was affected or harmed or how severely."]
    pub r#actuality: super::super::types::Code,
    #[doc = "The overall type of event, intended for search and filtering purposes."]
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "This element defines the specific type of event that occurred or that was prevented from occurring."]
    pub r#event: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "This subject or group impacted by the event."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The Encounter during which AdverseEvent was created or to which the creation of this record is tightly associated."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The date (and perhaps time) when the adverse event occurred."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "Estimated or actual date the AdverseEvent began, in the opinion of the reporter."]
    pub r#detected: Option<super::super::types::DateTime>,
    #[doc = "The date on which the existence of the AdverseEvent was first recorded."]
    pub r#recorded_date: Option<super::super::types::DateTime>,
    #[doc = "Includes information about the reaction that occurred as a result of exposure to a substance (for example, a drug or a chemical)."]
    pub r#resulting_condition: Vec<Box<super::super::types::Reference>>,
    #[doc = "The information about where the adverse event occurred."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "Assessment whether this event was of real importance."]
    pub r#seriousness: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Describes the severity of the adverse event, in relation to the subject. Contrast to AdverseEvent.seriousness - a severe rash might not be serious, but a mild heart problem is."]
    pub r#severity: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Describes the type of outcome from the adverse event."]
    pub r#outcome: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Information on who recorded the adverse event.  May be the patient or a practitioner."]
    pub r#recorder: Option<Box<super::super::types::Reference>>,
    #[doc = "Parties that may or should contribute or have contributed information to the adverse event, which can consist of one or more activities.  Such information includes information leading to the decision to perform the activity and how to perform the activity (e.g. consultant), information that the activity itself seeks to reveal (e.g. informant of clinical history), or information about what activity was performed (e.g. informant witness)."]
    pub r#contributor: Vec<Box<super::super::types::Reference>>,
    #[doc = "Describes the entity that is suspected to have caused the adverse event."]
    pub r#suspect_entity: Vec<AdverseEventSuspectEntity>,
    #[doc = "AdverseEvent.subjectMedicalHistory."]
    pub r#subject_medical_history: Vec<Box<super::super::types::Reference>>,
    #[doc = "AdverseEvent.referenceDocument."]
    pub r#reference_document: Vec<Box<super::super::types::Reference>>,
    #[doc = "AdverseEvent.study."]
    pub r#study: Vec<Box<super::super::types::Reference>>,
}
impl crate::AnyResource for AdverseEvent {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4;
}
impl serde::ser::Serialize for AdverseEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "AdverseEvent")?;
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
            if let Some(some) = self.r#identifier.as_ref() {
                state.serialize_entry("identifier", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#actuality.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("actuality", &some)?;
                }
                if self.r#actuality.id.is_some() || !self.r#actuality.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#actuality.id.as_ref(),
                        extension: &self.r#actuality.extension,
                    };
                    state.serialize_entry("_actuality", &primitive_element)?;
                }
            } else {
                state.serialize_entry("actuality", &self.r#actuality)?;
            }
            if !self.r#category.is_empty() {
                state.serialize_entry("category", &self.r#category)?;
            }
            if let Some(some) = self.r#event.as_ref() {
                state.serialize_entry("event", some)?;
            }
            state.serialize_entry("subject", &self.r#subject)?;
            if let Some(some) = self.r#encounter.as_ref() {
                state.serialize_entry("encounter", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("date", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_date", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#date.as_ref() {
                    state.serialize_entry("date", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#detected.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("detected", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_detected", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#detected.as_ref() {
                    state.serialize_entry("detected", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#recorded_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("recordedDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_recordedDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#recorded_date.as_ref() {
                    state.serialize_entry("recordedDate", some)?;
                }
            }
            if !self.r#resulting_condition.is_empty() {
                state.serialize_entry("resultingCondition", &self.r#resulting_condition)?;
            }
            if let Some(some) = self.r#location.as_ref() {
                state.serialize_entry("location", some)?;
            }
            if let Some(some) = self.r#seriousness.as_ref() {
                state.serialize_entry("seriousness", some)?;
            }
            if let Some(some) = self.r#severity.as_ref() {
                state.serialize_entry("severity", some)?;
            }
            if let Some(some) = self.r#outcome.as_ref() {
                state.serialize_entry("outcome", some)?;
            }
            if let Some(some) = self.r#recorder.as_ref() {
                state.serialize_entry("recorder", some)?;
            }
            if !self.r#contributor.is_empty() {
                state.serialize_entry("contributor", &self.r#contributor)?;
            }
            if !self.r#suspect_entity.is_empty() {
                state.serialize_entry("suspectEntity", &self.r#suspect_entity)?;
            }
            if !self.r#subject_medical_history.is_empty() {
                state.serialize_entry("subjectMedicalHistory", &self.r#subject_medical_history)?;
            }
            if !self.r#reference_document.is_empty() {
                state.serialize_entry("referenceDocument", &self.r#reference_document)?;
            }
            if !self.r#study.is_empty() {
                state.serialize_entry("study", &self.r#study)?;
            }
            state.end()
        })
    }
}
