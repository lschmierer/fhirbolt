// Generated on 2023-04-10 by fhirbolt-codegen v0.1.0
#[doc = "The intended subjects for the ResearchElementDefinition. If this element is not provided, a Patient subject is assumed, but the subject of the ResearchElementDefinition can be anything."]
#[derive(Debug, Clone, PartialEq)]
pub enum ResearchElementDefinitionSubject {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ResearchElementDefinitionSubject {
    fn default() -> ResearchElementDefinitionSubject {
        ResearchElementDefinitionSubject::Invalid
    }
}
#[doc = "Define members of the research element using Codes (such as condition, medication, or observation), Expressions ( using an expression language such as FHIRPath or CQL) or DataRequirements (such as Diabetes diagnosis onset in the last year)."]
#[derive(Debug, Clone, PartialEq)]
pub enum ResearchElementDefinitionCharacteristicDefinition {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Canonical(Box<super::super::types::Canonical>),
    Expression(Box<super::super::types::Expression>),
    DataRequirement(Box<super::super::types::DataRequirement>),
    Invalid,
}
impl Default for ResearchElementDefinitionCharacteristicDefinition {
    fn default() -> ResearchElementDefinitionCharacteristicDefinition {
        ResearchElementDefinitionCharacteristicDefinition::Invalid
    }
}
#[doc = "Indicates what effective period the study covers."]
#[derive(Debug, Clone, PartialEq)]
pub enum ResearchElementDefinitionCharacteristicStudyEffective {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Duration(Box<super::super::types::Duration>),
    Timing(Box<super::super::types::Timing>),
    Invalid,
}
impl Default for ResearchElementDefinitionCharacteristicStudyEffective {
    fn default() -> ResearchElementDefinitionCharacteristicStudyEffective {
        ResearchElementDefinitionCharacteristicStudyEffective::Invalid
    }
}
#[doc = "Indicates what effective period the study covers."]
#[derive(Debug, Clone, PartialEq)]
pub enum ResearchElementDefinitionCharacteristicParticipantEffective {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Duration(Box<super::super::types::Duration>),
    Timing(Box<super::super::types::Timing>),
    Invalid,
}
impl Default for ResearchElementDefinitionCharacteristicParticipantEffective {
    fn default() -> ResearchElementDefinitionCharacteristicParticipantEffective {
        ResearchElementDefinitionCharacteristicParticipantEffective::Invalid
    }
}
#[doc = "A characteristic that defines the members of the research element. Multiple characteristics are applied with \"and\" semantics."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ResearchElementDefinitionCharacteristic {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Define members of the research element using Codes (such as condition, medication, or observation), Expressions ( using an expression language such as FHIRPath or CQL) or DataRequirements (such as Diabetes diagnosis onset in the last year)."]
    pub r#definition: ResearchElementDefinitionCharacteristicDefinition,
    #[doc = "Use UsageContext to define the members of the population, such as Age Ranges, Genders, Settings."]
    pub r#usage_context: Vec<Box<super::super::types::UsageContext>>,
    #[doc = "When true, members with this characteristic are excluded from the element."]
    pub r#exclude: Option<super::super::types::Boolean>,
    #[doc = "Specifies the UCUM unit for the outcome."]
    pub r#unit_of_measure: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A narrative description of the time period the study covers."]
    pub r#study_effective_description: Option<super::super::types::String>,
    #[doc = "Indicates what effective period the study covers."]
    pub r#study_effective: Option<ResearchElementDefinitionCharacteristicStudyEffective>,
    #[doc = "Indicates duration from the study initiation."]
    pub r#study_effective_time_from_start: Option<Box<super::super::types::Duration>>,
    #[doc = "Indicates how elements are aggregated within the study effective period."]
    pub r#study_effective_group_measure: Option<super::super::types::Code>,
    #[doc = "A narrative description of the time period the study covers."]
    pub r#participant_effective_description: Option<super::super::types::String>,
    #[doc = "Indicates what effective period the study covers."]
    pub r#participant_effective:
        Option<ResearchElementDefinitionCharacteristicParticipantEffective>,
    #[doc = "Indicates duration from the participant's study entry."]
    pub r#participant_effective_time_from_start: Option<Box<super::super::types::Duration>>,
    #[doc = "Indicates how elements are aggregated within the study effective period."]
    pub r#participant_effective_group_measure: Option<super::super::types::Code>,
}
impl serde::ser::Serialize for ResearchElementDefinitionCharacteristic {
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
            match self.r#definition {
                ResearchElementDefinitionCharacteristicDefinition::CodeableConcept(ref value) => {
                    state.serialize_entry("definitionCodeableConcept", value)?;
                }
                ResearchElementDefinitionCharacteristicDefinition::Canonical(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("definitionCanonical", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_definitionCanonical", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("definitionCanonical", value)?;
                    }
                }
                ResearchElementDefinitionCharacteristicDefinition::Expression(ref value) => {
                    state.serialize_entry("definitionExpression", value)?;
                }
                ResearchElementDefinitionCharacteristicDefinition::DataRequirement(ref value) => {
                    state.serialize_entry("definitionDataRequirement", value)?;
                }
                ResearchElementDefinitionCharacteristicDefinition::Invalid => {
                    return Err(serde::ser::Error::custom("definition is a required field"))
                }
            }
            if !self.r#usage_context.is_empty() {
                state.serialize_entry("usageContext", &self.r#usage_context)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#exclude.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("exclude", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_exclude", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#exclude.as_ref() {
                    state.serialize_entry("exclude", some)?;
                }
            }
            if let Some(some) = self.r#unit_of_measure.as_ref() {
                state.serialize_entry("unitOfMeasure", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#study_effective_description.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("studyEffectiveDescription", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_studyEffectiveDescription", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#study_effective_description.as_ref() {
                    state.serialize_entry("studyEffectiveDescription", some)?;
                }
            }
            if let Some(some) = self.r#study_effective.as_ref() {
                match some {
                    ResearchElementDefinitionCharacteristicStudyEffective::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("studyEffectiveDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry(
                                    "_studyEffectiveDateTime",
                                    &primitive_element,
                                )?;
                            }
                        } else {
                            state.serialize_entry("studyEffectiveDateTime", value)?;
                        }
                    }
                    ResearchElementDefinitionCharacteristicStudyEffective::Period(ref value) => {
                        state.serialize_entry("studyEffectivePeriod", value)?;
                    }
                    ResearchElementDefinitionCharacteristicStudyEffective::Duration(ref value) => {
                        state.serialize_entry("studyEffectiveDuration", value)?;
                    }
                    ResearchElementDefinitionCharacteristicStudyEffective::Timing(ref value) => {
                        state.serialize_entry("studyEffectiveTiming", value)?;
                    }
                    ResearchElementDefinitionCharacteristicStudyEffective::Invalid => {
                        return Err(serde::ser::Error::custom("study_effective is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#study_effective_time_from_start.as_ref() {
                state.serialize_entry("studyEffectiveTimeFromStart", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#study_effective_group_measure.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("studyEffectiveGroupMeasure", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_studyEffectiveGroupMeasure", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#study_effective_group_measure.as_ref() {
                    state.serialize_entry("studyEffectiveGroupMeasure", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#participant_effective_description.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("participantEffectiveDescription", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry(
                            "_participantEffectiveDescription",
                            &primitive_element,
                        )?;
                    }
                }
            } else {
                if let Some(some) = self.r#participant_effective_description.as_ref() {
                    state.serialize_entry("participantEffectiveDescription", some)?;
                }
            }
            if let Some(some) = self.r#participant_effective.as_ref() {
                match some {
                    ResearchElementDefinitionCharacteristicParticipantEffective::DateTime(
                        ref value,
                    ) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("participantEffectiveDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry(
                                    "_participantEffectiveDateTime",
                                    &primitive_element,
                                )?;
                            }
                        } else {
                            state.serialize_entry("participantEffectiveDateTime", value)?;
                        }
                    }
                    ResearchElementDefinitionCharacteristicParticipantEffective::Period(
                        ref value,
                    ) => {
                        state.serialize_entry("participantEffectivePeriod", value)?;
                    }
                    ResearchElementDefinitionCharacteristicParticipantEffective::Duration(
                        ref value,
                    ) => {
                        state.serialize_entry("participantEffectiveDuration", value)?;
                    }
                    ResearchElementDefinitionCharacteristicParticipantEffective::Timing(
                        ref value,
                    ) => {
                        state.serialize_entry("participantEffectiveTiming", value)?;
                    }
                    ResearchElementDefinitionCharacteristicParticipantEffective::Invalid => {
                        return Err(serde::ser::Error::custom(
                            "participant_effective is invalid",
                        ))
                    }
                }
            }
            if let Some(some) = self.r#participant_effective_time_from_start.as_ref() {
                state.serialize_entry("participantEffectiveTimeFromStart", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#participant_effective_group_measure.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("participantEffectiveGroupMeasure", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry(
                            "_participantEffectiveGroupMeasure",
                            &primitive_element,
                        )?;
                    }
                }
            } else {
                if let Some(some) = self.r#participant_effective_group_measure.as_ref() {
                    state.serialize_entry("participantEffectiveGroupMeasure", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "The ResearchElementDefinition resource describes a \"PICO\" element that knowledge (evidence, assertion, recommendation) is about.\n\nNeed to be able to define and reuse the definition of individual elements of a research question."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ResearchElementDefinition {
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
    #[doc = "An absolute URI that is used to identify this research element definition when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which at which an authoritative instance of this research element definition is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the research element definition is stored on different servers."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "A formal identifier that is used to identify this research element definition when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The identifier that is used to identify this version of the research element definition when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the research element definition author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence. To provide a version consistent with the Decision Support Service specification, use the format Major.Minor.Revision (e.g. 1.0.0). For more information on versioning knowledge assets, refer to the Decision Support Service specification. Note that a version is required for non-experimental active artifacts."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "A natural language name identifying the research element definition. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for the research element definition."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The short title provides an alternate title for use in informal descriptive contexts where the full, formal title is not necessary."]
    pub r#short_title: Option<super::super::types::String>,
    #[doc = "An explanatory or alternate title for the ResearchElementDefinition giving additional information about its content."]
    pub r#subtitle: Option<super::super::types::String>,
    #[doc = "The status of this research element definition. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this research element definition is authored for testing purposes (or education/evaluation/marketing) and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "The intended subjects for the ResearchElementDefinition. If this element is not provided, a Patient subject is assumed, but the subject of the ResearchElementDefinition can be anything."]
    pub r#subject: Option<ResearchElementDefinitionSubject>,
    #[doc = "The date  (and optionally time) when the research element definition was published. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the research element definition changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual that published the research element definition."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "A free text natural language description of the research element definition from a consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "A human-readable string to clarify or explain concepts about the resource."]
    pub r#comment: Vec<super::super::types::String>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate research element definition instances."]
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    #[doc = "A legal or geographic region in which the research element definition is intended to be used."]
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Explanation of why this research element definition is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "A detailed description, from a clinical perspective, of how the ResearchElementDefinition is used."]
    pub r#usage: Option<super::super::types::String>,
    #[doc = "A copyright statement relating to the research element definition and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the research element definition."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "The date on which the resource content was approved by the publisher. Approval happens once when the content is officially approved for usage."]
    pub r#approval_date: Option<super::super::types::Date>,
    #[doc = "The date on which the resource content was last reviewed. Review happens periodically after approval but does not change the original approval date."]
    pub r#last_review_date: Option<super::super::types::Date>,
    #[doc = "The period during which the research element definition content was or is planned to be in active use."]
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    #[doc = "Descriptive topics related to the content of the ResearchElementDefinition. Topics provide a high-level categorization grouping types of ResearchElementDefinitions that can be useful for filtering and searching."]
    pub r#topic: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "An individiual or organization primarily involved in the creation and maintenance of the content."]
    pub r#author: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "An individual or organization primarily responsible for internal coherence of the content."]
    pub r#editor: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "An individual or organization primarily responsible for review of some aspect of the content."]
    pub r#reviewer: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "An individual or organization responsible for officially endorsing the content for use in some setting."]
    pub r#endorser: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "Related artifacts such as additional documentation, justification, or bibliographic references."]
    pub r#related_artifact: Vec<Box<super::super::types::RelatedArtifact>>,
    #[doc = "A reference to a Library resource containing the formal logic used by the ResearchElementDefinition."]
    pub r#library: Vec<super::super::types::Canonical>,
    #[doc = "The type of research element, a population, an exposure, or an outcome."]
    pub r#type: super::super::types::Code,
    #[doc = "The type of the outcome (e.g. Dichotomous, Continuous, or Descriptive)."]
    pub r#variable_type: Option<super::super::types::Code>,
    #[doc = "A characteristic that defines the members of the research element. Multiple characteristics are applied with \"and\" semantics."]
    pub r#characteristic: Vec<ResearchElementDefinitionCharacteristic>,
}
impl crate::AnyResource for ResearchElementDefinition {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4B;
}
impl serde::ser::Serialize for ResearchElementDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "ResearchElementDefinition")?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#url.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("url", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_url", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#url.as_ref() {
                    state.serialize_entry("url", some)?;
                }
            }
            if !self.r#identifier.is_empty() {
                state.serialize_entry("identifier", &self.r#identifier)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#version.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("version", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_version", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#version.as_ref() {
                    state.serialize_entry("version", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#name.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("name", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_name", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#name.as_ref() {
                    state.serialize_entry("name", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#title.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("title", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_title", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#title.as_ref() {
                    state.serialize_entry("title", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#short_title.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("shortTitle", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_shortTitle", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#short_title.as_ref() {
                    state.serialize_entry("shortTitle", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#subtitle.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("subtitle", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_subtitle", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#subtitle.as_ref() {
                    state.serialize_entry("subtitle", some)?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#experimental.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("experimental", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_experimental", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#experimental.as_ref() {
                    state.serialize_entry("experimental", some)?;
                }
            }
            if let Some(some) = self.r#subject.as_ref() {
                match some {
                    ResearchElementDefinitionSubject::CodeableConcept(ref value) => {
                        state.serialize_entry("subjectCodeableConcept", value)?;
                    }
                    ResearchElementDefinitionSubject::Reference(ref value) => {
                        state.serialize_entry("subjectReference", value)?;
                    }
                    ResearchElementDefinitionSubject::Invalid => {
                        return Err(serde::ser::Error::custom("subject is invalid"))
                    }
                }
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
                if let Some(some) = self.r#publisher.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("publisher", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_publisher", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#publisher.as_ref() {
                    state.serialize_entry("publisher", some)?;
                }
            }
            if !self.r#contact.is_empty() {
                state.serialize_entry("contact", &self.r#contact)?;
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
            if _ctx.output_json {
                if !self.r#comment.is_empty() {
                    let values = self
                        .r#comment
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("comment", &values)?;
                    }
                    let requires_elements = self
                        .r#comment
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#comment
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
                        state.serialize_entry("_comment", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#comment.is_empty() {
                    state.serialize_entry("comment", &self.r#comment)?;
                }
            }
            if !self.r#use_context.is_empty() {
                state.serialize_entry("useContext", &self.r#use_context)?;
            }
            if !self.r#jurisdiction.is_empty() {
                state.serialize_entry("jurisdiction", &self.r#jurisdiction)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#purpose.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("purpose", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_purpose", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#purpose.as_ref() {
                    state.serialize_entry("purpose", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#usage.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("usage", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_usage", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#usage.as_ref() {
                    state.serialize_entry("usage", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#copyright.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("copyright", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_copyright", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#copyright.as_ref() {
                    state.serialize_entry("copyright", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#approval_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("approvalDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_approvalDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#approval_date.as_ref() {
                    state.serialize_entry("approvalDate", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#last_review_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("lastReviewDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_lastReviewDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#last_review_date.as_ref() {
                    state.serialize_entry("lastReviewDate", some)?;
                }
            }
            if let Some(some) = self.r#effective_period.as_ref() {
                state.serialize_entry("effectivePeriod", some)?;
            }
            if !self.r#topic.is_empty() {
                state.serialize_entry("topic", &self.r#topic)?;
            }
            if !self.r#author.is_empty() {
                state.serialize_entry("author", &self.r#author)?;
            }
            if !self.r#editor.is_empty() {
                state.serialize_entry("editor", &self.r#editor)?;
            }
            if !self.r#reviewer.is_empty() {
                state.serialize_entry("reviewer", &self.r#reviewer)?;
            }
            if !self.r#endorser.is_empty() {
                state.serialize_entry("endorser", &self.r#endorser)?;
            }
            if !self.r#related_artifact.is_empty() {
                state.serialize_entry("relatedArtifact", &self.r#related_artifact)?;
            }
            if _ctx.output_json {
                if !self.r#library.is_empty() {
                    let values = self
                        .r#library
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("library", &values)?;
                    }
                    let requires_elements = self
                        .r#library
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#library
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
                        state.serialize_entry("_library", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#library.is_empty() {
                    state.serialize_entry("library", &self.r#library)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#type.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("type", &some)?;
                }
                if self.r#type.id.is_some() || !self.r#type.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#type.id.as_ref(),
                        extension: &self.r#type.extension,
                    };
                    state.serialize_entry("_type", &primitive_element)?;
                }
            } else {
                state.serialize_entry("type", &self.r#type)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#variable_type.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("variableType", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_variableType", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#variable_type.as_ref() {
                    state.serialize_entry("variableType", some)?;
                }
            }
            if !self.r#characteristic.is_empty() {
                state.serialize_entry("characteristic", &self.r#characteristic)?;
            }
            state.end()
        })
    }
}
