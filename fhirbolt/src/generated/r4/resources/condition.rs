// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
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
#[derive(Default, Debug, Clone)]
pub struct ConditionStage {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#summary: Option<Box<super::super::types::CodeableConcept>>,
    pub r#assessment: Vec<Box<super::super::types::Reference>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for ConditionStage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ConditionEvidence {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#detail: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for ConditionEvidence {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
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
                    }
                }
                Ok(ConditionEvidence {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#code: r#code.unwrap_or(vec![]),
                    r#detail: r#detail.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct Condition {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#clinical_status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#verification_status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#severity: Option<Box<super::super::types::CodeableConcept>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#body_site: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#onset: Option<ConditionOnset>,
    pub r#abatement: Option<ConditionAbatement>,
    pub r#recorded_date: Option<super::super::types::DateTime>,
    pub r#recorder: Option<Box<super::super::types::Reference>>,
    pub r#asserter: Option<Box<super::super::types::Reference>>,
    pub r#stage: Vec<ConditionStage>,
    pub r#evidence: Vec<ConditionEvidence>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
impl serde::ser::Serialize for Condition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Condition")?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#meta.as_ref() {
            state.serialize_entry("meta", some)?;
        }
        if let Some(some) = self.r#implicit_rules.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("implicitRules", some)?;
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
                state.serialize_entry("language", some)?;
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
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("onsetDateTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_onsetDateTime", &primitive_element)?;
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
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("onsetString", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_onsetString", &primitive_element)?;
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
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("abatementDateTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_abatementDateTime", &primitive_element)?;
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
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("abatementString", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_abatementString", &primitive_element)?;
                    }
                }
                ConditionAbatement::Invalid => {
                    return Err(serde::ser::Error::custom("abatement is invalid"))
                }
            }
        }
        if let Some(some) = self.r#recorded_date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("recordedDate", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_recordedDate", &primitive_element)?;
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
                let mut r#contained: Option<Vec<Box<super::Resource>>> = None;
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
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
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
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ImplicitRulesPrimitiveElement => {
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_implicitRules"));
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
                            some.value = Some(map_access.next_value()?);
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
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
                                return Err(serde::de::Error::duplicate_field("clinicalStatus"));
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
                            let r#enum =
                                r#onset.get_or_insert(ConditionOnset::DateTime(Default::default()));
                            if let ConditionOnset::DateTime(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("onsetDateTime"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("onset[x]"));
                            }
                        }
                        Field::OnsetDateTimePrimitiveElement => {
                            let r#enum =
                                r#onset.get_or_insert(ConditionOnset::DateTime(Default::default()));
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
                            let r#enum =
                                r#onset.get_or_insert(ConditionOnset::String(Default::default()));
                            if let ConditionOnset::String(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("onsetString"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("onset[x]"));
                            }
                        }
                        Field::OnsetStringPrimitiveElement => {
                            let r#enum =
                                r#onset.get_or_insert(ConditionOnset::String(Default::default()));
                            if let ConditionOnset::String(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_onsetString"));
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
                        }
                        Field::AbatementDateTime => {
                            let r#enum = r#abatement
                                .get_or_insert(ConditionAbatement::DateTime(Default::default()));
                            if let ConditionAbatement::DateTime(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "abatementDateTime",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("abatement[x]"));
                            }
                        }
                        Field::AbatementDateTimePrimitiveElement => {
                            let r#enum = r#abatement
                                .get_or_insert(ConditionAbatement::DateTime(Default::default()));
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
                                return Err(serde::de::Error::duplicate_field("_abatement[x]"));
                            }
                        }
                        Field::AbatementAge => {
                            if r#abatement.is_some() {
                                return Err(serde::de::Error::duplicate_field("abatementAge"));
                            }
                            r#abatement = Some(ConditionAbatement::Age(map_access.next_value()?));
                        }
                        Field::AbatementPeriod => {
                            if r#abatement.is_some() {
                                return Err(serde::de::Error::duplicate_field("abatementPeriod"));
                            }
                            r#abatement =
                                Some(ConditionAbatement::Period(map_access.next_value()?));
                        }
                        Field::AbatementRange => {
                            if r#abatement.is_some() {
                                return Err(serde::de::Error::duplicate_field("abatementRange"));
                            }
                            r#abatement = Some(ConditionAbatement::Range(map_access.next_value()?));
                        }
                        Field::AbatementString => {
                            let r#enum = r#abatement
                                .get_or_insert(ConditionAbatement::String(Default::default()));
                            if let ConditionAbatement::String(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "abatementString",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("abatement[x]"));
                            }
                        }
                        Field::AbatementStringPrimitiveElement => {
                            let r#enum = r#abatement
                                .get_or_insert(ConditionAbatement::String(Default::default()));
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
                                return Err(serde::de::Error::duplicate_field("_abatement[x]"));
                            }
                        }
                        Field::RecordedDate => {
                            let some = r#recorded_date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("recordedDate"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::RecordedDatePrimitiveElement => {
                            let some = r#recorded_date.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_recordedDate"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
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
                    r#subject: r#subject.ok_or(serde::de::Error::missing_field("subject"))?,
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
