// Generated on 2022-07-25 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum TriggerDefinitionTiming {
    Timing(Box<super::super::types::Timing>),
    Reference(Box<super::super::types::Reference>),
    Date(Box<super::super::types::Date>),
    DateTime(Box<super::super::types::DateTime>),
    Invalid,
}
impl Default for TriggerDefinitionTiming {
    fn default() -> TriggerDefinitionTiming {
        TriggerDefinitionTiming::Invalid
    }
}
#[derive(Default, Debug, Clone)]
pub struct TriggerDefinition {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: super::super::types::Code,
    pub r#name: Option<super::super::types::String>,
    pub r#timing: Option<TriggerDefinitionTiming>,
    pub r#data: Vec<Box<super::super::types::DataRequirement>>,
    pub r#condition: Option<Box<super::super::types::Expression>>,
}
impl serde::ser::Serialize for TriggerDefinition {
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
        if let Some(some) = self.r#type.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("type", &some)?;
        }
        if self.r#type.id.is_some() || !self.r#type.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#type.id,
                extension: &self.r#type.extension,
            };
            state.serialize_entry("_type", &primitive_element)?;
        }
        if let Some(some) = self.r#name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("name", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_name", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#timing.as_ref() {
            match some {
                TriggerDefinitionTiming::Timing(ref value) => {
                    state.serialize_entry("timingTiming", value)?;
                }
                TriggerDefinitionTiming::Reference(ref value) => {
                    state.serialize_entry("timingReference", value)?;
                }
                TriggerDefinitionTiming::Date(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("timingDate", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_timingDate", &primitive_element)?;
                    }
                }
                TriggerDefinitionTiming::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("timingDateTime", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_timingDateTime", &primitive_element)?;
                    }
                }
                TriggerDefinitionTiming::Invalid => {
                    return Err(serde::ser::Error::custom("timing is invalid"))
                }
            }
        }
        if !self.r#data.is_empty() {
            state.serialize_entry("data", &self.r#data)?;
        }
        if let Some(some) = self.r#condition.as_ref() {
            state.serialize_entry("condition", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for TriggerDefinition {
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "_type")]
            TypePrimitiveElement,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            #[serde(rename = "timingTiming")]
            TimingTiming,
            #[serde(rename = "timingReference")]
            TimingReference,
            #[serde(rename = "timingDate")]
            TimingDate,
            #[serde(rename = "_timingDate")]
            TimingDatePrimitiveElement,
            #[serde(rename = "timingDateTime")]
            TimingDateTime,
            #[serde(rename = "_timingDateTime")]
            TimingDateTimePrimitiveElement,
            #[serde(rename = "data")]
            Data,
            #[serde(rename = "condition")]
            Condition,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TriggerDefinition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TriggerDefinition")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TriggerDefinition, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#type: Option<super::super::types::Code> = None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#timing: Option<TriggerDefinitionTiming> = None;
                let mut r#data: Option<Vec<Box<super::super::types::DataRequirement>>> = None;
                let mut r#condition: Option<Box<super::super::types::Expression>> = None;
                crate::json::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                            Field::Type => {
                                let some = r#type.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                            Field::Name => {
                                let some = r#name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                            Field::TimingTiming => {
                                if r#timing.is_some() {
                                    return Err(serde::de::Error::duplicate_field("timingTiming"));
                                }
                                r#timing =
                                    Some(TriggerDefinitionTiming::Timing(map_access.next_value()?));
                            }
                            Field::TimingReference => {
                                if r#timing.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "timingReference",
                                    ));
                                }
                                r#timing = Some(TriggerDefinitionTiming::Reference(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::TimingDate => {
                                let r#enum = r#timing.get_or_insert(TriggerDefinitionTiming::Date(
                                    Default::default(),
                                ));
                                if let TriggerDefinitionTiming::Date(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "timingDate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("timing[x]"));
                                }
                            }
                            Field::TimingDatePrimitiveElement => {
                                let r#enum = r#timing.get_or_insert(TriggerDefinitionTiming::Date(
                                    Default::default(),
                                ));
                                if let TriggerDefinitionTiming::Date(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_timingDate",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_timing[x]"));
                                }
                            }
                            Field::TimingDateTime => {
                                let r#enum = r#timing.get_or_insert(
                                    TriggerDefinitionTiming::DateTime(Default::default()),
                                );
                                if let TriggerDefinitionTiming::DateTime(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "timingDateTime",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("timing[x]"));
                                }
                            }
                            Field::TimingDateTimePrimitiveElement => {
                                let r#enum = r#timing.get_or_insert(
                                    TriggerDefinitionTiming::DateTime(Default::default()),
                                );
                                if let TriggerDefinitionTiming::DateTime(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_timingDateTime",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_timing[x]"));
                                }
                            }
                            Field::Data => {
                                if r#data.is_some() {
                                    return Err(serde::de::Error::duplicate_field("data"));
                                }
                                r#data = Some(map_access.next_value()?);
                            }
                            Field::Condition => {
                                if r#condition.is_some() {
                                    return Err(serde::de::Error::duplicate_field("condition"));
                                }
                                r#condition = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => {
                                if config.mode == crate::json::de::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "type",
                                            "name",
                                            "timingTiming",
                                            "timingReference",
                                            "timingDate",
                                            "timingDateTime",
                                            "data",
                                            "condition",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(TriggerDefinition {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#type: if config.mode == crate::json::de::DeserializationMode::Lax {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#name,
                        r#timing,
                        r#data: r#data.unwrap_or(vec![]),
                        r#condition,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
