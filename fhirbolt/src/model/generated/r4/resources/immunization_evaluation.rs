// Generated on 2022-09-28 by fhirbolt-codegen v0.1.0
#[doc = "Nominal position in a series."]
#[derive(Debug, Clone)]
pub enum ImmunizationEvaluationDoseNumber {
    PositiveInt(Box<super::super::types::PositiveInt>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl crate::model::ResourceOrElement for ImmunizationEvaluationDoseNumber {}
impl Default for ImmunizationEvaluationDoseNumber {
    fn default() -> ImmunizationEvaluationDoseNumber {
        ImmunizationEvaluationDoseNumber::Invalid
    }
}
#[doc = "The recommended number of doses to achieve immunity."]
#[derive(Debug, Clone)]
pub enum ImmunizationEvaluationSeriesDoses {
    PositiveInt(Box<super::super::types::PositiveInt>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl crate::model::ResourceOrElement for ImmunizationEvaluationSeriesDoses {}
impl Default for ImmunizationEvaluationSeriesDoses {
    fn default() -> ImmunizationEvaluationSeriesDoses {
        ImmunizationEvaluationSeriesDoses::Invalid
    }
}
#[doc = "Describes a comparison of an immunization event against published recommendations to determine if the administration is \"valid\" in relation to those  recommendations."]
#[derive(Default, Debug, Clone)]
pub struct ImmunizationEvaluation {
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
    #[doc = "A unique identifier assigned to this immunization evaluation record."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Indicates the current status of the evaluation of the vaccination administration event."]
    pub r#status: super::super::types::Code,
    #[doc = "The individual for whom the evaluation is being done."]
    pub r#patient: Box<super::super::types::Reference>,
    #[doc = "The date the evaluation of the vaccine administration event was performed."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "Indicates the authority who published the protocol (e.g. ACIP)."]
    pub r#authority: Option<Box<super::super::types::Reference>>,
    #[doc = "The vaccine preventable disease the dose is being evaluated against."]
    pub r#target_disease: Box<super::super::types::CodeableConcept>,
    #[doc = "The vaccine administration event being evaluated."]
    pub r#immunization_event: Box<super::super::types::Reference>,
    #[doc = "Indicates if the dose is valid or not valid with respect to the published recommendations."]
    pub r#dose_status: Box<super::super::types::CodeableConcept>,
    #[doc = "Provides an explanation as to why the vaccine administration event is valid or not relative to the published recommendations."]
    pub r#dose_status_reason: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Additional information about the evaluation."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "One possible path to achieve presumed immunity against a disease - within the context of an authority."]
    pub r#series: Option<super::super::types::String>,
    #[doc = "Nominal position in a series."]
    pub r#dose_number: Option<ImmunizationEvaluationDoseNumber>,
    #[doc = "The recommended number of doses to achieve immunity."]
    pub r#series_doses: Option<ImmunizationEvaluationSeriesDoses>,
}
impl crate::model::ResourceOrElement for ImmunizationEvaluation {}
impl serde::ser::Serialize for ImmunizationEvaluation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "ImmunizationEvaluation")?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#meta.as_ref() {
            state.serialize_entry("meta", some)?;
        }
        if let Some(some) = self.r#implicit_rules.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("implicitRules", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_implicitRules", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#language.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("language", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_language", &primitive_element)?;
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
        if let Some(some) = self.r#status.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("status", &some)?;
        }
        if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#status.id,
                extension: &self.r#status.extension,
            };
            state.serialize_entry("_status", &primitive_element)?;
        }
        state.serialize_entry("patient", &self.r#patient)?;
        if let Some(some) = self.r#date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("date", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_date", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#authority.as_ref() {
            state.serialize_entry("authority", some)?;
        }
        state.serialize_entry("targetDisease", &self.r#target_disease)?;
        state.serialize_entry("immunizationEvent", &self.r#immunization_event)?;
        state.serialize_entry("doseStatus", &self.r#dose_status)?;
        if !self.r#dose_status_reason.is_empty() {
            state.serialize_entry("doseStatusReason", &self.r#dose_status_reason)?;
        }
        if let Some(some) = self.r#description.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("description", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_description", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#series.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("series", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_series", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#dose_number.as_ref() {
            match some {
                ImmunizationEvaluationDoseNumber::PositiveInt(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("doseNumberPositiveInt", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_doseNumberPositiveInt", &primitive_element)?;
                    }
                }
                ImmunizationEvaluationDoseNumber::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("doseNumberString", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_doseNumberString", &primitive_element)?;
                    }
                }
                ImmunizationEvaluationDoseNumber::Invalid => {
                    return Err(serde::ser::Error::custom("dose_number is invalid"))
                }
            }
        }
        if let Some(some) = self.r#series_doses.as_ref() {
            match some {
                ImmunizationEvaluationSeriesDoses::PositiveInt(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("seriesDosesPositiveInt", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_seriesDosesPositiveInt", &primitive_element)?;
                    }
                }
                ImmunizationEvaluationSeriesDoses::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("seriesDosesString", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_seriesDosesString", &primitive_element)?;
                    }
                }
                ImmunizationEvaluationSeriesDoses::Invalid => {
                    return Err(serde::ser::Error::custom("series_doses is invalid"))
                }
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ImmunizationEvaluation {
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
            #[serde(rename = "patient")]
            Patient,
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "_date")]
            DatePrimitiveElement,
            #[serde(rename = "authority")]
            Authority,
            #[serde(rename = "targetDisease")]
            TargetDisease,
            #[serde(rename = "immunizationEvent")]
            ImmunizationEvent,
            #[serde(rename = "doseStatus")]
            DoseStatus,
            #[serde(rename = "doseStatusReason")]
            DoseStatusReason,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "series")]
            Series,
            #[serde(rename = "_series")]
            SeriesPrimitiveElement,
            #[serde(rename = "doseNumberPositiveInt")]
            DoseNumberPositiveInt,
            #[serde(rename = "_doseNumberPositiveInt")]
            DoseNumberPositiveIntPrimitiveElement,
            #[serde(rename = "doseNumberString")]
            DoseNumberString,
            #[serde(rename = "_doseNumberString")]
            DoseNumberStringPrimitiveElement,
            #[serde(rename = "seriesDosesPositiveInt")]
            SeriesDosesPositiveInt,
            #[serde(rename = "_seriesDosesPositiveInt")]
            SeriesDosesPositiveIntPrimitiveElement,
            #[serde(rename = "seriesDosesString")]
            SeriesDosesString,
            #[serde(rename = "_seriesDosesString")]
            SeriesDosesStringPrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImmunizationEvaluation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImmunizationEvaluation")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ImmunizationEvaluation, V::Error>
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
                let mut r#patient: Option<Box<super::super::types::Reference>> = None;
                let mut r#date: Option<super::super::types::DateTime> = None;
                let mut r#authority: Option<Box<super::super::types::Reference>> = None;
                let mut r#target_disease: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#immunization_event: Option<Box<super::super::types::Reference>> = None;
                let mut r#dose_status: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#dose_status_reason: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#series: Option<super::super::types::String> = None;
                let mut r#dose_number: Option<ImmunizationEvaluationDoseNumber> = None;
                let mut r#series_doses: Option<ImmunizationEvaluationSeriesDoses> = None;
                crate::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "ImmunizationEvaluation" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"ImmunizationEvaluation",
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
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ImplicitRulesPrimitiveElement => {
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
                            }
                            Field::Language => {
                                let some = r#language.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::LanguagePrimitiveElement => {
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
                            Field::Status => {
                                let some = r#status.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::StatusPrimitiveElement => {
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
                            }
                            Field::Patient => {
                                if r#patient.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patient"));
                                }
                                r#patient = Some(map_access.next_value()?);
                            }
                            Field::Date => {
                                let some = r#date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("date"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::DatePrimitiveElement => {
                                let some = r#date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_date"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Authority => {
                                if r#authority.is_some() {
                                    return Err(serde::de::Error::duplicate_field("authority"));
                                }
                                r#authority = Some(map_access.next_value()?);
                            }
                            Field::TargetDisease => {
                                if r#target_disease.is_some() {
                                    return Err(serde::de::Error::duplicate_field("targetDisease"));
                                }
                                r#target_disease = Some(map_access.next_value()?);
                            }
                            Field::ImmunizationEvent => {
                                if r#immunization_event.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "immunizationEvent",
                                    ));
                                }
                                r#immunization_event = Some(map_access.next_value()?);
                            }
                            Field::DoseStatus => {
                                if r#dose_status.is_some() {
                                    return Err(serde::de::Error::duplicate_field("doseStatus"));
                                }
                                r#dose_status = Some(map_access.next_value()?);
                            }
                            Field::DoseStatusReason => {
                                if r#dose_status_reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "doseStatusReason",
                                    ));
                                }
                                r#dose_status_reason = Some(map_access.next_value()?);
                            }
                            Field::Description => {
                                let some = r#description.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::DescriptionPrimitiveElement => {
                                let some = r#description.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_description"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Series => {
                                let some = r#series.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("series"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::SeriesPrimitiveElement => {
                                let some = r#series.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_series"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::DoseNumberPositiveInt => {
                                let r#enum = r#dose_number.get_or_insert(
                                    ImmunizationEvaluationDoseNumber::PositiveInt(
                                        Default::default(),
                                    ),
                                );
                                if let ImmunizationEvaluationDoseNumber::PositiveInt(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "doseNumberPositiveInt",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("doseNumber[x]"));
                                }
                            }
                            Field::DoseNumberPositiveIntPrimitiveElement => {
                                let r#enum = r#dose_number.get_or_insert(
                                    ImmunizationEvaluationDoseNumber::PositiveInt(
                                        Default::default(),
                                    ),
                                );
                                if let ImmunizationEvaluationDoseNumber::PositiveInt(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_doseNumberPositiveInt",
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
                                        "_doseNumber[x]",
                                    ));
                                }
                            }
                            Field::DoseNumberString => {
                                let r#enum = r#dose_number.get_or_insert(
                                    ImmunizationEvaluationDoseNumber::String(Default::default()),
                                );
                                if let ImmunizationEvaluationDoseNumber::String(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "doseNumberString",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("doseNumber[x]"));
                                }
                            }
                            Field::DoseNumberStringPrimitiveElement => {
                                let r#enum = r#dose_number.get_or_insert(
                                    ImmunizationEvaluationDoseNumber::String(Default::default()),
                                );
                                if let ImmunizationEvaluationDoseNumber::String(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_doseNumberString",
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
                                        "_doseNumber[x]",
                                    ));
                                }
                            }
                            Field::SeriesDosesPositiveInt => {
                                let r#enum = r#series_doses.get_or_insert(
                                    ImmunizationEvaluationSeriesDoses::PositiveInt(
                                        Default::default(),
                                    ),
                                );
                                if let ImmunizationEvaluationSeriesDoses::PositiveInt(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "seriesDosesPositiveInt",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "seriesDoses[x]",
                                    ));
                                }
                            }
                            Field::SeriesDosesPositiveIntPrimitiveElement => {
                                let r#enum = r#series_doses.get_or_insert(
                                    ImmunizationEvaluationSeriesDoses::PositiveInt(
                                        Default::default(),
                                    ),
                                );
                                if let ImmunizationEvaluationSeriesDoses::PositiveInt(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_seriesDosesPositiveInt",
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
                                        "_seriesDoses[x]",
                                    ));
                                }
                            }
                            Field::SeriesDosesString => {
                                let r#enum = r#series_doses.get_or_insert(
                                    ImmunizationEvaluationSeriesDoses::String(Default::default()),
                                );
                                if let ImmunizationEvaluationSeriesDoses::String(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "seriesDosesString",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "seriesDoses[x]",
                                    ));
                                }
                            }
                            Field::SeriesDosesStringPrimitiveElement => {
                                let r#enum = r#series_doses.get_or_insert(
                                    ImmunizationEvaluationSeriesDoses::String(Default::default()),
                                );
                                if let ImmunizationEvaluationSeriesDoses::String(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_seriesDosesString",
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
                                        "_seriesDoses[x]",
                                    ));
                                }
                            }
                            Field::Unknown(key) => {
                                if config.mode == crate::DeserializationMode::Strict {
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
                                            "patient",
                                            "date",
                                            "authority",
                                            "targetDisease",
                                            "immunizationEvent",
                                            "doseStatus",
                                            "doseStatusReason",
                                            "description",
                                            "series",
                                            "doseNumberPositiveInt",
                                            "doseNumberString",
                                            "seriesDosesPositiveInt",
                                            "seriesDosesString",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(ImmunizationEvaluation {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#status: if config.mode == crate::DeserializationMode::Lax {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#patient: if config.mode == crate::DeserializationMode::Lax {
                            r#patient.unwrap_or(Default::default())
                        } else {
                            r#patient.ok_or(serde::de::Error::missing_field("patient"))?
                        },
                        r#date,
                        r#authority,
                        r#target_disease: if config.mode == crate::DeserializationMode::Lax {
                            r#target_disease.unwrap_or(Default::default())
                        } else {
                            r#target_disease
                                .ok_or(serde::de::Error::missing_field("targetDisease"))?
                        },
                        r#immunization_event: if config.mode == crate::DeserializationMode::Lax {
                            r#immunization_event.unwrap_or(Default::default())
                        } else {
                            r#immunization_event
                                .ok_or(serde::de::Error::missing_field("immunizationEvent"))?
                        },
                        r#dose_status: if config.mode == crate::DeserializationMode::Lax {
                            r#dose_status.unwrap_or(Default::default())
                        } else {
                            r#dose_status.ok_or(serde::de::Error::missing_field("doseStatus"))?
                        },
                        r#dose_status_reason: r#dose_status_reason.unwrap_or(vec![]),
                        r#description,
                        r#series,
                        r#dose_number,
                        r#series_doses,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
