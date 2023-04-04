// Generated on 2023-04-04 by fhirbolt-codegen v0.1.0
#[doc = "The date (and possibly time) the risk assessment was performed."]
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Default, Debug, Clone)]
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
impl<'de> serde::de::Deserialize<'de> for RiskAssessmentPrediction {
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
            #[serde(rename = "outcome")]
            Outcome,
            #[serde(rename = "probabilityDecimal")]
            ProbabilityDecimal,
            #[serde(rename = "_probabilityDecimal")]
            ProbabilityDecimalPrimitiveElement,
            #[serde(rename = "probabilityRange")]
            ProbabilityRange,
            #[serde(rename = "qualitativeRisk")]
            QualitativeRisk,
            #[serde(rename = "relativeRisk")]
            RelativeRisk,
            #[serde(rename = "_relativeRisk")]
            RelativeRiskPrimitiveElement,
            #[serde(rename = "whenPeriod")]
            WhenPeriod,
            #[serde(rename = "whenRange")]
            WhenRange,
            #[serde(rename = "rationale")]
            Rationale,
            #[serde(rename = "_rationale")]
            RationalePrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RiskAssessmentPrediction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("RiskAssessmentPrediction")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<RiskAssessmentPrediction, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#outcome: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#probability: Option<RiskAssessmentPredictionProbability> = None;
                let mut r#qualitative_risk: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#relative_risk: Option<super::super::types::Decimal> = None;
                let mut r#when: Option<RiskAssessmentPredictionWhen> = None;
                let mut r#rationale: Option<super::super::types::String> = None;
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
                            Field::Outcome => {
                                if r#outcome.is_some() {
                                    return Err(serde::de::Error::duplicate_field("outcome"));
                                }
                                r#outcome = Some(map_access.next_value()?);
                            }
                            Field::ProbabilityDecimal => {
                                if _ctx.from_json {
                                    let r#enum = r#probability.get_or_insert(
                                        RiskAssessmentPredictionProbability::Decimal(
                                            Default::default(),
                                        ),
                                    );
                                    if let RiskAssessmentPredictionProbability::Decimal(variant) =
                                        r#enum
                                    {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "probabilityDecimal",
                                            ));
                                        }
                                        let value: serde_json::Number = map_access.next_value()?;
                                        variant.value = Some(format!("{}", value));
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "probability[x]",
                                        ));
                                    }
                                } else {
                                    if r#probability.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "probabilityDecimal",
                                        ));
                                    }
                                    r#probability =
                                        Some(RiskAssessmentPredictionProbability::Decimal(
                                            map_access.next_value()?,
                                        ));
                                }
                            }
                            Field::ProbabilityDecimalPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#probability.get_or_insert(
                                        RiskAssessmentPredictionProbability::Decimal(
                                            Default::default(),
                                        ),
                                    );
                                    if let RiskAssessmentPredictionProbability::Decimal(variant) =
                                        r#enum
                                    {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_probabilityDecimal",
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
                                            "_probability[x]",
                                        ));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "probabilityDecimal",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "outcome",
                                            "probabilityDecimal",
                                            "probabilityRange",
                                            "qualitativeRisk",
                                            "relativeRisk",
                                            "whenPeriod",
                                            "whenRange",
                                            "rationale",
                                        ],
                                    ));
                                }
                            }
                            Field::ProbabilityRange => {
                                if r#probability.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "probabilityRange",
                                    ));
                                }
                                r#probability = Some(RiskAssessmentPredictionProbability::Range(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::QualitativeRisk => {
                                if r#qualitative_risk.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "qualitativeRisk",
                                    ));
                                }
                                r#qualitative_risk = Some(map_access.next_value()?);
                            }
                            Field::RelativeRisk => {
                                if _ctx.from_json {
                                    let some = r#relative_risk.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "relativeRisk",
                                        ));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    some.value = Some(format!("{}", value));
                                } else {
                                    if r#relative_risk.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "relativeRisk",
                                        ));
                                    }
                                    r#relative_risk = Some(map_access.next_value()?);
                                }
                            }
                            Field::RelativeRiskPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#relative_risk.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_relativeRisk",
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
                                        "relativeRisk",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "outcome",
                                            "probabilityDecimal",
                                            "probabilityRange",
                                            "qualitativeRisk",
                                            "relativeRisk",
                                            "whenPeriod",
                                            "whenRange",
                                            "rationale",
                                        ],
                                    ));
                                }
                            }
                            Field::WhenPeriod => {
                                if r#when.is_some() {
                                    return Err(serde::de::Error::duplicate_field("whenPeriod"));
                                }
                                r#when = Some(RiskAssessmentPredictionWhen::Period(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::WhenRange => {
                                if r#when.is_some() {
                                    return Err(serde::de::Error::duplicate_field("whenRange"));
                                }
                                r#when = Some(RiskAssessmentPredictionWhen::Range(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::Rationale => {
                                if _ctx.from_json {
                                    let some = r#rationale.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("rationale"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#rationale.is_some() {
                                        return Err(serde::de::Error::duplicate_field("rationale"));
                                    }
                                    r#rationale = Some(map_access.next_value()?);
                                }
                            }
                            Field::RationalePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#rationale.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_rationale",
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
                                        "rationale",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "outcome",
                                            "probabilityDecimal",
                                            "probabilityRange",
                                            "qualitativeRisk",
                                            "relativeRisk",
                                            "whenPeriod",
                                            "whenRange",
                                            "rationale",
                                        ],
                                    ));
                                }
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
                                        "outcome",
                                        "probabilityDecimal",
                                        "probabilityRange",
                                        "qualitativeRisk",
                                        "relativeRisk",
                                        "whenPeriod",
                                        "whenRange",
                                        "rationale",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(RiskAssessmentPrediction {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#outcome,
                        r#probability,
                        r#qualitative_risk,
                        r#relative_risk,
                        r#when,
                        r#rationale,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "An assessment of the likely outcome(s) for a patient or other subject as well as the likelihood of each outcome."]
#[derive(Default, Debug, Clone)]
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
impl crate::AnyResource for RiskAssessment {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4;
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
impl<'de> serde::de::Deserialize<'de> for RiskAssessment {
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
            #[serde(rename = "basedOn")]
            BasedOn,
            #[serde(rename = "parent")]
            Parent,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "method")]
            Method,
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "encounter")]
            Encounter,
            #[serde(rename = "occurrenceDateTime")]
            OccurrenceDateTime,
            #[serde(rename = "_occurrenceDateTime")]
            OccurrenceDateTimePrimitiveElement,
            #[serde(rename = "occurrencePeriod")]
            OccurrencePeriod,
            #[serde(rename = "condition")]
            Condition,
            #[serde(rename = "performer")]
            Performer,
            #[serde(rename = "reasonCode")]
            ReasonCode,
            #[serde(rename = "reasonReference")]
            ReasonReference,
            #[serde(rename = "basis")]
            Basis,
            #[serde(rename = "prediction")]
            Prediction,
            #[serde(rename = "mitigation")]
            Mitigation,
            #[serde(rename = "_mitigation")]
            MitigationPrimitiveElement,
            #[serde(rename = "note")]
            Note,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RiskAssessment;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("RiskAssessment")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<RiskAssessment, V::Error>
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
                let mut r#based_on: Option<Box<super::super::types::Reference>> = None;
                let mut r#parent: Option<Box<super::super::types::Reference>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#method: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#encounter: Option<Box<super::super::types::Reference>> = None;
                let mut r#occurrence: Option<RiskAssessmentOccurrence> = None;
                let mut r#condition: Option<Box<super::super::types::Reference>> = None;
                let mut r#performer: Option<Box<super::super::types::Reference>> = None;
                let mut r#reason_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#reason_reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#basis: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#prediction: Option<Vec<RiskAssessmentPrediction>> = None;
                let mut r#mitigation: Option<super::super::types::String> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "RiskAssessment" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"RiskAssessment",
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
                                            "basedOn",
                                            "parent",
                                            "status",
                                            "method",
                                            "code",
                                            "subject",
                                            "encounter",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "condition",
                                            "performer",
                                            "reasonCode",
                                            "reasonReference",
                                            "basis",
                                            "prediction",
                                            "mitigation",
                                            "note",
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
                                            "basedOn",
                                            "parent",
                                            "status",
                                            "method",
                                            "code",
                                            "subject",
                                            "encounter",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "condition",
                                            "performer",
                                            "reasonCode",
                                            "reasonReference",
                                            "basis",
                                            "prediction",
                                            "mitigation",
                                            "note",
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
                            Field::BasedOn => {
                                if r#based_on.is_some() {
                                    return Err(serde::de::Error::duplicate_field("basedOn"));
                                }
                                r#based_on = Some(map_access.next_value()?);
                            }
                            Field::Parent => {
                                if r#parent.is_some() {
                                    return Err(serde::de::Error::duplicate_field("parent"));
                                }
                                r#parent = Some(map_access.next_value()?);
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
                                            "basedOn",
                                            "parent",
                                            "status",
                                            "method",
                                            "code",
                                            "subject",
                                            "encounter",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "condition",
                                            "performer",
                                            "reasonCode",
                                            "reasonReference",
                                            "basis",
                                            "prediction",
                                            "mitigation",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::Method => {
                                if r#method.is_some() {
                                    return Err(serde::de::Error::duplicate_field("method"));
                                }
                                r#method = Some(map_access.next_value()?);
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
                            Field::OccurrenceDateTime => {
                                if _ctx.from_json {
                                    let r#enum = r#occurrence.get_or_insert(
                                        RiskAssessmentOccurrence::DateTime(Default::default()),
                                    );
                                    if let RiskAssessmentOccurrence::DateTime(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "occurrenceDateTime",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "occurrence[x]",
                                        ));
                                    }
                                } else {
                                    if r#occurrence.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "occurrenceDateTime",
                                        ));
                                    }
                                    r#occurrence = Some(RiskAssessmentOccurrence::DateTime(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::OccurrenceDateTimePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#occurrence.get_or_insert(
                                        RiskAssessmentOccurrence::DateTime(Default::default()),
                                    );
                                    if let RiskAssessmentOccurrence::DateTime(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_occurrenceDateTime",
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
                                            "_occurrence[x]",
                                        ));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "occurrenceDateTime",
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
                                            "basedOn",
                                            "parent",
                                            "status",
                                            "method",
                                            "code",
                                            "subject",
                                            "encounter",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "condition",
                                            "performer",
                                            "reasonCode",
                                            "reasonReference",
                                            "basis",
                                            "prediction",
                                            "mitigation",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::OccurrencePeriod => {
                                if r#occurrence.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "occurrencePeriod",
                                    ));
                                }
                                r#occurrence = Some(RiskAssessmentOccurrence::Period(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::Condition => {
                                if r#condition.is_some() {
                                    return Err(serde::de::Error::duplicate_field("condition"));
                                }
                                r#condition = Some(map_access.next_value()?);
                            }
                            Field::Performer => {
                                if r#performer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("performer"));
                                }
                                r#performer = Some(map_access.next_value()?);
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
                            Field::Basis => {
                                if _ctx.from_json {
                                    if r#basis.is_some() {
                                        return Err(serde::de::Error::duplicate_field("basis"));
                                    }
                                    r#basis = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#basis.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Prediction => {
                                if _ctx.from_json {
                                    if r#prediction.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "prediction",
                                        ));
                                    }
                                    r#prediction = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#prediction.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Mitigation => {
                                if _ctx.from_json {
                                    let some = r#mitigation.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "mitigation",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#mitigation.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "mitigation",
                                        ));
                                    }
                                    r#mitigation = Some(map_access.next_value()?);
                                }
                            }
                            Field::MitigationPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#mitigation.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_mitigation",
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
                                        "mitigation",
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
                                            "basedOn",
                                            "parent",
                                            "status",
                                            "method",
                                            "code",
                                            "subject",
                                            "encounter",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "condition",
                                            "performer",
                                            "reasonCode",
                                            "reasonReference",
                                            "basis",
                                            "prediction",
                                            "mitigation",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::Note => {
                                if _ctx.from_json {
                                    if r#note.is_some() {
                                        return Err(serde::de::Error::duplicate_field("note"));
                                    }
                                    r#note = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#note.get_or_insert(Default::default());
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
                                        "basedOn",
                                        "parent",
                                        "status",
                                        "method",
                                        "code",
                                        "subject",
                                        "encounter",
                                        "occurrenceDateTime",
                                        "occurrencePeriod",
                                        "condition",
                                        "performer",
                                        "reasonCode",
                                        "reasonReference",
                                        "basis",
                                        "prediction",
                                        "mitigation",
                                        "note",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(RiskAssessment {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#based_on,
                        r#parent,
                        r#status: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#method,
                        r#code,
                        r#subject: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#subject.unwrap_or(Default::default())
                        } else {
                            r#subject.ok_or(serde::de::Error::missing_field("subject"))?
                        },
                        r#encounter,
                        r#occurrence,
                        r#condition,
                        r#performer,
                        r#reason_code: r#reason_code.unwrap_or(vec![]),
                        r#reason_reference: r#reason_reference.unwrap_or(vec![]),
                        r#basis: r#basis.unwrap_or(vec![]),
                        r#prediction: r#prediction.unwrap_or(vec![]),
                        r#mitigation,
                        r#note: r#note.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
