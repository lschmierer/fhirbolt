// Generated on 2022-07-27 by fhirbolt-codegen v0.1.0
#[doc = "Time when specimen was collected from subject - the physiologically relevant time."]
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
#[doc = "Abstinence or reduction from some or all food, drink, or both, for a period of time prior to sample collection."]
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
#[doc = "A record of the time or period when the specimen processing occurred.  For example the time of sample fixation or the period of time the sample was in formalin."]
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
#[doc = "Introduced substance to preserve, maintain or enhance the specimen. Examples: Formalin, Citrate, EDTA."]
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
#[doc = "Details concerning the specimen collection."]
#[derive(Default, Debug, Clone)]
pub struct SpecimenCollection {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Person who collected the specimen."]
    pub r#collector: Option<Box<super::super::types::Reference>>,
    #[doc = "Time when specimen was collected from subject - the physiologically relevant time."]
    pub r#collected: Option<SpecimenCollectionCollected>,
    #[doc = "The span of time over which the collection of a specimen occurred."]
    pub r#duration: Option<Box<super::super::types::Duration>>,
    #[doc = "The quantity of specimen collected; for instance the volume of a blood sample, or the physical measurement of an anatomic pathology sample."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "A coded value specifying the technique that is used to perform the procedure."]
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Anatomical location from which the specimen was collected (if subject is a patient). This is the target site.  This element is not used for environmental specimens."]
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Abstinence or reduction from some or all food, drink, or both, for a period of time prior to sample collection."]
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
#[doc = "Details concerning processing and processing steps for the specimen."]
#[derive(Default, Debug, Clone)]
pub struct SpecimenProcessing {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Textual description of procedure."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "A coded value specifying the procedure used to process the specimen."]
    pub r#procedure: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Material used in the processing step."]
    pub r#additive: Vec<Box<super::super::types::Reference>>,
    #[doc = "A record of the time or period when the specimen processing occurred.  For example the time of sample fixation or the period of time the sample was in formalin."]
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
#[doc = "The container holding the specimen.  The recursive nature of containers; i.e. blood in tube in tray in rack is not addressed here."]
#[derive(Default, Debug, Clone)]
pub struct SpecimenContainer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Id for container. There may be multiple; a manufacturer's bar code, lab assigned identifier, etc. The container ID may differ from the specimen id in some circumstances."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Textual description of the container."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "The type of container associated with the specimen (e.g. slide, aliquot, etc.)."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The capacity (volume or other measure) the container may contain."]
    pub r#capacity: Option<Box<super::super::types::Quantity>>,
    #[doc = "The quantity of specimen in the container; may be volume, dimensions, or other appropriate measurements, depending on the specimen type."]
    pub r#specimen_quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "Introduced substance to preserve, maintain or enhance the specimen. Examples: Formalin, Citrate, EDTA."]
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
#[doc = "A sample to be used for analysis."]
#[derive(Default, Debug, Clone)]
pub struct Specimen {
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
    #[doc = "Id for specimen."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The identifier assigned by the lab when accessioning specimen(s). This is not necessarily the same as the specimen identifier, depending on local lab procedures."]
    pub r#accession_identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "The availability of the specimen."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "The kind of material that forms the specimen."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Where the specimen came from. This may be from patient(s), from a location (e.g., the source of an environmental sample), or a sampling of a substance or a device."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "Time when specimen was received for processing or testing."]
    pub r#received_time: Option<super::super::types::DateTime>,
    #[doc = "Reference to the parent (source) specimen which is used when the specimen was either derived from or a component of another specimen."]
    pub r#parent: Vec<Box<super::super::types::Reference>>,
    #[doc = "Details concerning a service request that required a specimen to be collected."]
    pub r#request: Vec<Box<super::super::types::Reference>>,
    #[doc = "Details concerning the specimen collection."]
    pub r#collection: Option<SpecimenCollection>,
    #[doc = "Details concerning processing and processing steps for the specimen."]
    pub r#processing: Vec<SpecimenProcessing>,
    #[doc = "The container holding the specimen.  The recursive nature of containers; i.e. blood in tube in tray in rack is not addressed here."]
    pub r#container: Vec<SpecimenContainer>,
    #[doc = "A mode or state of being that describes the nature of the specimen."]
    pub r#condition: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "To communicate any details or issues about the specimen or during the specimen collection. (for example: broken vial, sent with patient, frozen)."]
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
