// Generated on 2022-12-28 by fhirbolt-codegen v0.1.0
#[doc = "Estimated or actual date or date-time  the condition began, in the opinion of the clinician."]
#[derive(Debug, Clone)]
pub enum ConditionOnset {
    DateTime(Box<super::super::types::DateTime>),
    Age(Box<super::super::types::Age>),
    Period(Box<super::super::types::Period>),
    Range(Box<super::super::types::Range>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for ConditionOnset {
    fn default() -> ConditionOnset {
        ConditionOnset::Invalid
    }
}
#[doc = "The date or estimated date that the condition resolved or went into remission. This is called \"abatement\" because of the many overloaded connotations associated with \"remission\" or \"resolution\" - Conditions are never really resolved, but they can abate."]
#[derive(Debug, Clone)]
pub enum ConditionAbatement {
    DateTime(Box<super::super::types::DateTime>),
    Age(Box<super::super::types::Age>),
    Period(Box<super::super::types::Period>),
    Range(Box<super::super::types::Range>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for ConditionAbatement {
    fn default() -> ConditionAbatement {
        ConditionAbatement::Invalid
    }
}
#[doc = "Clinical stage or grade of a condition. May include formal severity assessments."]
#[derive(Default, Debug, Clone)]
pub struct ConditionStage {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A simple summary of the stage such as \"Stage 3\". The determination of the stage is disease-specific."]
    pub r#summary: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Reference to a formal record of the evidence on which the staging assessment is based."]
    pub r#assessment: Vec<Box<super::super::types::Reference>>,
    #[doc = "The kind of staging, such as pathological or clinical staging."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for ConditionStage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
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
            if let Some(some) = self.r#summary.as_ref() {
                state.serialize_entry("summary", some)?;
            }
            if !self.r#assessment.is_empty() {
                state.serialize_entry("assessment", &self.r#assessment)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ConditionStage {
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
            #[serde(rename = "summary")]
            Summary,
            #[serde(rename = "assessment")]
            Assessment,
            #[serde(rename = "type")]
            Type,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ConditionStage;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ConditionStage")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ConditionStage, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#summary: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#assessment: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
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
                            Field::Summary => {
                                if r#summary.is_some() {
                                    return Err(serde::de::Error::duplicate_field("summary"));
                                }
                                r#summary = Some(map_access.next_value()?);
                            }
                            Field::Assessment => {
                                if r#assessment.is_some() {
                                    return Err(serde::de::Error::duplicate_field("assessment"));
                                }
                                r#assessment = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
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
                                        "summary",
                                        "assessment",
                                        "type",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ConditionStage {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#summary,
                        r#assessment: r#assessment.unwrap_or(vec![]),
                        r#type,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Supporting evidence / manifestations that are the basis of the Condition's verification status, such as evidence that confirmed or refuted the condition."]
#[derive(Default, Debug, Clone)]
pub struct ConditionEvidence {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A manifestation or symptom that led to the recording of this condition."]
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Links to other relevant information, including pathology reports."]
    pub r#detail: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for ConditionEvidence {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
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
            if !self.r#code.is_empty() {
                state.serialize_entry("code", &self.r#code)?;
            }
            if !self.r#detail.is_empty() {
                state.serialize_entry("detail", &self.r#detail)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ConditionEvidence {
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
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "detail")]
            Detail,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ConditionEvidence;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ConditionEvidence")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ConditionEvidence, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#detail: Option<Vec<Box<super::super::types::Reference>>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
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
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
                            }
                            Field::Detail => {
                                if r#detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field("detail"));
                                }
                                r#detail = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "code", "detail"],
                                ));
                            },
                        }
                    }
                    Ok(ConditionEvidence {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#code: r#code.unwrap_or(vec![]),
                        r#detail: r#detail.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A clinical condition, problem, diagnosis, or other event, situation, issue, or clinical concept that has risen to a level of concern."]
#[derive(Default, Debug, Clone)]
pub struct Condition {
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
    #[doc = "Business identifiers assigned to this condition by the performer or other systems which remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The clinical status of the condition."]
    pub r#clinical_status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The verification status to support the clinical status of the condition."]
    pub r#verification_status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A category assigned to the condition."]
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A subjective assessment of the severity of the condition as evaluated by the clinician."]
    pub r#severity: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identification of the condition, problem or diagnosis."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The anatomical location where this condition manifests itself."]
    pub r#body_site: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates the patient or group who the condition record is associated with."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The Encounter during which this Condition was created or to which the creation of this record is tightly associated."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Estimated or actual date or date-time  the condition began, in the opinion of the clinician."]
    pub r#onset: Option<ConditionOnset>,
    #[doc = "The date or estimated date that the condition resolved or went into remission. This is called \"abatement\" because of the many overloaded connotations associated with \"remission\" or \"resolution\" - Conditions are never really resolved, but they can abate."]
    pub r#abatement: Option<ConditionAbatement>,
    #[doc = "The recordedDate represents when this particular Condition record was created in the system, which is often a system-generated date."]
    pub r#recorded_date: Option<super::super::types::DateTime>,
    #[doc = "Individual who recorded the record and takes responsibility for its content."]
    pub r#recorder: Option<Box<super::super::types::Reference>>,
    #[doc = "Individual who is making the condition statement."]
    pub r#asserter: Option<Box<super::super::types::Reference>>,
    #[doc = "Clinical stage or grade of a condition. May include formal severity assessments."]
    pub r#stage: Vec<ConditionStage>,
    #[doc = "Supporting evidence / manifestations that are the basis of the Condition's verification status, such as evidence that confirmed or refuted the condition."]
    pub r#evidence: Vec<ConditionEvidence>,
    #[doc = "Additional information about the Condition. This is a general notes/comments entry  for description of the Condition, its diagnosis and prognosis."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
impl crate::AnyResource for Condition {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4
    }
}
impl serde::ser::Serialize for Condition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "Condition")?;
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
            if !self.r#category.is_empty() {
                state.serialize_entry("category", &self.r#category)?;
            }
            if let Some(some) = self.r#severity.as_ref() {
                state.serialize_entry("severity", some)?;
            }
            if let Some(some) = self.r#code.as_ref() {
                state.serialize_entry("code", some)?;
            }
            if !self.r#body_site.is_empty() {
                state.serialize_entry("bodySite", &self.r#body_site)?;
            }
            state.serialize_entry("subject", &self.r#subject)?;
            if let Some(some) = self.r#encounter.as_ref() {
                state.serialize_entry("encounter", some)?;
            }
            if let Some(some) = self.r#onset.as_ref() {
                match some {
                    ConditionOnset::DateTime(ref value) => {
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
                    ConditionOnset::Age(ref value) => {
                        state.serialize_entry("onsetAge", value)?;
                    }
                    ConditionOnset::Period(ref value) => {
                        state.serialize_entry("onsetPeriod", value)?;
                    }
                    ConditionOnset::Range(ref value) => {
                        state.serialize_entry("onsetRange", value)?;
                    }
                    ConditionOnset::String(ref value) => {
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
                    ConditionOnset::Invalid => {
                        return Err(serde::ser::Error::custom("onset is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#abatement.as_ref() {
                match some {
                    ConditionAbatement::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("abatementDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_abatementDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("abatementDateTime", value)?;
                        }
                    }
                    ConditionAbatement::Age(ref value) => {
                        state.serialize_entry("abatementAge", value)?;
                    }
                    ConditionAbatement::Period(ref value) => {
                        state.serialize_entry("abatementPeriod", value)?;
                    }
                    ConditionAbatement::Range(ref value) => {
                        state.serialize_entry("abatementRange", value)?;
                    }
                    ConditionAbatement::String(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("abatementString", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_abatementString", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("abatementString", value)?;
                        }
                    }
                    ConditionAbatement::Invalid => {
                        return Err(serde::ser::Error::custom("abatement is invalid"))
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
            if !self.r#stage.is_empty() {
                state.serialize_entry("stage", &self.r#stage)?;
            }
            if !self.r#evidence.is_empty() {
                state.serialize_entry("evidence", &self.r#evidence)?;
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for Condition {
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
            #[serde(rename = "clinicalStatus")]
            ClinicalStatus,
            #[serde(rename = "verificationStatus")]
            VerificationStatus,
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "severity")]
            Severity,
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "bodySite")]
            BodySite,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "encounter")]
            Encounter,
            #[serde(rename = "onsetDateTime")]
            OnsetDateTime,
            #[serde(rename = "_onsetDateTime")]
            OnsetDateTimePrimitiveElement,
            #[serde(rename = "onsetAge")]
            OnsetAge,
            #[serde(rename = "onsetPeriod")]
            OnsetPeriod,
            #[serde(rename = "onsetRange")]
            OnsetRange,
            #[serde(rename = "onsetString")]
            OnsetString,
            #[serde(rename = "_onsetString")]
            OnsetStringPrimitiveElement,
            #[serde(rename = "abatementDateTime")]
            AbatementDateTime,
            #[serde(rename = "_abatementDateTime")]
            AbatementDateTimePrimitiveElement,
            #[serde(rename = "abatementAge")]
            AbatementAge,
            #[serde(rename = "abatementPeriod")]
            AbatementPeriod,
            #[serde(rename = "abatementRange")]
            AbatementRange,
            #[serde(rename = "abatementString")]
            AbatementString,
            #[serde(rename = "_abatementString")]
            AbatementStringPrimitiveElement,
            #[serde(rename = "recordedDate")]
            RecordedDate,
            #[serde(rename = "_recordedDate")]
            RecordedDatePrimitiveElement,
            #[serde(rename = "recorder")]
            Recorder,
            #[serde(rename = "asserter")]
            Asserter,
            #[serde(rename = "stage")]
            Stage,
            #[serde(rename = "evidence")]
            Evidence,
            #[serde(rename = "note")]
            Note,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Condition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Condition")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Condition, V::Error>
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
                let mut r#clinical_status: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#verification_status: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#category: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#severity: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#body_site: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#encounter: Option<Box<super::super::types::Reference>> = None;
                let mut r#onset: Option<ConditionOnset> = None;
                let mut r#abatement: Option<ConditionAbatement> = None;
                let mut r#recorded_date: Option<super::super::types::DateTime> = None;
                let mut r#recorder: Option<Box<super::super::types::Reference>> = None;
                let mut r#asserter: Option<Box<super::super::types::Reference>> = None;
                let mut r#stage: Option<Vec<ConditionStage>> = None;
                let mut r#evidence: Option<Vec<ConditionEvidence>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "Condition" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"Condition",
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
                                            "clinicalStatus",
                                            "verificationStatus",
                                            "category",
                                            "severity",
                                            "code",
                                            "bodySite",
                                            "subject",
                                            "encounter",
                                            "onsetDateTime",
                                            "onsetAge",
                                            "onsetPeriod",
                                            "onsetRange",
                                            "onsetString",
                                            "abatementDateTime",
                                            "abatementAge",
                                            "abatementPeriod",
                                            "abatementRange",
                                            "abatementString",
                                            "recordedDate",
                                            "recorder",
                                            "asserter",
                                            "stage",
                                            "evidence",
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
                                            "clinicalStatus",
                                            "verificationStatus",
                                            "category",
                                            "severity",
                                            "code",
                                            "bodySite",
                                            "subject",
                                            "encounter",
                                            "onsetDateTime",
                                            "onsetAge",
                                            "onsetPeriod",
                                            "onsetRange",
                                            "onsetString",
                                            "abatementDateTime",
                                            "abatementAge",
                                            "abatementPeriod",
                                            "abatementRange",
                                            "abatementString",
                                            "recordedDate",
                                            "recorder",
                                            "asserter",
                                            "stage",
                                            "evidence",
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
                            Field::ClinicalStatus => {
                                if r#clinical_status.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "clinicalStatus",
                                    ));
                                }
                                r#clinical_status = Some(map_access.next_value()?);
                            }
                            Field::VerificationStatus => {
                                if r#verification_status.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "verificationStatus",
                                    ));
                                }
                                r#verification_status = Some(map_access.next_value()?);
                            }
                            Field::Category => {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                r#category = Some(map_access.next_value()?);
                            }
                            Field::Severity => {
                                if r#severity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("severity"));
                                }
                                r#severity = Some(map_access.next_value()?);
                            }
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
                            }
                            Field::BodySite => {
                                if r#body_site.is_some() {
                                    return Err(serde::de::Error::duplicate_field("bodySite"));
                                }
                                r#body_site = Some(map_access.next_value()?);
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
                            Field::OnsetDateTime => {
                                if _ctx.from_json {
                                    let r#enum = r#onset.get_or_insert(ConditionOnset::DateTime(
                                        Default::default(),
                                    ));
                                    if let ConditionOnset::DateTime(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "onsetDateTime",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("onset[x]"));
                                    }
                                } else {
                                    if r#onset.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "onsetDateTime",
                                        ));
                                    }
                                    r#onset =
                                        Some(ConditionOnset::DateTime(map_access.next_value()?));
                                }
                            }
                            Field::OnsetDateTimePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#onset.get_or_insert(ConditionOnset::DateTime(
                                        Default::default(),
                                    ));
                                    if let ConditionOnset::DateTime(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_onsetDateTime",
                                            ));
                                        }
                                        let super::super::serde_helpers::PrimitiveElementOwned {
                                            id,
                                            extension,
                                        } = map_access.next_value()?;
                                        variant.id = id;
                                        variant.extension = extension;
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("_onset[x]"));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "onsetDateTime",
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
                                            "clinicalStatus",
                                            "verificationStatus",
                                            "category",
                                            "severity",
                                            "code",
                                            "bodySite",
                                            "subject",
                                            "encounter",
                                            "onsetDateTime",
                                            "onsetAge",
                                            "onsetPeriod",
                                            "onsetRange",
                                            "onsetString",
                                            "abatementDateTime",
                                            "abatementAge",
                                            "abatementPeriod",
                                            "abatementRange",
                                            "abatementString",
                                            "recordedDate",
                                            "recorder",
                                            "asserter",
                                            "stage",
                                            "evidence",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::OnsetAge => {
                                if r#onset.is_some() {
                                    return Err(serde::de::Error::duplicate_field("onsetAge"));
                                }
                                r#onset = Some(ConditionOnset::Age(map_access.next_value()?));
                            }
                            Field::OnsetPeriod => {
                                if r#onset.is_some() {
                                    return Err(serde::de::Error::duplicate_field("onsetPeriod"));
                                }
                                r#onset = Some(ConditionOnset::Period(map_access.next_value()?));
                            }
                            Field::OnsetRange => {
                                if r#onset.is_some() {
                                    return Err(serde::de::Error::duplicate_field("onsetRange"));
                                }
                                r#onset = Some(ConditionOnset::Range(map_access.next_value()?));
                            }
                            Field::OnsetString => {
                                if _ctx.from_json {
                                    let r#enum = r#onset
                                        .get_or_insert(ConditionOnset::String(Default::default()));
                                    if let ConditionOnset::String(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "onsetString",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("onset[x]"));
                                    }
                                } else {
                                    if r#onset.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "onsetString",
                                        ));
                                    }
                                    r#onset =
                                        Some(ConditionOnset::String(map_access.next_value()?));
                                }
                            }
                            Field::OnsetStringPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#onset
                                        .get_or_insert(ConditionOnset::String(Default::default()));
                                    if let ConditionOnset::String(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_onsetString",
                                            ));
                                        }
                                        let super::super::serde_helpers::PrimitiveElementOwned {
                                            id,
                                            extension,
                                        } = map_access.next_value()?;
                                        variant.id = id;
                                        variant.extension = extension;
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("_onset[x]"));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "onsetString",
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
                                            "clinicalStatus",
                                            "verificationStatus",
                                            "category",
                                            "severity",
                                            "code",
                                            "bodySite",
                                            "subject",
                                            "encounter",
                                            "onsetDateTime",
                                            "onsetAge",
                                            "onsetPeriod",
                                            "onsetRange",
                                            "onsetString",
                                            "abatementDateTime",
                                            "abatementAge",
                                            "abatementPeriod",
                                            "abatementRange",
                                            "abatementString",
                                            "recordedDate",
                                            "recorder",
                                            "asserter",
                                            "stage",
                                            "evidence",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::AbatementDateTime => {
                                if _ctx.from_json {
                                    let r#enum = r#abatement.get_or_insert(
                                        ConditionAbatement::DateTime(Default::default()),
                                    );
                                    if let ConditionAbatement::DateTime(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "abatementDateTime",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "abatement[x]",
                                        ));
                                    }
                                } else {
                                    if r#abatement.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "abatementDateTime",
                                        ));
                                    }
                                    r#abatement = Some(ConditionAbatement::DateTime(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::AbatementDateTimePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#abatement.get_or_insert(
                                        ConditionAbatement::DateTime(Default::default()),
                                    );
                                    if let ConditionAbatement::DateTime(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_abatementDateTime",
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
                                            "_abatement[x]",
                                        ));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "abatementDateTime",
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
                                            "clinicalStatus",
                                            "verificationStatus",
                                            "category",
                                            "severity",
                                            "code",
                                            "bodySite",
                                            "subject",
                                            "encounter",
                                            "onsetDateTime",
                                            "onsetAge",
                                            "onsetPeriod",
                                            "onsetRange",
                                            "onsetString",
                                            "abatementDateTime",
                                            "abatementAge",
                                            "abatementPeriod",
                                            "abatementRange",
                                            "abatementString",
                                            "recordedDate",
                                            "recorder",
                                            "asserter",
                                            "stage",
                                            "evidence",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::AbatementAge => {
                                if r#abatement.is_some() {
                                    return Err(serde::de::Error::duplicate_field("abatementAge"));
                                }
                                r#abatement =
                                    Some(ConditionAbatement::Age(map_access.next_value()?));
                            }
                            Field::AbatementPeriod => {
                                if r#abatement.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "abatementPeriod",
                                    ));
                                }
                                r#abatement =
                                    Some(ConditionAbatement::Period(map_access.next_value()?));
                            }
                            Field::AbatementRange => {
                                if r#abatement.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "abatementRange",
                                    ));
                                }
                                r#abatement =
                                    Some(ConditionAbatement::Range(map_access.next_value()?));
                            }
                            Field::AbatementString => {
                                if _ctx.from_json {
                                    let r#enum = r#abatement.get_or_insert(
                                        ConditionAbatement::String(Default::default()),
                                    );
                                    if let ConditionAbatement::String(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "abatementString",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "abatement[x]",
                                        ));
                                    }
                                } else {
                                    if r#abatement.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "abatementString",
                                        ));
                                    }
                                    r#abatement =
                                        Some(ConditionAbatement::String(map_access.next_value()?));
                                }
                            }
                            Field::AbatementStringPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#abatement.get_or_insert(
                                        ConditionAbatement::String(Default::default()),
                                    );
                                    if let ConditionAbatement::String(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_abatementString",
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
                                            "_abatement[x]",
                                        ));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "abatementString",
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
                                            "clinicalStatus",
                                            "verificationStatus",
                                            "category",
                                            "severity",
                                            "code",
                                            "bodySite",
                                            "subject",
                                            "encounter",
                                            "onsetDateTime",
                                            "onsetAge",
                                            "onsetPeriod",
                                            "onsetRange",
                                            "onsetString",
                                            "abatementDateTime",
                                            "abatementAge",
                                            "abatementPeriod",
                                            "abatementRange",
                                            "abatementString",
                                            "recordedDate",
                                            "recorder",
                                            "asserter",
                                            "stage",
                                            "evidence",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::RecordedDate => {
                                if _ctx.from_json {
                                    let some = r#recorded_date.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "recordedDate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#recorded_date.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "recordedDate",
                                        ));
                                    }
                                    r#recorded_date = Some(map_access.next_value()?);
                                }
                            }
                            Field::RecordedDatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#recorded_date.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_recordedDate",
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
                                        "recordedDate",
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
                                            "clinicalStatus",
                                            "verificationStatus",
                                            "category",
                                            "severity",
                                            "code",
                                            "bodySite",
                                            "subject",
                                            "encounter",
                                            "onsetDateTime",
                                            "onsetAge",
                                            "onsetPeriod",
                                            "onsetRange",
                                            "onsetString",
                                            "abatementDateTime",
                                            "abatementAge",
                                            "abatementPeriod",
                                            "abatementRange",
                                            "abatementString",
                                            "recordedDate",
                                            "recorder",
                                            "asserter",
                                            "stage",
                                            "evidence",
                                            "note",
                                        ],
                                    ));
                                }
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
                            Field::Stage => {
                                if r#stage.is_some() {
                                    return Err(serde::de::Error::duplicate_field("stage"));
                                }
                                r#stage = Some(map_access.next_value()?);
                            }
                            Field::Evidence => {
                                if r#evidence.is_some() {
                                    return Err(serde::de::Error::duplicate_field("evidence"));
                                }
                                r#evidence = Some(map_access.next_value()?);
                            }
                            Field::Note => {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some(map_access.next_value()?);
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
                                        "clinicalStatus",
                                        "verificationStatus",
                                        "category",
                                        "severity",
                                        "code",
                                        "bodySite",
                                        "subject",
                                        "encounter",
                                        "onsetDateTime",
                                        "onsetAge",
                                        "onsetPeriod",
                                        "onsetRange",
                                        "onsetString",
                                        "abatementDateTime",
                                        "abatementAge",
                                        "abatementPeriod",
                                        "abatementRange",
                                        "abatementString",
                                        "recordedDate",
                                        "recorder",
                                        "asserter",
                                        "stage",
                                        "evidence",
                                        "note",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(Condition {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#clinical_status,
                        r#verification_status,
                        r#category: r#category.unwrap_or(vec![]),
                        r#severity,
                        r#code,
                        r#body_site: r#body_site.unwrap_or(vec![]),
                        r#subject: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#subject.unwrap_or(Default::default())
                        } else {
                            r#subject.ok_or(serde::de::Error::missing_field("subject"))?
                        },
                        r#encounter,
                        r#onset,
                        r#abatement,
                        r#recorded_date,
                        r#recorder,
                        r#asserter,
                        r#stage: r#stage.unwrap_or(vec![]),
                        r#evidence: r#evidence.unwrap_or(vec![]),
                        r#note: r#note.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
