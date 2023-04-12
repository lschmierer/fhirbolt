// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "The date (and possibly time) the risk assessment was performed."]
#[derive(Debug, Clone, PartialEq)]
pub enum RiskAssessmentOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for RiskAssessmentOccurrence {
    fn default() -> RiskAssessmentOccurrence {
        RiskAssessmentOccurrence::Invalid
    }
}
#[doc = "Indicates how likely the outcome is (in the specified timeframe)."]
#[derive(Debug, Clone, PartialEq)]
pub enum RiskAssessmentPredictionProbability {
    Decimal(Box<super::super::types::Decimal>),
    Range(Box<super::super::types::Range>),
    Invalid,
}
impl Default for RiskAssessmentPredictionProbability {
    fn default() -> RiskAssessmentPredictionProbability {
        RiskAssessmentPredictionProbability::Invalid
    }
}
#[doc = "Indicates the period of time or age range of the subject to which the specified probability applies."]
#[derive(Debug, Clone, PartialEq)]
pub enum RiskAssessmentPredictionWhen {
    Period(Box<super::super::types::Period>),
    Range(Box<super::super::types::Range>),
    Invalid,
}
impl Default for RiskAssessmentPredictionWhen {
    fn default() -> RiskAssessmentPredictionWhen {
        RiskAssessmentPredictionWhen::Invalid
    }
}
#[doc = "Describes the expected outcome for the subject."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RiskAssessmentPrediction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "One of the potential outcomes for the patient (e.g. remission, death,  a particular condition)."]
    pub r#outcome: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates how likely the outcome is (in the specified timeframe)."]
    pub r#probability: Option<RiskAssessmentPredictionProbability>,
    #[doc = "Indicates how likely the outcome is (in the specified timeframe), expressed as a qualitative value (e.g. low, medium, or high)."]
    pub r#qualitative_risk: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates the risk for this particular subject (with their specific characteristics) divided by the risk of the population in general.  (Numbers greater than 1 = higher risk than the population, numbers less than 1 = lower risk.)."]
    pub r#relative_risk: Option<super::super::types::Decimal>,
    #[doc = "Indicates the period of time or age range of the subject to which the specified probability applies."]
    pub r#when: Option<RiskAssessmentPredictionWhen>,
    #[doc = "Additional information explaining the basis for the prediction."]
    pub r#rationale: Option<super::super::types::String>,
}
impl serde::ser::Serialize for RiskAssessmentPrediction {
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
            if let Some(some) = self.r#outcome.as_ref() {
                state.serialize_entry("outcome", some)?;
            }
            if let Some(some) = self.r#probability.as_ref() {
                match some {
                    RiskAssessmentPredictionProbability::Decimal(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = some.parse::<serde_json::Number>().map_err(|_| {
                                    serde::ser::Error::custom("error serializing decimal")
                                })?;
                                state.serialize_entry("probabilityDecimal", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_probabilityDecimal", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("probabilityDecimal", value)?;
                        }
                    }
                    RiskAssessmentPredictionProbability::Range(ref value) => {
                        state.serialize_entry("probabilityRange", value)?;
                    }
                    RiskAssessmentPredictionProbability::Invalid => {
                        return Err(serde::ser::Error::custom("probability is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#qualitative_risk.as_ref() {
                state.serialize_entry("qualitativeRisk", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#relative_risk.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("relativeRisk", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_relativeRisk", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#relative_risk.as_ref() {
                    state.serialize_entry("relativeRisk", some)?;
                }
            }
            if let Some(some) = self.r#when.as_ref() {
                match some {
                    RiskAssessmentPredictionWhen::Period(ref value) => {
                        state.serialize_entry("whenPeriod", value)?;
                    }
                    RiskAssessmentPredictionWhen::Range(ref value) => {
                        state.serialize_entry("whenRange", value)?;
                    }
                    RiskAssessmentPredictionWhen::Invalid => {
                        return Err(serde::ser::Error::custom("when is invalid"))
                    }
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#rationale.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("rationale", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_rationale", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#rationale.as_ref() {
                    state.serialize_entry("rationale", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "An assessment of the likely outcome(s) for a patient or other subject as well as the likelihood of each outcome."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RiskAssessment {
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
    #[doc = "Business identifier assigned to the risk assessment."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "A reference to the request that is fulfilled by this risk assessment."]
    pub r#based_on: Option<Box<super::super::types::Reference>>,
    #[doc = "A reference to a resource that this risk assessment is part of, such as a Procedure."]
    pub r#parent: Option<Box<super::super::types::Reference>>,
    #[doc = "The status of the RiskAssessment, using the same statuses as an Observation."]
    pub r#status: super::super::types::Code,
    #[doc = "The algorithm, process or mechanism used to evaluate the risk."]
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The type of the risk assessment performed."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The patient or group the risk assessment applies to."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The encounter where the assessment was performed."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The date (and possibly time) the risk assessment was performed."]
    pub r#occurrence: Option<RiskAssessmentOccurrence>,
    #[doc = "For assessments or prognosis specific to a particular condition, indicates the condition being assessed."]
    pub r#condition: Option<Box<super::super::types::Reference>>,
    #[doc = "The provider or software application that performed the assessment."]
    pub r#performer: Option<Box<super::super::types::Reference>>,
    #[doc = "The reason the risk assessment was performed."]
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Resources supporting the reason the risk assessment was performed."]
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "Indicates the source data considered as part of the assessment (for example, FamilyHistory, Observations, Procedures, Conditions, etc.)."]
    pub r#basis: Vec<Box<super::super::types::Reference>>,
    #[doc = "Describes the expected outcome for the subject."]
    pub r#prediction: Vec<RiskAssessmentPrediction>,
    #[doc = "A description of the steps that might be taken to reduce the identified risk(s)."]
    pub r#mitigation: Option<super::super::types::String>,
    #[doc = "Additional comments about the risk assessment."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
impl serde::ser::Serialize for RiskAssessment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "RiskAssessment")?;
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
            if let Some(some) = self.r#based_on.as_ref() {
                state.serialize_entry("basedOn", some)?;
            }
            if let Some(some) = self.r#parent.as_ref() {
                state.serialize_entry("parent", some)?;
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
            if let Some(some) = self.r#method.as_ref() {
                state.serialize_entry("method", some)?;
            }
            if let Some(some) = self.r#code.as_ref() {
                state.serialize_entry("code", some)?;
            }
            state.serialize_entry("subject", &self.r#subject)?;
            if let Some(some) = self.r#encounter.as_ref() {
                state.serialize_entry("encounter", some)?;
            }
            if let Some(some) = self.r#occurrence.as_ref() {
                match some {
                    RiskAssessmentOccurrence::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("occurrenceDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_occurrenceDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("occurrenceDateTime", value)?;
                        }
                    }
                    RiskAssessmentOccurrence::Period(ref value) => {
                        state.serialize_entry("occurrencePeriod", value)?;
                    }
                    RiskAssessmentOccurrence::Invalid => {
                        return Err(serde::ser::Error::custom("occurrence is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#condition.as_ref() {
                state.serialize_entry("condition", some)?;
            }
            if let Some(some) = self.r#performer.as_ref() {
                state.serialize_entry("performer", some)?;
            }
            if !self.r#reason_code.is_empty() {
                state.serialize_entry("reasonCode", &self.r#reason_code)?;
            }
            if !self.r#reason_reference.is_empty() {
                state.serialize_entry("reasonReference", &self.r#reason_reference)?;
            }
            if !self.r#basis.is_empty() {
                state.serialize_entry("basis", &self.r#basis)?;
            }
            if !self.r#prediction.is_empty() {
                state.serialize_entry("prediction", &self.r#prediction)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#mitigation.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("mitigation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_mitigation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#mitigation.as_ref() {
                    state.serialize_entry("mitigation", some)?;
                }
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            state.end()
        })
    }
}
