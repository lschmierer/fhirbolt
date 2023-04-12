// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "Nominal position of the recommended dose in a series (e.g. dose 2 is the next recommended dose)."]
#[derive(Debug, Clone, PartialEq)]
pub enum ImmunizationRecommendationRecommendationDoseNumber {
    PositiveInt(Box<super::super::types::PositiveInt>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for ImmunizationRecommendationRecommendationDoseNumber {
    fn default() -> ImmunizationRecommendationRecommendationDoseNumber {
        ImmunizationRecommendationRecommendationDoseNumber::Invalid
    }
}
#[doc = "The recommended number of doses to achieve immunity."]
#[derive(Debug, Clone, PartialEq)]
pub enum ImmunizationRecommendationRecommendationSeriesDoses {
    PositiveInt(Box<super::super::types::PositiveInt>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for ImmunizationRecommendationRecommendationSeriesDoses {
    fn default() -> ImmunizationRecommendationRecommendationSeriesDoses {
        ImmunizationRecommendationRecommendationSeriesDoses::Invalid
    }
}
#[doc = "Vaccine date recommendations.  For example, earliest date to administer, latest date to administer, etc."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ImmunizationRecommendationRecommendationDateCriterion {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Date classification of recommendation.  For example, earliest date to give, latest date to give, etc."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "The date whose meaning is specified by dateCriterion.code."]
    pub r#value: super::super::types::DateTime,
}
impl serde::ser::Serialize for ImmunizationRecommendationRecommendationDateCriterion {
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
            state.serialize_entry("code", &self.r#code)?;
            if _ctx.output_json {
                if let Some(some) = self.r#value.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("value", &some)?;
                }
                if self.r#value.id.is_some() || !self.r#value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#value.id.as_ref(),
                        extension: &self.r#value.extension,
                    };
                    state.serialize_entry("_value", &primitive_element)?;
                }
            } else {
                state.serialize_entry("value", &self.r#value)?;
            }
            state.end()
        })
    }
}
#[doc = "Vaccine administration recommendations."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ImmunizationRecommendationRecommendation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Vaccine(s) or vaccine group that pertain to the recommendation."]
    pub r#vaccine_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The targeted disease for the recommendation."]
    pub r#target_disease: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Vaccine(s) which should not be used to fulfill the recommendation."]
    pub r#contraindicated_vaccine_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates the patient status with respect to the path to immunity for the target disease."]
    pub r#forecast_status: Box<super::super::types::CodeableConcept>,
    #[doc = "The reason for the assigned forecast status."]
    pub r#forecast_reason: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Vaccine date recommendations.  For example, earliest date to administer, latest date to administer, etc."]
    pub r#date_criterion: Vec<ImmunizationRecommendationRecommendationDateCriterion>,
    #[doc = "Contains the description about the protocol under which the vaccine was administered."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "One possible path to achieve presumed immunity against a disease - within the context of an authority."]
    pub r#series: Option<super::super::types::String>,
    #[doc = "Nominal position of the recommended dose in a series (e.g. dose 2 is the next recommended dose)."]
    pub r#dose_number: Option<ImmunizationRecommendationRecommendationDoseNumber>,
    #[doc = "The recommended number of doses to achieve immunity."]
    pub r#series_doses: Option<ImmunizationRecommendationRecommendationSeriesDoses>,
    #[doc = "Immunization event history and/or evaluation that supports the status and recommendation."]
    pub r#supporting_immunization: Vec<Box<super::super::types::Reference>>,
    #[doc = "Patient Information that supports the status and recommendation.  This includes patient observations, adverse reactions and allergy/intolerance information."]
    pub r#supporting_patient_information: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for ImmunizationRecommendationRecommendation {
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
            if !self.r#vaccine_code.is_empty() {
                state.serialize_entry("vaccineCode", &self.r#vaccine_code)?;
            }
            if let Some(some) = self.r#target_disease.as_ref() {
                state.serialize_entry("targetDisease", some)?;
            }
            if !self.r#contraindicated_vaccine_code.is_empty() {
                state.serialize_entry(
                    "contraindicatedVaccineCode",
                    &self.r#contraindicated_vaccine_code,
                )?;
            }
            state.serialize_entry("forecastStatus", &self.r#forecast_status)?;
            if !self.r#forecast_reason.is_empty() {
                state.serialize_entry("forecastReason", &self.r#forecast_reason)?;
            }
            if !self.r#date_criterion.is_empty() {
                state.serialize_entry("dateCriterion", &self.r#date_criterion)?;
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
                if let Some(some) = self.r#series.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("series", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_series", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#series.as_ref() {
                    state.serialize_entry("series", some)?;
                }
            }
            if let Some(some) = self.r#dose_number.as_ref() {
                match some {
                    ImmunizationRecommendationRecommendationDoseNumber::PositiveInt(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("doseNumberPositiveInt", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry(
                                    "_doseNumberPositiveInt",
                                    &primitive_element,
                                )?;
                            }
                        } else {
                            state.serialize_entry("doseNumberPositiveInt", value)?;
                        }
                    }
                    ImmunizationRecommendationRecommendationDoseNumber::String(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("doseNumberString", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_doseNumberString", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("doseNumberString", value)?;
                        }
                    }
                    ImmunizationRecommendationRecommendationDoseNumber::Invalid => {
                        return Err(serde::ser::Error::custom("dose_number is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#series_doses.as_ref() {
                match some {
                    ImmunizationRecommendationRecommendationSeriesDoses::PositiveInt(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("seriesDosesPositiveInt", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry(
                                    "_seriesDosesPositiveInt",
                                    &primitive_element,
                                )?;
                            }
                        } else {
                            state.serialize_entry("seriesDosesPositiveInt", value)?;
                        }
                    }
                    ImmunizationRecommendationRecommendationSeriesDoses::String(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("seriesDosesString", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_seriesDosesString", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("seriesDosesString", value)?;
                        }
                    }
                    ImmunizationRecommendationRecommendationSeriesDoses::Invalid => {
                        return Err(serde::ser::Error::custom("series_doses is invalid"))
                    }
                }
            }
            if !self.r#supporting_immunization.is_empty() {
                state.serialize_entry("supportingImmunization", &self.r#supporting_immunization)?;
            }
            if !self.r#supporting_patient_information.is_empty() {
                state.serialize_entry(
                    "supportingPatientInformation",
                    &self.r#supporting_patient_information,
                )?;
            }
            state.end()
        })
    }
}
#[doc = "A patient's point-in-time set of recommendations (i.e. forecasting) according to a published schedule with optional supporting justification."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ImmunizationRecommendation {
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
    #[doc = "A unique identifier assigned to this particular recommendation record."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The patient the recommendation(s) are for."]
    pub r#patient: Box<super::super::types::Reference>,
    #[doc = "The date the immunization recommendation(s) were created."]
    pub r#date: super::super::types::DateTime,
    #[doc = "Indicates the authority who published the protocol (e.g. ACIP)."]
    pub r#authority: Option<Box<super::super::types::Reference>>,
    #[doc = "Vaccine administration recommendations."]
    pub r#recommendation: Vec<ImmunizationRecommendationRecommendation>,
}
impl serde::ser::Serialize for ImmunizationRecommendation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "ImmunizationRecommendation")?;
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
            state.serialize_entry("patient", &self.r#patient)?;
            if _ctx.output_json {
                if let Some(some) = self.r#date.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("date", &some)?;
                }
                if self.r#date.id.is_some() || !self.r#date.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#date.id.as_ref(),
                        extension: &self.r#date.extension,
                    };
                    state.serialize_entry("_date", &primitive_element)?;
                }
            } else {
                state.serialize_entry("date", &self.r#date)?;
            }
            if let Some(some) = self.r#authority.as_ref() {
                state.serialize_entry("authority", some)?;
            }
            if !self.r#recommendation.is_empty() {
                state.serialize_entry("recommendation", &self.r#recommendation)?;
            }
            state.end()
        })
    }
}
