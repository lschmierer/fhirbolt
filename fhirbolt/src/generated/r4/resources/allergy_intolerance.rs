// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
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
#[derive(Default, Debug, Clone)]
pub struct AllergyIntoleranceReaction {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#substance: Option<Box<super::super::types::CodeableConcept>>,
    pub r#manifestation: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#description: Option<super::super::types::String>,
    pub r#onset: Option<super::super::types::DateTime>,
    pub r#severity: Option<super::super::types::Code>,
    pub r#exposure_route: Option<Box<super::super::types::CodeableConcept>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
impl serde::ser::Serialize for AllergyIntoleranceReaction {
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
        if let Some(some) = self.r#substance.as_ref() {
            state.serialize_entry("substance", some)?;
        }
        if !self.r#manifestation.is_empty() {
            state.serialize_entry("manifestation", &self.r#manifestation)?;
        }
        if let Some(some) = self.r#description.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("description", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_description", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#onset.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("onset", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_onset", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#severity.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("severity", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_severity", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#exposure_route.as_ref() {
            state.serialize_entry("exposureRoute", some)?;
        }
        if !self.r#note.is_empty() {
            state.serialize_entry("note", &self.r#note)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for AllergyIntoleranceReaction {
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
            #[serde(rename = "substance")]
            Substance,
            #[serde(rename = "manifestation")]
            Manifestation,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "onset")]
            Onset,
            #[serde(rename = "_onset")]
            OnsetPrimitiveElement,
            #[serde(rename = "severity")]
            Severity,
            #[serde(rename = "_severity")]
            SeverityPrimitiveElement,
            #[serde(rename = "exposureRoute")]
            ExposureRoute,
            #[serde(rename = "note")]
            Note,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AllergyIntoleranceReaction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AllergyIntoleranceReaction")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<AllergyIntoleranceReaction, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#substance: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#manifestation: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#onset: Option<super::super::types::DateTime> = None;
                let mut r#severity: Option<super::super::types::Code> = None;
                let mut r#exposure_route: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
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
                        Field::Substance => {
                            if r#substance.is_some() {
                                return Err(serde::de::Error::duplicate_field("substance"));
                            }
                            r#substance = Some(map_access.next_value()?);
                        }
                        Field::Manifestation => {
                            if r#manifestation.is_some() {
                                return Err(serde::de::Error::duplicate_field("manifestation"));
                            }
                            r#manifestation = Some(map_access.next_value()?);
                        }
                        Field::Description => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            some.value = Some(map_access.next_value()?);
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
                        Field::Onset => {
                            let some = r#onset.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("onset"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::OnsetPrimitiveElement => {
                            let some = r#onset.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_onset"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Severity => {
                            let some = r#severity.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("severity"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::SeverityPrimitiveElement => {
                            let some = r#severity.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_severity"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::ExposureRoute => {
                            if r#exposure_route.is_some() {
                                return Err(serde::de::Error::duplicate_field("exposureRoute"));
                            }
                            r#exposure_route = Some(map_access.next_value()?);
                        }
                        Field::Note => {
                            if r#note.is_some() {
                                return Err(serde::de::Error::duplicate_field("note"));
                            }
                            r#note = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(AllergyIntoleranceReaction {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#substance,
                    r#manifestation: r#manifestation.unwrap_or(vec![]),
                    r#description,
                    r#onset,
                    r#severity,
                    r#exposure_route,
                    r#note: r#note.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct AllergyIntolerance {
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
    pub r#type: Option<super::super::types::Code>,
    pub r#category: Vec<super::super::types::Code>,
    pub r#criticality: Option<super::super::types::Code>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#onset: Option<AllergyIntoleranceOnset>,
    pub r#recorded_date: Option<super::super::types::DateTime>,
    pub r#recorder: Option<Box<super::super::types::Reference>>,
    pub r#asserter: Option<Box<super::super::types::Reference>>,
    pub r#last_occurrence: Option<super::super::types::DateTime>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#reaction: Vec<AllergyIntoleranceReaction>,
}
impl serde::ser::Serialize for AllergyIntolerance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "AllergyIntolerance")?;
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
        if let Some(some) = self.r#type.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_type", &primitive_element)?;
            }
        }
        if !self.r#category.is_empty() {
            let values: Vec<_> = self.r#category.iter().map(|v| &v.value).collect();
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
                                id: &e.id,
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
        if let Some(some) = self.r#criticality.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("criticality", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_criticality", &primitive_element)?;
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
                AllergyIntoleranceOnset::Invalid => {
                    return Err(serde::ser::Error::custom("onset is invalid"))
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
        if let Some(some) = self.r#last_occurrence.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("lastOccurrence", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_lastOccurrence", &primitive_element)?;
            }
        }
        if !self.r#note.is_empty() {
            state.serialize_entry("note", &self.r#note)?;
        }
        if !self.r#reaction.is_empty() {
            state.serialize_entry("reaction", &self.r#reaction)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for AllergyIntolerance {
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "_type")]
            TypePrimitiveElement,
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "_category")]
            CategoryPrimitiveElement,
            #[serde(rename = "criticality")]
            Criticality,
            #[serde(rename = "_criticality")]
            CriticalityPrimitiveElement,
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "patient")]
            Patient,
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
            #[serde(rename = "recordedDate")]
            RecordedDate,
            #[serde(rename = "_recordedDate")]
            RecordedDatePrimitiveElement,
            #[serde(rename = "recorder")]
            Recorder,
            #[serde(rename = "asserter")]
            Asserter,
            #[serde(rename = "lastOccurrence")]
            LastOccurrence,
            #[serde(rename = "_lastOccurrence")]
            LastOccurrencePrimitiveElement,
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "reaction")]
            Reaction,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AllergyIntolerance;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AllergyIntolerance")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<AllergyIntolerance, V::Error>
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
                let mut r#type: Option<super::super::types::Code> = None;
                let mut r#category: Option<Vec<super::super::types::Code>> = None;
                let mut r#criticality: Option<super::super::types::Code> = None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#patient: Option<Box<super::super::types::Reference>> = None;
                let mut r#encounter: Option<Box<super::super::types::Reference>> = None;
                let mut r#onset: Option<AllergyIntoleranceOnset> = None;
                let mut r#recorded_date: Option<super::super::types::DateTime> = None;
                let mut r#recorder: Option<Box<super::super::types::Reference>> = None;
                let mut r#asserter: Option<Box<super::super::types::Reference>> = None;
                let mut r#last_occurrence: Option<super::super::types::DateTime> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#reaction: Option<Vec<AllergyIntoleranceReaction>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "AllergyIntolerance" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"AllergyIntolerance",
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
                        Field::Type => {
                            let some = r#type.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::TypePrimitiveElement => {
                            let some = r#type.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_type"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Category => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#category.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::CategoryPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#category.get_or_insert(Vec::with_capacity(elements.len()));
                            if vec.len() != elements.len() {
                                return Err(serde::de::Error::invalid_length(
                                    elements.len(),
                                    &"primitive values length",
                                ));
                            }
                            if vec
                                .iter()
                                .any(|e| e.id.is_some() || !e.extension.is_empty())
                            {
                                return Err(serde::de::Error::duplicate_field("_category"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::Criticality => {
                            let some = r#criticality.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("criticality"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::CriticalityPrimitiveElement => {
                            let some = r#criticality.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_criticality"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some(map_access.next_value()?);
                        }
                        Field::Patient => {
                            if r#patient.is_some() {
                                return Err(serde::de::Error::duplicate_field("patient"));
                            }
                            r#patient = Some(map_access.next_value()?);
                        }
                        Field::Encounter => {
                            if r#encounter.is_some() {
                                return Err(serde::de::Error::duplicate_field("encounter"));
                            }
                            r#encounter = Some(map_access.next_value()?);
                        }
                        Field::OnsetDateTime => {
                            let r#enum = r#onset.get_or_insert(AllergyIntoleranceOnset::DateTime(
                                Default::default(),
                            ));
                            if let AllergyIntoleranceOnset::DateTime(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("onsetDateTime"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("onset[x]"));
                            }
                        }
                        Field::OnsetDateTimePrimitiveElement => {
                            let r#enum = r#onset.get_or_insert(AllergyIntoleranceOnset::DateTime(
                                Default::default(),
                            ));
                            if let AllergyIntoleranceOnset::DateTime(variant) = r#enum {
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
                            r#onset = Some(AllergyIntoleranceOnset::Age(map_access.next_value()?));
                        }
                        Field::OnsetPeriod => {
                            if r#onset.is_some() {
                                return Err(serde::de::Error::duplicate_field("onsetPeriod"));
                            }
                            r#onset =
                                Some(AllergyIntoleranceOnset::Period(map_access.next_value()?));
                        }
                        Field::OnsetRange => {
                            if r#onset.is_some() {
                                return Err(serde::de::Error::duplicate_field("onsetRange"));
                            }
                            r#onset =
                                Some(AllergyIntoleranceOnset::Range(map_access.next_value()?));
                        }
                        Field::OnsetString => {
                            let r#enum = r#onset
                                .get_or_insert(AllergyIntoleranceOnset::String(Default::default()));
                            if let AllergyIntoleranceOnset::String(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("onsetString"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("onset[x]"));
                            }
                        }
                        Field::OnsetStringPrimitiveElement => {
                            let r#enum = r#onset
                                .get_or_insert(AllergyIntoleranceOnset::String(Default::default()));
                            if let AllergyIntoleranceOnset::String(variant) = r#enum {
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
                        Field::LastOccurrence => {
                            let some = r#last_occurrence.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastOccurrence"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::LastOccurrencePrimitiveElement => {
                            let some = r#last_occurrence.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_lastOccurrence"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Note => {
                            if r#note.is_some() {
                                return Err(serde::de::Error::duplicate_field("note"));
                            }
                            r#note = Some(map_access.next_value()?);
                        }
                        Field::Reaction => {
                            if r#reaction.is_some() {
                                return Err(serde::de::Error::duplicate_field("reaction"));
                            }
                            r#reaction = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(AllergyIntolerance {
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
                    r#type,
                    r#category: r#category.unwrap_or(vec![]),
                    r#criticality,
                    r#code,
                    r#patient: r#patient.ok_or(serde::de::Error::missing_field("patient"))?,
                    r#encounter,
                    r#onset,
                    r#recorded_date,
                    r#recorder,
                    r#asserter,
                    r#last_occurrence,
                    r#note: r#note.unwrap_or(vec![]),
                    r#reaction: r#reaction.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
