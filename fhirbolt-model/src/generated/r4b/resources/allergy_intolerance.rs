// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "Estimated or actual date,  date-time, or age when allergy or intolerance was identified."]
#[derive(Debug, Clone, PartialEq)]
pub enum AllergyIntoleranceOnset {
    DateTime(Box<super::super::types::DateTime>),
    Age(Box<super::super::types::Age>),
    Period(Box<super::super::types::Period>),
    Range(Box<super::super::types::Range>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for AllergyIntoleranceOnset {
    fn default() -> AllergyIntoleranceOnset {
        AllergyIntoleranceOnset::Invalid
    }
}
#[doc = "Details about each adverse reaction event linked to exposure to the identified substance."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AllergyIntoleranceReaction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identification of the specific substance (or pharmaceutical product) considered to be responsible for the Adverse Reaction event. Note: the substance for a specific reaction may be different from the substance identified as the cause of the risk, but it must be consistent with it. For instance, it may be a more specific substance (e.g. a brand medication) or a composite product that includes the identified substance. It must be clinically safe to only process the 'code' and ignore the 'reaction.substance'.  If a receiving system is unable to confirm that AllergyIntolerance.reaction.substance falls within the semantic scope of AllergyIntolerance.code, then the receiving system should ignore AllergyIntolerance.reaction.substance."]
    pub r#substance: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Clinical symptoms and/or signs that are observed or associated with the adverse reaction event."]
    pub r#manifestation: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Text description about the reaction as a whole, including details of the manifestation if required."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Record of the date and/or time of the onset of the Reaction."]
    pub r#onset: Option<super::super::types::DateTime>,
    #[doc = "Clinical assessment of the severity of the reaction event as a whole, potentially considering multiple different manifestations."]
    pub r#severity: Option<super::super::types::Code>,
    #[doc = "Identification of the route by which the subject was exposed to the substance."]
    pub r#exposure_route: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Additional text about the adverse reaction event not captured in other fields."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
impl serde::ser::Serialize for AllergyIntoleranceReaction {
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
            if let Some(some) = self.r#substance.as_ref() {
                state.serialize_entry("substance", some)?;
            }
            if !self.r#manifestation.is_empty() {
                state.serialize_entry("manifestation", &self.r#manifestation)?;
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
                if let Some(some) = self.r#onset.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("onset", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_onset", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#onset.as_ref() {
                    state.serialize_entry("onset", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#severity.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("severity", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_severity", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#severity.as_ref() {
                    state.serialize_entry("severity", some)?;
                }
            }
            if let Some(some) = self.r#exposure_route.as_ref() {
                state.serialize_entry("exposureRoute", some)?;
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            state.end()
        })
    }
}
#[doc = "Risk of harmful or undesirable, physiological response which is unique to an individual and associated with exposure to a substance.\n\nTo record a clinical assessment of a propensity, or potential risk to an individual, of an adverse reaction upon future exposure to the specified substance, or class of substance."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AllergyIntolerance {
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
    #[doc = "Business identifiers assigned to this AllergyIntolerance by the performer or other systems which remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The clinical status of the allergy or intolerance."]
    pub r#clinical_status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Assertion about certainty associated with the propensity, or potential risk, of a reaction to the identified substance (including pharmaceutical product)."]
    pub r#verification_status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identification of the underlying physiological mechanism for the reaction risk."]
    pub r#type: Option<super::super::types::Code>,
    #[doc = "Category of the identified substance."]
    pub r#category: Vec<super::super::types::Code>,
    #[doc = "Estimate of the potential clinical harm, or seriousness, of the reaction to the identified substance."]
    pub r#criticality: Option<super::super::types::Code>,
    #[doc = "Code for an allergy or intolerance statement (either a positive or a negated/excluded statement).  This may be a code for a substance or pharmaceutical product that is considered to be responsible for the adverse reaction risk (e.g., \"Latex\"), an allergy or intolerance condition (e.g., \"Latex allergy\"), or a negated/excluded code for a specific substance or class (e.g., \"No latex allergy\") or a general or categorical negated statement (e.g.,  \"No known allergy\", \"No known drug allergies\").  Note: the substance for a specific reaction may be different from the substance identified as the cause of the risk, but it must be consistent with it. For instance, it may be a more specific substance (e.g. a brand medication) or a composite product that includes the identified substance. It must be clinically safe to only process the 'code' and ignore the 'reaction.substance'.  If a receiving system is unable to confirm that AllergyIntolerance.reaction.substance falls within the semantic scope of AllergyIntolerance.code, then the receiving system should ignore AllergyIntolerance.reaction.substance."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The patient who has the allergy or intolerance."]
    pub r#patient: Box<super::super::types::Reference>,
    #[doc = "The encounter when the allergy or intolerance was asserted."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Estimated or actual date,  date-time, or age when allergy or intolerance was identified."]
    pub r#onset: Option<AllergyIntoleranceOnset>,
    #[doc = "The recordedDate represents when this particular AllergyIntolerance record was created in the system, which is often a system-generated date."]
    pub r#recorded_date: Option<super::super::types::DateTime>,
    #[doc = "Individual who recorded the record and takes responsibility for its content."]
    pub r#recorder: Option<Box<super::super::types::Reference>>,
    #[doc = "The source of the information about the allergy that is recorded."]
    pub r#asserter: Option<Box<super::super::types::Reference>>,
    #[doc = "Represents the date and/or time of the last known occurrence of a reaction event."]
    pub r#last_occurrence: Option<super::super::types::DateTime>,
    #[doc = "Additional narrative about the propensity for the Adverse Reaction, not captured in other fields."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Details about each adverse reaction event linked to exposure to the identified substance."]
    pub r#reaction: Vec<AllergyIntoleranceReaction>,
}
impl serde::ser::Serialize for AllergyIntolerance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "AllergyIntolerance")?;
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
            if let Some(some) = self.r#clinical_status.as_ref() {
                state.serialize_entry("clinicalStatus", some)?;
            }
            if let Some(some) = self.r#verification_status.as_ref() {
                state.serialize_entry("verificationStatus", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#type.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("type", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_type", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#type.as_ref() {
                    state.serialize_entry("type", some)?;
                }
            }
            if _ctx.output_json {
                if !self.r#category.is_empty() {
                    let values = self
                        .r#category
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("category", &values)?;
                    }
                    let requires_elements = self
                        .r#category
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#category
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
                        state.serialize_entry("_category", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#category.is_empty() {
                    state.serialize_entry("category", &self.r#category)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#criticality.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("criticality", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_criticality", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#criticality.as_ref() {
                    state.serialize_entry("criticality", some)?;
                }
            }
            if let Some(some) = self.r#code.as_ref() {
                state.serialize_entry("code", some)?;
            }
            state.serialize_entry("patient", &self.r#patient)?;
            if let Some(some) = self.r#encounter.as_ref() {
                state.serialize_entry("encounter", some)?;
            }
            if let Some(some) = self.r#onset.as_ref() {
                match some {
                    AllergyIntoleranceOnset::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("onsetDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_onsetDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("onsetDateTime", value)?;
                        }
                    }
                    AllergyIntoleranceOnset::Age(ref value) => {
                        state.serialize_entry("onsetAge", value)?;
                    }
                    AllergyIntoleranceOnset::Period(ref value) => {
                        state.serialize_entry("onsetPeriod", value)?;
                    }
                    AllergyIntoleranceOnset::Range(ref value) => {
                        state.serialize_entry("onsetRange", value)?;
                    }
                    AllergyIntoleranceOnset::String(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("onsetString", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_onsetString", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("onsetString", value)?;
                        }
                    }
                    AllergyIntoleranceOnset::Invalid => {
                        return Err(serde::ser::Error::custom("onset is invalid"))
                    }
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
            if let Some(some) = self.r#recorder.as_ref() {
                state.serialize_entry("recorder", some)?;
            }
            if let Some(some) = self.r#asserter.as_ref() {
                state.serialize_entry("asserter", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#last_occurrence.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("lastOccurrence", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_lastOccurrence", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#last_occurrence.as_ref() {
                    state.serialize_entry("lastOccurrence", some)?;
                }
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            if !self.r#reaction.is_empty() {
                state.serialize_entry("reaction", &self.r#reaction)?;
            }
            state.end()
        })
    }
}
