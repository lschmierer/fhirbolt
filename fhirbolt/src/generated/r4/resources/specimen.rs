// Generated on 2022-07-25 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum SpecimenCollectionCollected {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for SpecimenCollectionCollected {
    fn default() -> SpecimenCollectionCollected {
        SpecimenCollectionCollected::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum SpecimenCollectionFastingStatus {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Duration(Box<super::super::types::Duration>),
    Invalid,
}
impl Default for SpecimenCollectionFastingStatus {
    fn default() -> SpecimenCollectionFastingStatus {
        SpecimenCollectionFastingStatus::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum SpecimenProcessingTime {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for SpecimenProcessingTime {
    fn default() -> SpecimenProcessingTime {
        SpecimenProcessingTime::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum SpecimenContainerAdditive {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for SpecimenContainerAdditive {
    fn default() -> SpecimenContainerAdditive {
        SpecimenContainerAdditive::Invalid
    }
}
#[derive(Default, Debug, Clone)]
pub struct SpecimenCollection {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#collector: Option<Box<super::super::types::Reference>>,
    pub r#collected: Option<SpecimenCollectionCollected>,
    pub r#duration: Option<Box<super::super::types::Duration>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    pub r#fasting_status: Option<SpecimenCollectionFastingStatus>,
}
impl serde::ser::Serialize for SpecimenCollection {
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
        if let Some(some) = self.r#collector.as_ref() {
            state.serialize_entry("collector", some)?;
        }
        if let Some(some) = self.r#collected.as_ref() {
            match some {
                SpecimenCollectionCollected::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("collectedDateTime", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_collectedDateTime", &primitive_element)?;
                    }
                }
                SpecimenCollectionCollected::Period(ref value) => {
                    state.serialize_entry("collectedPeriod", value)?;
                }
                SpecimenCollectionCollected::Invalid => {
                    return Err(serde::ser::Error::custom("collected is invalid"))
                }
            }
        }
        if let Some(some) = self.r#duration.as_ref() {
            state.serialize_entry("duration", some)?;
        }
        if let Some(some) = self.r#quantity.as_ref() {
            state.serialize_entry("quantity", some)?;
        }
        if let Some(some) = self.r#method.as_ref() {
            state.serialize_entry("method", some)?;
        }
        if let Some(some) = self.r#body_site.as_ref() {
            state.serialize_entry("bodySite", some)?;
        }
        if let Some(some) = self.r#fasting_status.as_ref() {
            match some {
                SpecimenCollectionFastingStatus::CodeableConcept(ref value) => {
                    state.serialize_entry("fastingStatusCodeableConcept", value)?;
                }
                SpecimenCollectionFastingStatus::Duration(ref value) => {
                    state.serialize_entry("fastingStatusDuration", value)?;
                }
                SpecimenCollectionFastingStatus::Invalid => {
                    return Err(serde::ser::Error::custom("fasting_status is invalid"))
                }
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SpecimenCollection {
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
            #[serde(rename = "collector")]
            Collector,
            #[serde(rename = "collectedDateTime")]
            CollectedDateTime,
            #[serde(rename = "_collectedDateTime")]
            CollectedDateTimePrimitiveElement,
            #[serde(rename = "collectedPeriod")]
            CollectedPeriod,
            #[serde(rename = "duration")]
            Duration,
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "method")]
            Method,
            #[serde(rename = "bodySite")]
            BodySite,
            #[serde(rename = "fastingStatusCodeableConcept")]
            FastingStatusCodeableConcept,
            #[serde(rename = "fastingStatusDuration")]
            FastingStatusDuration,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SpecimenCollection;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SpecimenCollection")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SpecimenCollection, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#collector: Option<Box<super::super::types::Reference>> = None;
                let mut r#collected: Option<SpecimenCollectionCollected> = None;
                let mut r#duration: Option<Box<super::super::types::Duration>> = None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#method: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#body_site: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#fasting_status: Option<SpecimenCollectionFastingStatus> = None;
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
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Collector => {
                                if r#collector.is_some() {
                                    return Err(serde::de::Error::duplicate_field("collector"));
                                }
                                r#collector = Some(map_access.next_value()?);
                            }
                            Field::CollectedDateTime => {
                                let r#enum = r#collected.get_or_insert(
                                    SpecimenCollectionCollected::DateTime(Default::default()),
                                );
                                if let SpecimenCollectionCollected::DateTime(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "collectedDateTime",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("collected[x]"));
                                }
                            }
                            Field::CollectedDateTimePrimitiveElement => {
                                let r#enum = r#collected.get_or_insert(
                                    SpecimenCollectionCollected::DateTime(Default::default()),
                                );
                                if let SpecimenCollectionCollected::DateTime(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_collectedDateTime",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_collected[x]"));
                                }
                            }
                            Field::CollectedPeriod => {
                                if r#collected.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "collectedPeriod",
                                    ));
                                }
                                r#collected = Some(SpecimenCollectionCollected::Period(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::Duration => {
                                if r#duration.is_some() {
                                    return Err(serde::de::Error::duplicate_field("duration"));
                                }
                                r#duration = Some(map_access.next_value()?);
                            }
                            Field::Quantity => {
                                if r#quantity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("quantity"));
                                }
                                r#quantity = Some(map_access.next_value()?);
                            }
                            Field::Method => {
                                if r#method.is_some() {
                                    return Err(serde::de::Error::duplicate_field("method"));
                                }
                                r#method = Some(map_access.next_value()?);
                            }
                            Field::BodySite => {
                                if r#body_site.is_some() {
                                    return Err(serde::de::Error::duplicate_field("bodySite"));
                                }
                                r#body_site = Some(map_access.next_value()?);
                            }
                            Field::FastingStatusCodeableConcept => {
                                if r#fasting_status.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "fastingStatusCodeableConcept",
                                    ));
                                }
                                r#fasting_status =
                                    Some(SpecimenCollectionFastingStatus::CodeableConcept(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::FastingStatusDuration => {
                                if r#fasting_status.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "fastingStatusDuration",
                                    ));
                                }
                                r#fasting_status = Some(SpecimenCollectionFastingStatus::Duration(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::Unknown(key) => {
                                if config.mode == crate::json::de::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "collector",
                                            "collectedDateTime",
                                            "collectedPeriod",
                                            "duration",
                                            "quantity",
                                            "method",
                                            "bodySite",
                                            "fastingStatusCodeableConcept",
                                            "fastingStatusDuration",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(SpecimenCollection {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#collector,
                        r#collected,
                        r#duration,
                        r#quantity,
                        r#method,
                        r#body_site,
                        r#fasting_status,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SpecimenProcessing {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Option<super::super::types::String>,
    pub r#procedure: Option<Box<super::super::types::CodeableConcept>>,
    pub r#additive: Vec<Box<super::super::types::Reference>>,
    pub r#time: Option<SpecimenProcessingTime>,
}
impl serde::ser::Serialize for SpecimenProcessing {
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
        if let Some(some) = self.r#procedure.as_ref() {
            state.serialize_entry("procedure", some)?;
        }
        if !self.r#additive.is_empty() {
            state.serialize_entry("additive", &self.r#additive)?;
        }
        if let Some(some) = self.r#time.as_ref() {
            match some {
                SpecimenProcessingTime::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("timeDateTime", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_timeDateTime", &primitive_element)?;
                    }
                }
                SpecimenProcessingTime::Period(ref value) => {
                    state.serialize_entry("timePeriod", value)?;
                }
                SpecimenProcessingTime::Invalid => {
                    return Err(serde::ser::Error::custom("time is invalid"))
                }
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SpecimenProcessing {
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
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "procedure")]
            Procedure,
            #[serde(rename = "additive")]
            Additive,
            #[serde(rename = "timeDateTime")]
            TimeDateTime,
            #[serde(rename = "_timeDateTime")]
            TimeDateTimePrimitiveElement,
            #[serde(rename = "timePeriod")]
            TimePeriod,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SpecimenProcessing;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SpecimenProcessing")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SpecimenProcessing, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#procedure: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#additive: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#time: Option<SpecimenProcessingTime> = None;
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
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
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
                            Field::Procedure => {
                                if r#procedure.is_some() {
                                    return Err(serde::de::Error::duplicate_field("procedure"));
                                }
                                r#procedure = Some(map_access.next_value()?);
                            }
                            Field::Additive => {
                                if r#additive.is_some() {
                                    return Err(serde::de::Error::duplicate_field("additive"));
                                }
                                r#additive = Some(map_access.next_value()?);
                            }
                            Field::TimeDateTime => {
                                let r#enum = r#time.get_or_insert(
                                    SpecimenProcessingTime::DateTime(Default::default()),
                                );
                                if let SpecimenProcessingTime::DateTime(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "timeDateTime",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("time[x]"));
                                }
                            }
                            Field::TimeDateTimePrimitiveElement => {
                                let r#enum = r#time.get_or_insert(
                                    SpecimenProcessingTime::DateTime(Default::default()),
                                );
                                if let SpecimenProcessingTime::DateTime(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_timeDateTime",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_time[x]"));
                                }
                            }
                            Field::TimePeriod => {
                                if r#time.is_some() {
                                    return Err(serde::de::Error::duplicate_field("timePeriod"));
                                }
                                r#time =
                                    Some(SpecimenProcessingTime::Period(map_access.next_value()?));
                            }
                            Field::Unknown(key) => {
                                if config.mode == crate::json::de::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "description",
                                            "procedure",
                                            "additive",
                                            "timeDateTime",
                                            "timePeriod",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(SpecimenProcessing {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#description,
                        r#procedure,
                        r#additive: r#additive.unwrap_or(vec![]),
                        r#time,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SpecimenContainer {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#description: Option<super::super::types::String>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#capacity: Option<Box<super::super::types::Quantity>>,
    pub r#specimen_quantity: Option<Box<super::super::types::Quantity>>,
    pub r#additive: Option<SpecimenContainerAdditive>,
}
impl serde::ser::Serialize for SpecimenContainer {
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
        if !self.r#identifier.is_empty() {
            state.serialize_entry("identifier", &self.r#identifier)?;
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
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#capacity.as_ref() {
            state.serialize_entry("capacity", some)?;
        }
        if let Some(some) = self.r#specimen_quantity.as_ref() {
            state.serialize_entry("specimenQuantity", some)?;
        }
        if let Some(some) = self.r#additive.as_ref() {
            match some {
                SpecimenContainerAdditive::CodeableConcept(ref value) => {
                    state.serialize_entry("additiveCodeableConcept", value)?;
                }
                SpecimenContainerAdditive::Reference(ref value) => {
                    state.serialize_entry("additiveReference", value)?;
                }
                SpecimenContainerAdditive::Invalid => {
                    return Err(serde::ser::Error::custom("additive is invalid"))
                }
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SpecimenContainer {
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
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "capacity")]
            Capacity,
            #[serde(rename = "specimenQuantity")]
            SpecimenQuantity,
            #[serde(rename = "additiveCodeableConcept")]
            AdditiveCodeableConcept,
            #[serde(rename = "additiveReference")]
            AdditiveReference,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SpecimenContainer;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SpecimenContainer")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SpecimenContainer, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#capacity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#specimen_quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#additive: Option<SpecimenContainerAdditive> = None;
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
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Capacity => {
                                if r#capacity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("capacity"));
                                }
                                r#capacity = Some(map_access.next_value()?);
                            }
                            Field::SpecimenQuantity => {
                                if r#specimen_quantity.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "specimenQuantity",
                                    ));
                                }
                                r#specimen_quantity = Some(map_access.next_value()?);
                            }
                            Field::AdditiveCodeableConcept => {
                                if r#additive.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "additiveCodeableConcept",
                                    ));
                                }
                                r#additive = Some(SpecimenContainerAdditive::CodeableConcept(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::AdditiveReference => {
                                if r#additive.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "additiveReference",
                                    ));
                                }
                                r#additive = Some(SpecimenContainerAdditive::Reference(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::Unknown(key) => {
                                if config.mode == crate::json::de::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "description",
                                            "type",
                                            "capacity",
                                            "specimenQuantity",
                                            "additiveCodeableConcept",
                                            "additiveReference",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(SpecimenContainer {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#description,
                        r#type,
                        r#capacity,
                        r#specimen_quantity,
                        r#additive,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct Specimen {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#accession_identifier: Option<Box<super::super::types::Identifier>>,
    pub r#status: Option<super::super::types::Code>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#received_time: Option<super::super::types::DateTime>,
    pub r#parent: Vec<Box<super::super::types::Reference>>,
    pub r#request: Vec<Box<super::super::types::Reference>>,
    pub r#collection: Option<SpecimenCollection>,
    pub r#processing: Vec<SpecimenProcessing>,
    pub r#container: Vec<SpecimenContainer>,
    pub r#condition: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
impl serde::ser::Serialize for Specimen {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Specimen")?;
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
        if let Some(some) = self.r#accession_identifier.as_ref() {
            state.serialize_entry("accessionIdentifier", some)?;
        }
        if let Some(some) = self.r#status.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("status", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_status", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#subject.as_ref() {
            state.serialize_entry("subject", some)?;
        }
        if let Some(some) = self.r#received_time.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("receivedTime", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_receivedTime", &primitive_element)?;
            }
        }
        if !self.r#parent.is_empty() {
            state.serialize_entry("parent", &self.r#parent)?;
        }
        if !self.r#request.is_empty() {
            state.serialize_entry("request", &self.r#request)?;
        }
        if let Some(some) = self.r#collection.as_ref() {
            state.serialize_entry("collection", some)?;
        }
        if !self.r#processing.is_empty() {
            state.serialize_entry("processing", &self.r#processing)?;
        }
        if !self.r#container.is_empty() {
            state.serialize_entry("container", &self.r#container)?;
        }
        if !self.r#condition.is_empty() {
            state.serialize_entry("condition", &self.r#condition)?;
        }
        if !self.r#note.is_empty() {
            state.serialize_entry("note", &self.r#note)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for Specimen {
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
            #[serde(rename = "accessionIdentifier")]
            AccessionIdentifier,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "receivedTime")]
            ReceivedTime,
            #[serde(rename = "_receivedTime")]
            ReceivedTimePrimitiveElement,
            #[serde(rename = "parent")]
            Parent,
            #[serde(rename = "request")]
            Request,
            #[serde(rename = "collection")]
            Collection,
            #[serde(rename = "processing")]
            Processing,
            #[serde(rename = "container")]
            Container,
            #[serde(rename = "condition")]
            Condition,
            #[serde(rename = "note")]
            Note,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Specimen;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Specimen")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Specimen, V::Error>
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
                let mut r#accession_identifier: Option<Box<super::super::types::Identifier>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#received_time: Option<super::super::types::DateTime> = None;
                let mut r#parent: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#request: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#collection: Option<SpecimenCollection> = None;
                let mut r#processing: Option<Vec<SpecimenProcessing>> = None;
                let mut r#container: Option<Vec<SpecimenContainer>> = None;
                let mut r#condition: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                crate::json::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "Specimen" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"Specimen",
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
                            Field::AccessionIdentifier => {
                                if r#accession_identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "accessionIdentifier",
                                    ));
                                }
                                r#accession_identifier = Some(map_access.next_value()?);
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
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Subject => {
                                if r#subject.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subject"));
                                }
                                r#subject = Some(map_access.next_value()?);
                            }
                            Field::ReceivedTime => {
                                let some = r#received_time.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("receivedTime"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ReceivedTimePrimitiveElement => {
                                let some = r#received_time.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_receivedTime"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Parent => {
                                if r#parent.is_some() {
                                    return Err(serde::de::Error::duplicate_field("parent"));
                                }
                                r#parent = Some(map_access.next_value()?);
                            }
                            Field::Request => {
                                if r#request.is_some() {
                                    return Err(serde::de::Error::duplicate_field("request"));
                                }
                                r#request = Some(map_access.next_value()?);
                            }
                            Field::Collection => {
                                if r#collection.is_some() {
                                    return Err(serde::de::Error::duplicate_field("collection"));
                                }
                                r#collection = Some(map_access.next_value()?);
                            }
                            Field::Processing => {
                                if r#processing.is_some() {
                                    return Err(serde::de::Error::duplicate_field("processing"));
                                }
                                r#processing = Some(map_access.next_value()?);
                            }
                            Field::Container => {
                                if r#container.is_some() {
                                    return Err(serde::de::Error::duplicate_field("container"));
                                }
                                r#container = Some(map_access.next_value()?);
                            }
                            Field::Condition => {
                                if r#condition.is_some() {
                                    return Err(serde::de::Error::duplicate_field("condition"));
                                }
                                r#condition = Some(map_access.next_value()?);
                            }
                            Field::Note => {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => {
                                if config.mode == crate::json::de::DeserializationMode::Strict {
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
                                            "accessionIdentifier",
                                            "status",
                                            "type",
                                            "subject",
                                            "receivedTime",
                                            "parent",
                                            "request",
                                            "collection",
                                            "processing",
                                            "container",
                                            "condition",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(Specimen {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#accession_identifier,
                        r#status,
                        r#type,
                        r#subject,
                        r#received_time,
                        r#parent: r#parent.unwrap_or(vec![]),
                        r#request: r#request.unwrap_or(vec![]),
                        r#collection,
                        r#processing: r#processing.unwrap_or(vec![]),
                        r#container: r#container.unwrap_or(vec![]),
                        r#condition: r#condition.unwrap_or(vec![]),
                        r#note: r#note.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
