// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum FamilyMemberHistoryBorn {
    Period(Box<super::super::types::Period>),
    Date(Box<super::super::types::Date>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for FamilyMemberHistoryBorn {
    fn default() -> FamilyMemberHistoryBorn {
        FamilyMemberHistoryBorn::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum FamilyMemberHistoryAge {
    Age(Box<super::super::types::Age>),
    Range(Box<super::super::types::Range>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for FamilyMemberHistoryAge {
    fn default() -> FamilyMemberHistoryAge {
        FamilyMemberHistoryAge::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum FamilyMemberHistoryDeceased {
    Boolean(Box<super::super::types::Boolean>),
    Age(Box<super::super::types::Age>),
    Range(Box<super::super::types::Range>),
    Date(Box<super::super::types::Date>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for FamilyMemberHistoryDeceased {
    fn default() -> FamilyMemberHistoryDeceased {
        FamilyMemberHistoryDeceased::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum FamilyMemberHistoryConditionOnset {
    Age(Box<super::super::types::Age>),
    Range(Box<super::super::types::Range>),
    Period(Box<super::super::types::Period>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for FamilyMemberHistoryConditionOnset {
    fn default() -> FamilyMemberHistoryConditionOnset {
        FamilyMemberHistoryConditionOnset::Invalid
    }
}
#[derive(Default, Debug, Clone)]
pub struct FamilyMemberHistoryCondition {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#outcome: Option<Box<super::super::types::CodeableConcept>>,
    pub r#contributed_to_death: Option<super::super::types::Boolean>,
    pub r#onset: Option<FamilyMemberHistoryConditionOnset>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
impl serde::ser::Serialize for FamilyMemberHistoryCondition {
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
        state.serialize_entry("code", &self.r#code)?;
        if let Some(some) = self.r#outcome.as_ref() {
            state.serialize_entry("outcome", some)?;
        }
        if let Some(some) = self.r#contributed_to_death.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("contributedToDeath", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_contributedToDeath", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#onset.as_ref() {
            match some {
                FamilyMemberHistoryConditionOnset::Age(ref value) => {
                    state.serialize_entry("onsetAge", value)?;
                }
                FamilyMemberHistoryConditionOnset::Range(ref value) => {
                    state.serialize_entry("onsetRange", value)?;
                }
                FamilyMemberHistoryConditionOnset::Period(ref value) => {
                    state.serialize_entry("onsetPeriod", value)?;
                }
                FamilyMemberHistoryConditionOnset::String(ref value) => {
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
                FamilyMemberHistoryConditionOnset::Invalid => {
                    return Err(serde::ser::Error::custom("onset is invalid"))
                }
            }
        }
        if !self.r#note.is_empty() {
            state.serialize_entry("note", &self.r#note)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for FamilyMemberHistoryCondition {
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
            #[serde(rename = "outcome")]
            Outcome,
            #[serde(rename = "contributedToDeath")]
            ContributedToDeath,
            #[serde(rename = "_contributedToDeath")]
            ContributedToDeathPrimitiveElement,
            #[serde(rename = "onsetAge")]
            OnsetAge,
            #[serde(rename = "onsetRange")]
            OnsetRange,
            #[serde(rename = "onsetPeriod")]
            OnsetPeriod,
            #[serde(rename = "onsetString")]
            OnsetString,
            #[serde(rename = "_onsetString")]
            OnsetStringPrimitiveElement,
            #[serde(rename = "note")]
            Note,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FamilyMemberHistoryCondition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("FamilyMemberHistoryCondition")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<FamilyMemberHistoryCondition, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#outcome: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#contributed_to_death: Option<super::super::types::Boolean> = None;
                let mut r#onset: Option<FamilyMemberHistoryConditionOnset> = None;
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
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some(map_access.next_value()?);
                        }
                        Field::Outcome => {
                            if r#outcome.is_some() {
                                return Err(serde::de::Error::duplicate_field("outcome"));
                            }
                            r#outcome = Some(map_access.next_value()?);
                        }
                        Field::ContributedToDeath => {
                            let some = r#contributed_to_death.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "contributedToDeath",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ContributedToDeathPrimitiveElement => {
                            let some = r#contributed_to_death.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_contributedToDeath",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::OnsetAge => {
                            if r#onset.is_some() {
                                return Err(serde::de::Error::duplicate_field("onsetAge"));
                            }
                            r#onset = Some(FamilyMemberHistoryConditionOnset::Age(
                                map_access.next_value()?,
                            ));
                        }
                        Field::OnsetRange => {
                            if r#onset.is_some() {
                                return Err(serde::de::Error::duplicate_field("onsetRange"));
                            }
                            r#onset = Some(FamilyMemberHistoryConditionOnset::Range(
                                map_access.next_value()?,
                            ));
                        }
                        Field::OnsetPeriod => {
                            if r#onset.is_some() {
                                return Err(serde::de::Error::duplicate_field("onsetPeriod"));
                            }
                            r#onset = Some(FamilyMemberHistoryConditionOnset::Period(
                                map_access.next_value()?,
                            ));
                        }
                        Field::OnsetString => {
                            let r#enum = r#onset.get_or_insert(
                                FamilyMemberHistoryConditionOnset::String(Default::default()),
                            );
                            if let FamilyMemberHistoryConditionOnset::String(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("onsetString"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("onset[x]"));
                            }
                        }
                        Field::OnsetStringPrimitiveElement => {
                            let r#enum = r#onset.get_or_insert(
                                FamilyMemberHistoryConditionOnset::String(Default::default()),
                            );
                            if let FamilyMemberHistoryConditionOnset::String(variant) = r#enum {
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
                        Field::Note => {
                            if r#note.is_some() {
                                return Err(serde::de::Error::duplicate_field("note"));
                            }
                            r#note = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(FamilyMemberHistoryCondition {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#code: r#code.ok_or(serde::de::Error::missing_field("code"))?,
                    r#outcome,
                    r#contributed_to_death,
                    r#onset,
                    r#note: r#note.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct FamilyMemberHistory {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    pub r#status: super::super::types::Code,
    pub r#data_absent_reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#name: Option<super::super::types::String>,
    pub r#relationship: Box<super::super::types::CodeableConcept>,
    pub r#sex: Option<Box<super::super::types::CodeableConcept>>,
    pub r#born: Option<FamilyMemberHistoryBorn>,
    pub r#age: Option<FamilyMemberHistoryAge>,
    pub r#estimated_age: Option<super::super::types::Boolean>,
    pub r#deceased: Option<FamilyMemberHistoryDeceased>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#condition: Vec<FamilyMemberHistoryCondition>,
}
impl serde::ser::Serialize for FamilyMemberHistory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "FamilyMemberHistory")?;
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
        if !self.r#instantiates_canonical.is_empty() {
            let values: Vec<_> = self
                .r#instantiates_canonical
                .iter()
                .map(|v| &v.value)
                .collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("instantiatesCanonical", &values)?;
            }
            let requires_elements = self
                .r#instantiates_canonical
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#instantiates_canonical
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
                state.serialize_entry("_instantiatesCanonical", &primitive_elements)?;
            }
        }
        if !self.r#instantiates_uri.is_empty() {
            let values: Vec<_> = self.r#instantiates_uri.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("instantiatesUri", &values)?;
            }
            let requires_elements = self
                .r#instantiates_uri
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#instantiates_uri
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
                state.serialize_entry("_instantiatesUri", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#status.value.as_ref() {
            state.serialize_entry("status", some)?;
        }
        if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#status.id,
                extension: &self.r#status.extension,
            };
            state.serialize_entry("_status", &primitive_element)?;
        }
        if let Some(some) = self.r#data_absent_reason.as_ref() {
            state.serialize_entry("dataAbsentReason", some)?;
        }
        state.serialize_entry("patient", &self.r#patient)?;
        if let Some(some) = self.r#date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("date", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_date", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("name", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_name", &primitive_element)?;
            }
        }
        state.serialize_entry("relationship", &self.r#relationship)?;
        if let Some(some) = self.r#sex.as_ref() {
            state.serialize_entry("sex", some)?;
        }
        if let Some(some) = self.r#born.as_ref() {
            match some {
                FamilyMemberHistoryBorn::Period(ref value) => {
                    state.serialize_entry("bornPeriod", value)?;
                }
                FamilyMemberHistoryBorn::Date(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("bornDate", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_bornDate", &primitive_element)?;
                    }
                }
                FamilyMemberHistoryBorn::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("bornString", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_bornString", &primitive_element)?;
                    }
                }
                FamilyMemberHistoryBorn::Invalid => {
                    return Err(serde::ser::Error::custom("born is invalid"))
                }
            }
        }
        if let Some(some) = self.r#age.as_ref() {
            match some {
                FamilyMemberHistoryAge::Age(ref value) => {
                    state.serialize_entry("ageAge", value)?;
                }
                FamilyMemberHistoryAge::Range(ref value) => {
                    state.serialize_entry("ageRange", value)?;
                }
                FamilyMemberHistoryAge::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("ageString", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_ageString", &primitive_element)?;
                    }
                }
                FamilyMemberHistoryAge::Invalid => {
                    return Err(serde::ser::Error::custom("age is invalid"))
                }
            }
        }
        if let Some(some) = self.r#estimated_age.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("estimatedAge", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_estimatedAge", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#deceased.as_ref() {
            match some {
                FamilyMemberHistoryDeceased::Boolean(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("deceasedBoolean", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_deceasedBoolean", &primitive_element)?;
                    }
                }
                FamilyMemberHistoryDeceased::Age(ref value) => {
                    state.serialize_entry("deceasedAge", value)?;
                }
                FamilyMemberHistoryDeceased::Range(ref value) => {
                    state.serialize_entry("deceasedRange", value)?;
                }
                FamilyMemberHistoryDeceased::Date(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("deceasedDate", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_deceasedDate", &primitive_element)?;
                    }
                }
                FamilyMemberHistoryDeceased::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("deceasedString", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_deceasedString", &primitive_element)?;
                    }
                }
                FamilyMemberHistoryDeceased::Invalid => {
                    return Err(serde::ser::Error::custom("deceased is invalid"))
                }
            }
        }
        if !self.r#reason_code.is_empty() {
            state.serialize_entry("reasonCode", &self.r#reason_code)?;
        }
        if !self.r#reason_reference.is_empty() {
            state.serialize_entry("reasonReference", &self.r#reason_reference)?;
        }
        if !self.r#note.is_empty() {
            state.serialize_entry("note", &self.r#note)?;
        }
        if !self.r#condition.is_empty() {
            state.serialize_entry("condition", &self.r#condition)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for FamilyMemberHistory {
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
            #[serde(rename = "instantiatesCanonical")]
            InstantiatesCanonical,
            #[serde(rename = "_instantiatesCanonical")]
            InstantiatesCanonicalPrimitiveElement,
            #[serde(rename = "instantiatesUri")]
            InstantiatesUri,
            #[serde(rename = "_instantiatesUri")]
            InstantiatesUriPrimitiveElement,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "dataAbsentReason")]
            DataAbsentReason,
            #[serde(rename = "patient")]
            Patient,
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "_date")]
            DatePrimitiveElement,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            #[serde(rename = "relationship")]
            Relationship,
            #[serde(rename = "sex")]
            Sex,
            #[serde(rename = "bornPeriod")]
            BornPeriod,
            #[serde(rename = "bornDate")]
            BornDate,
            #[serde(rename = "_bornDate")]
            BornDatePrimitiveElement,
            #[serde(rename = "bornString")]
            BornString,
            #[serde(rename = "_bornString")]
            BornStringPrimitiveElement,
            #[serde(rename = "ageAge")]
            AgeAge,
            #[serde(rename = "ageRange")]
            AgeRange,
            #[serde(rename = "ageString")]
            AgeString,
            #[serde(rename = "_ageString")]
            AgeStringPrimitiveElement,
            #[serde(rename = "estimatedAge")]
            EstimatedAge,
            #[serde(rename = "_estimatedAge")]
            EstimatedAgePrimitiveElement,
            #[serde(rename = "deceasedBoolean")]
            DeceasedBoolean,
            #[serde(rename = "_deceasedBoolean")]
            DeceasedBooleanPrimitiveElement,
            #[serde(rename = "deceasedAge")]
            DeceasedAge,
            #[serde(rename = "deceasedRange")]
            DeceasedRange,
            #[serde(rename = "deceasedDate")]
            DeceasedDate,
            #[serde(rename = "_deceasedDate")]
            DeceasedDatePrimitiveElement,
            #[serde(rename = "deceasedString")]
            DeceasedString,
            #[serde(rename = "_deceasedString")]
            DeceasedStringPrimitiveElement,
            #[serde(rename = "reasonCode")]
            ReasonCode,
            #[serde(rename = "reasonReference")]
            ReasonReference,
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "condition")]
            Condition,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FamilyMemberHistory;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("FamilyMemberHistory")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<FamilyMemberHistory, V::Error>
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
                let mut r#instantiates_canonical: Option<Vec<super::super::types::Canonical>> =
                    None;
                let mut r#instantiates_uri: Option<Vec<super::super::types::Uri>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#data_absent_reason: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#patient: Option<Box<super::super::types::Reference>> = None;
                let mut r#date: Option<super::super::types::DateTime> = None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#relationship: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#sex: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#born: Option<FamilyMemberHistoryBorn> = None;
                let mut r#age: Option<FamilyMemberHistoryAge> = None;
                let mut r#estimated_age: Option<super::super::types::Boolean> = None;
                let mut r#deceased: Option<FamilyMemberHistoryDeceased> = None;
                let mut r#reason_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#reason_reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#condition: Option<Vec<FamilyMemberHistoryCondition>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "FamilyMemberHistory" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"FamilyMemberHistory",
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
                        Field::InstantiatesCanonical => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#instantiates_canonical.get_or_insert(
                                std::iter::repeat(Default::default())
                                    .take(values.len())
                                    .collect::<Vec<_>>(),
                            );
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field(
                                    "instantiatesCanonical",
                                ));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::InstantiatesCanonicalPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#instantiates_canonical.get_or_insert(
                                std::iter::repeat(Default::default())
                                    .take(elements.len())
                                    .collect::<Vec<_>>(),
                            );
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
                                return Err(serde::de::Error::duplicate_field(
                                    "_instantiatesCanonical",
                                ));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::InstantiatesUri => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#instantiates_uri.get_or_insert(
                                std::iter::repeat(Default::default())
                                    .take(values.len())
                                    .collect::<Vec<_>>(),
                            );
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("instantiatesUri"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::InstantiatesUriPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#instantiates_uri.get_or_insert(
                                std::iter::repeat(Default::default())
                                    .take(elements.len())
                                    .collect::<Vec<_>>(),
                            );
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
                                return Err(serde::de::Error::duplicate_field("_instantiatesUri"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::Status => {
                            let some = r#status.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            some.value = Some(map_access.next_value()?);
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
                        Field::DataAbsentReason => {
                            if r#data_absent_reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataAbsentReason"));
                            }
                            r#data_absent_reason = Some(map_access.next_value()?);
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
                            some.value = Some(map_access.next_value()?);
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
                        Field::Name => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::NamePrimitiveElement => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Relationship => {
                            if r#relationship.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationship"));
                            }
                            r#relationship = Some(map_access.next_value()?);
                        }
                        Field::Sex => {
                            if r#sex.is_some() {
                                return Err(serde::de::Error::duplicate_field("sex"));
                            }
                            r#sex = Some(map_access.next_value()?);
                        }
                        Field::BornPeriod => {
                            if r#born.is_some() {
                                return Err(serde::de::Error::duplicate_field("bornPeriod"));
                            }
                            r#born =
                                Some(FamilyMemberHistoryBorn::Period(map_access.next_value()?));
                        }
                        Field::BornDate => {
                            let r#enum = r#born
                                .get_or_insert(FamilyMemberHistoryBorn::Date(Default::default()));
                            if let FamilyMemberHistoryBorn::Date(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("bornDate"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("born[x]"));
                            }
                        }
                        Field::BornDatePrimitiveElement => {
                            let r#enum = r#born
                                .get_or_insert(FamilyMemberHistoryBorn::Date(Default::default()));
                            if let FamilyMemberHistoryBorn::Date(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_bornDate"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_born[x]"));
                            }
                        }
                        Field::BornString => {
                            let r#enum = r#born
                                .get_or_insert(FamilyMemberHistoryBorn::String(Default::default()));
                            if let FamilyMemberHistoryBorn::String(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("bornString"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("born[x]"));
                            }
                        }
                        Field::BornStringPrimitiveElement => {
                            let r#enum = r#born
                                .get_or_insert(FamilyMemberHistoryBorn::String(Default::default()));
                            if let FamilyMemberHistoryBorn::String(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_bornString"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_born[x]"));
                            }
                        }
                        Field::AgeAge => {
                            if r#age.is_some() {
                                return Err(serde::de::Error::duplicate_field("ageAge"));
                            }
                            r#age = Some(FamilyMemberHistoryAge::Age(map_access.next_value()?));
                        }
                        Field::AgeRange => {
                            if r#age.is_some() {
                                return Err(serde::de::Error::duplicate_field("ageRange"));
                            }
                            r#age = Some(FamilyMemberHistoryAge::Range(map_access.next_value()?));
                        }
                        Field::AgeString => {
                            let r#enum = r#age
                                .get_or_insert(FamilyMemberHistoryAge::String(Default::default()));
                            if let FamilyMemberHistoryAge::String(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("ageString"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("age[x]"));
                            }
                        }
                        Field::AgeStringPrimitiveElement => {
                            let r#enum = r#age
                                .get_or_insert(FamilyMemberHistoryAge::String(Default::default()));
                            if let FamilyMemberHistoryAge::String(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_ageString"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_age[x]"));
                            }
                        }
                        Field::EstimatedAge => {
                            let some = r#estimated_age.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("estimatedAge"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::EstimatedAgePrimitiveElement => {
                            let some = r#estimated_age.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_estimatedAge"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::DeceasedBoolean => {
                            let r#enum = r#deceased.get_or_insert(
                                FamilyMemberHistoryDeceased::Boolean(Default::default()),
                            );
                            if let FamilyMemberHistoryDeceased::Boolean(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "deceasedBoolean",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("deceased[x]"));
                            }
                        }
                        Field::DeceasedBooleanPrimitiveElement => {
                            let r#enum = r#deceased.get_or_insert(
                                FamilyMemberHistoryDeceased::Boolean(Default::default()),
                            );
                            if let FamilyMemberHistoryDeceased::Boolean(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_deceasedBoolean",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_deceased[x]"));
                            }
                        }
                        Field::DeceasedAge => {
                            if r#deceased.is_some() {
                                return Err(serde::de::Error::duplicate_field("deceasedAge"));
                            }
                            r#deceased =
                                Some(FamilyMemberHistoryDeceased::Age(map_access.next_value()?));
                        }
                        Field::DeceasedRange => {
                            if r#deceased.is_some() {
                                return Err(serde::de::Error::duplicate_field("deceasedRange"));
                            }
                            r#deceased =
                                Some(FamilyMemberHistoryDeceased::Range(map_access.next_value()?));
                        }
                        Field::DeceasedDate => {
                            let r#enum = r#deceased.get_or_insert(
                                FamilyMemberHistoryDeceased::Date(Default::default()),
                            );
                            if let FamilyMemberHistoryDeceased::Date(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("deceasedDate"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("deceased[x]"));
                            }
                        }
                        Field::DeceasedDatePrimitiveElement => {
                            let r#enum = r#deceased.get_or_insert(
                                FamilyMemberHistoryDeceased::Date(Default::default()),
                            );
                            if let FamilyMemberHistoryDeceased::Date(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_deceasedDate"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_deceased[x]"));
                            }
                        }
                        Field::DeceasedString => {
                            let r#enum = r#deceased.get_or_insert(
                                FamilyMemberHistoryDeceased::String(Default::default()),
                            );
                            if let FamilyMemberHistoryDeceased::String(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "deceasedString",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("deceased[x]"));
                            }
                        }
                        Field::DeceasedStringPrimitiveElement => {
                            let r#enum = r#deceased.get_or_insert(
                                FamilyMemberHistoryDeceased::String(Default::default()),
                            );
                            if let FamilyMemberHistoryDeceased::String(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_deceasedString",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_deceased[x]"));
                            }
                        }
                        Field::ReasonCode => {
                            if r#reason_code.is_some() {
                                return Err(serde::de::Error::duplicate_field("reasonCode"));
                            }
                            r#reason_code = Some(map_access.next_value()?);
                        }
                        Field::ReasonReference => {
                            if r#reason_reference.is_some() {
                                return Err(serde::de::Error::duplicate_field("reasonReference"));
                            }
                            r#reason_reference = Some(map_access.next_value()?);
                        }
                        Field::Note => {
                            if r#note.is_some() {
                                return Err(serde::de::Error::duplicate_field("note"));
                            }
                            r#note = Some(map_access.next_value()?);
                        }
                        Field::Condition => {
                            if r#condition.is_some() {
                                return Err(serde::de::Error::duplicate_field("condition"));
                            }
                            r#condition = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(FamilyMemberHistory {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#instantiates_canonical: r#instantiates_canonical.unwrap_or(vec![]),
                    r#instantiates_uri: r#instantiates_uri.unwrap_or(vec![]),
                    r#status: r#status.ok_or(serde::de::Error::missing_field("status"))?,
                    r#data_absent_reason,
                    r#patient: r#patient.ok_or(serde::de::Error::missing_field("patient"))?,
                    r#date,
                    r#name,
                    r#relationship: r#relationship
                        .ok_or(serde::de::Error::missing_field("relationship"))?,
                    r#sex,
                    r#born,
                    r#age,
                    r#estimated_age,
                    r#deceased,
                    r#reason_code: r#reason_code.unwrap_or(vec![]),
                    r#reason_reference: r#reason_reference.unwrap_or(vec![]),
                    r#note: r#note.unwrap_or(vec![]),
                    r#condition: r#condition.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
