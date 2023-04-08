// Generated on 2023-04-08 by fhirbolt-codegen v0.1.0
#[doc = "Time of product collection."]
#[derive(Debug, Clone, PartialEq)]
pub enum BiologicallyDerivedProductCollectionCollected {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for BiologicallyDerivedProductCollectionCollected {
    fn default() -> BiologicallyDerivedProductCollectionCollected {
        BiologicallyDerivedProductCollectionCollected::Invalid
    }
}
#[doc = "Time of processing."]
#[derive(Debug, Clone, PartialEq)]
pub enum BiologicallyDerivedProductProcessingTime {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for BiologicallyDerivedProductProcessingTime {
    fn default() -> BiologicallyDerivedProductProcessingTime {
        BiologicallyDerivedProductProcessingTime::Invalid
    }
}
#[doc = "Time of manipulation."]
#[derive(Debug, Clone, PartialEq)]
pub enum BiologicallyDerivedProductManipulationTime {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for BiologicallyDerivedProductManipulationTime {
    fn default() -> BiologicallyDerivedProductManipulationTime {
        BiologicallyDerivedProductManipulationTime::Invalid
    }
}
#[doc = "How this product was collected."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BiologicallyDerivedProductCollection {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Healthcare professional who is performing the collection."]
    pub r#collector: Option<Box<super::super::types::Reference>>,
    #[doc = "The patient or entity, such as a hospital or vendor in the case of a processed/manipulated/manufactured product, providing the product."]
    pub r#source: Option<Box<super::super::types::Reference>>,
    #[doc = "Time of product collection."]
    pub r#collected: Option<BiologicallyDerivedProductCollectionCollected>,
}
impl serde::ser::Serialize for BiologicallyDerivedProductCollection {
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
            if let Some(some) = self.r#collector.as_ref() {
                state.serialize_entry("collector", some)?;
            }
            if let Some(some) = self.r#source.as_ref() {
                state.serialize_entry("source", some)?;
            }
            if let Some(some) = self.r#collected.as_ref() {
                match some {
                    BiologicallyDerivedProductCollectionCollected::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("collectedDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_collectedDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("collectedDateTime", value)?;
                        }
                    }
                    BiologicallyDerivedProductCollectionCollected::Period(ref value) => {
                        state.serialize_entry("collectedPeriod", value)?;
                    }
                    BiologicallyDerivedProductCollectionCollected::Invalid => {
                        return Err(serde::ser::Error::custom("collected is invalid"))
                    }
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for BiologicallyDerivedProductCollection {
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
            #[serde(rename = "source")]
            Source,
            #[serde(rename = "collectedDateTime")]
            CollectedDateTime,
            #[serde(rename = "_collectedDateTime")]
            CollectedDateTimePrimitiveElement,
            #[serde(rename = "collectedPeriod")]
            CollectedPeriod,
            Unknown(std::string::String),
        }
        fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
            Err(E::unknown_field(
                field,
                &[
                    "id",
                    "extension",
                    "modifierExtension",
                    "collector",
                    "source",
                    "collectedDateTime",
                    "collectedPeriod",
                ],
            ))
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BiologicallyDerivedProductCollection;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("BiologicallyDerivedProductCollection")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<BiologicallyDerivedProductCollection, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#collector: Option<Box<super::super::types::Reference>> = None;
                let mut r#source: Option<Box<super::super::types::Reference>> = None;
                let mut r#collected: Option<BiologicallyDerivedProductCollectionCollected> = None;
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
                            Field::Collector => {
                                if r#collector.is_some() {
                                    return Err(serde::de::Error::duplicate_field("collector"));
                                }
                                r#collector = Some(map_access.next_value()?);
                            }
                            Field::Source => {
                                if r#source.is_some() {
                                    return Err(serde::de::Error::duplicate_field("source"));
                                }
                                r#source = Some(map_access.next_value()?);
                            }
                            Field::CollectedDateTime => {
                                if _ctx.from_json {
                                    let r#enum = r#collected.get_or_insert(
                                        BiologicallyDerivedProductCollectionCollected::DateTime(
                                            Default::default(),
                                        ),
                                    );
                                    if let BiologicallyDerivedProductCollectionCollected::DateTime(
                                        variant,
                                    ) = r#enum
                                    {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "collectedDateTime",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "collected[x]",
                                        ));
                                    }
                                } else {
                                    if r#collected.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "collectedDateTime",
                                        ));
                                    }
                                    r#collected = Some(
                                        BiologicallyDerivedProductCollectionCollected::DateTime(
                                            map_access.next_value()?,
                                        ),
                                    );
                                }
                            }
                            Field::CollectedDateTimePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#collected.get_or_insert(
                                        BiologicallyDerivedProductCollectionCollected::DateTime(
                                            Default::default(),
                                        ),
                                    );
                                    if let BiologicallyDerivedProductCollectionCollected::DateTime(
                                        variant,
                                    ) = r#enum
                                    {
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
                                        return Err(serde::de::Error::duplicate_field(
                                            "_collected[x]",
                                        ));
                                    }
                                } else {
                                    return unknown_field_error("collectedDateTime");
                                }
                            }
                            Field::CollectedPeriod => {
                                if r#collected.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "collectedPeriod",
                                    ));
                                }
                                r#collected =
                                    Some(BiologicallyDerivedProductCollectionCollected::Period(
                                        map_access.next_value()?,
                                    ));
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
                                        "collector",
                                        "source",
                                        "collectedDateTime",
                                        "collectedPeriod",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(BiologicallyDerivedProductCollection {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#collector,
                        r#source,
                        r#collected,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Any processing of the product during collection that does not change the fundamental nature of the product. For example adding anti-coagulants during the collection of Peripheral Blood Stem Cells."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BiologicallyDerivedProductProcessing {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Description of of processing."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Procesing code."]
    pub r#procedure: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Substance added during processing."]
    pub r#additive: Option<Box<super::super::types::Reference>>,
    #[doc = "Time of processing."]
    pub r#time: Option<BiologicallyDerivedProductProcessingTime>,
}
impl serde::ser::Serialize for BiologicallyDerivedProductProcessing {
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
            if let Some(some) = self.r#procedure.as_ref() {
                state.serialize_entry("procedure", some)?;
            }
            if let Some(some) = self.r#additive.as_ref() {
                state.serialize_entry("additive", some)?;
            }
            if let Some(some) = self.r#time.as_ref() {
                match some {
                    BiologicallyDerivedProductProcessingTime::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("timeDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_timeDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("timeDateTime", value)?;
                        }
                    }
                    BiologicallyDerivedProductProcessingTime::Period(ref value) => {
                        state.serialize_entry("timePeriod", value)?;
                    }
                    BiologicallyDerivedProductProcessingTime::Invalid => {
                        return Err(serde::ser::Error::custom("time is invalid"))
                    }
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for BiologicallyDerivedProductProcessing {
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
            Unknown(std::string::String),
        }
        fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
            Err(E::unknown_field(
                field,
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
            ))
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BiologicallyDerivedProductProcessing;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("BiologicallyDerivedProductProcessing")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<BiologicallyDerivedProductProcessing, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#procedure: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#additive: Option<Box<super::super::types::Reference>> = None;
                let mut r#time: Option<BiologicallyDerivedProductProcessingTime> = None;
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
                            Field::Description => {
                                if _ctx.from_json {
                                    let some = r#description.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "description",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#description.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "description",
                                        ));
                                    }
                                    r#description = Some(map_access.next_value()?);
                                }
                            }
                            Field::DescriptionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#description.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_description",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return unknown_field_error("description");
                                }
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
                                if _ctx.from_json {
                                    let r#enum = r#time.get_or_insert(
                                        BiologicallyDerivedProductProcessingTime::DateTime(
                                            Default::default(),
                                        ),
                                    );
                                    if let BiologicallyDerivedProductProcessingTime::DateTime(
                                        variant,
                                    ) = r#enum
                                    {
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
                                } else {
                                    if r#time.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "timeDateTime",
                                        ));
                                    }
                                    r#time =
                                        Some(BiologicallyDerivedProductProcessingTime::DateTime(
                                            map_access.next_value()?,
                                        ));
                                }
                            }
                            Field::TimeDateTimePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#time.get_or_insert(
                                        BiologicallyDerivedProductProcessingTime::DateTime(
                                            Default::default(),
                                        ),
                                    );
                                    if let BiologicallyDerivedProductProcessingTime::DateTime(
                                        variant,
                                    ) = r#enum
                                    {
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
                                } else {
                                    return unknown_field_error("timeDateTime");
                                }
                            }
                            Field::TimePeriod => {
                                if r#time.is_some() {
                                    return Err(serde::de::Error::duplicate_field("timePeriod"));
                                }
                                r#time = Some(BiologicallyDerivedProductProcessingTime::Period(
                                    map_access.next_value()?,
                                ));
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
                                        "description",
                                        "procedure",
                                        "additive",
                                        "timeDateTime",
                                        "timePeriod",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(BiologicallyDerivedProductProcessing {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#description,
                        r#procedure,
                        r#additive,
                        r#time,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Any manipulation of product post-collection that is intended to alter the product.  For example a buffy-coat enrichment or CD8 reduction of Peripheral Blood Stem Cells to make it more suitable for infusion."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BiologicallyDerivedProductManipulation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Description of manipulation."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Time of manipulation."]
    pub r#time: Option<BiologicallyDerivedProductManipulationTime>,
}
impl serde::ser::Serialize for BiologicallyDerivedProductManipulation {
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
            if let Some(some) = self.r#time.as_ref() {
                match some {
                    BiologicallyDerivedProductManipulationTime::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("timeDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_timeDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("timeDateTime", value)?;
                        }
                    }
                    BiologicallyDerivedProductManipulationTime::Period(ref value) => {
                        state.serialize_entry("timePeriod", value)?;
                    }
                    BiologicallyDerivedProductManipulationTime::Invalid => {
                        return Err(serde::ser::Error::custom("time is invalid"))
                    }
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for BiologicallyDerivedProductManipulation {
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
            #[serde(rename = "timeDateTime")]
            TimeDateTime,
            #[serde(rename = "_timeDateTime")]
            TimeDateTimePrimitiveElement,
            #[serde(rename = "timePeriod")]
            TimePeriod,
            Unknown(std::string::String),
        }
        fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
            Err(E::unknown_field(
                field,
                &[
                    "id",
                    "extension",
                    "modifierExtension",
                    "description",
                    "timeDateTime",
                    "timePeriod",
                ],
            ))
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BiologicallyDerivedProductManipulation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("BiologicallyDerivedProductManipulation")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<BiologicallyDerivedProductManipulation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#time: Option<BiologicallyDerivedProductManipulationTime> = None;
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
                            Field::Description => {
                                if _ctx.from_json {
                                    let some = r#description.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "description",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#description.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "description",
                                        ));
                                    }
                                    r#description = Some(map_access.next_value()?);
                                }
                            }
                            Field::DescriptionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#description.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_description",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return unknown_field_error("description");
                                }
                            }
                            Field::TimeDateTime => {
                                if _ctx.from_json {
                                    let r#enum = r#time.get_or_insert(
                                        BiologicallyDerivedProductManipulationTime::DateTime(
                                            Default::default(),
                                        ),
                                    );
                                    if let BiologicallyDerivedProductManipulationTime::DateTime(
                                        variant,
                                    ) = r#enum
                                    {
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
                                } else {
                                    if r#time.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "timeDateTime",
                                        ));
                                    }
                                    r#time =
                                        Some(BiologicallyDerivedProductManipulationTime::DateTime(
                                            map_access.next_value()?,
                                        ));
                                }
                            }
                            Field::TimeDateTimePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#time.get_or_insert(
                                        BiologicallyDerivedProductManipulationTime::DateTime(
                                            Default::default(),
                                        ),
                                    );
                                    if let BiologicallyDerivedProductManipulationTime::DateTime(
                                        variant,
                                    ) = r#enum
                                    {
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
                                } else {
                                    return unknown_field_error("timeDateTime");
                                }
                            }
                            Field::TimePeriod => {
                                if r#time.is_some() {
                                    return Err(serde::de::Error::duplicate_field("timePeriod"));
                                }
                                r#time = Some(BiologicallyDerivedProductManipulationTime::Period(
                                    map_access.next_value()?,
                                ));
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
                                        "description",
                                        "timeDateTime",
                                        "timePeriod",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(BiologicallyDerivedProductManipulation {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#description,
                        r#time,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Product storage."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BiologicallyDerivedProductStorage {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Description of storage."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Storage temperature."]
    pub r#temperature: Option<super::super::types::Decimal>,
    #[doc = "Temperature scale used."]
    pub r#scale: Option<super::super::types::Code>,
    #[doc = "Storage timeperiod."]
    pub r#duration: Option<Box<super::super::types::Period>>,
}
impl serde::ser::Serialize for BiologicallyDerivedProductStorage {
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
                if let Some(some) = self.r#temperature.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("temperature", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_temperature", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#temperature.as_ref() {
                    state.serialize_entry("temperature", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#scale.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("scale", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_scale", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#scale.as_ref() {
                    state.serialize_entry("scale", some)?;
                }
            }
            if let Some(some) = self.r#duration.as_ref() {
                state.serialize_entry("duration", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for BiologicallyDerivedProductStorage {
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
            #[serde(rename = "temperature")]
            Temperature,
            #[serde(rename = "_temperature")]
            TemperaturePrimitiveElement,
            #[serde(rename = "scale")]
            Scale,
            #[serde(rename = "_scale")]
            ScalePrimitiveElement,
            #[serde(rename = "duration")]
            Duration,
            Unknown(std::string::String),
        }
        fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
            Err(E::unknown_field(
                field,
                &[
                    "id",
                    "extension",
                    "modifierExtension",
                    "description",
                    "temperature",
                    "scale",
                    "duration",
                ],
            ))
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BiologicallyDerivedProductStorage;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("BiologicallyDerivedProductStorage")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<BiologicallyDerivedProductStorage, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#temperature: Option<super::super::types::Decimal> = None;
                let mut r#scale: Option<super::super::types::Code> = None;
                let mut r#duration: Option<Box<super::super::types::Period>> = None;
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
                            Field::Description => {
                                if _ctx.from_json {
                                    let some = r#description.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "description",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#description.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "description",
                                        ));
                                    }
                                    r#description = Some(map_access.next_value()?);
                                }
                            }
                            Field::DescriptionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#description.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_description",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return unknown_field_error("description");
                                }
                            }
                            Field::Temperature => {
                                if _ctx.from_json {
                                    let some = r#temperature.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "temperature",
                                        ));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    some.value = Some(format!("{}", value));
                                } else {
                                    if r#temperature.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "temperature",
                                        ));
                                    }
                                    r#temperature = Some(map_access.next_value()?);
                                }
                            }
                            Field::TemperaturePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#temperature.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_temperature",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return unknown_field_error("temperature");
                                }
                            }
                            Field::Scale => {
                                if _ctx.from_json {
                                    let some = r#scale.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("scale"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#scale.is_some() {
                                        return Err(serde::de::Error::duplicate_field("scale"));
                                    }
                                    r#scale = Some(map_access.next_value()?);
                                }
                            }
                            Field::ScalePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#scale.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_scale"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return unknown_field_error("scale");
                                }
                            }
                            Field::Duration => {
                                if r#duration.is_some() {
                                    return Err(serde::de::Error::duplicate_field("duration"));
                                }
                                r#duration = Some(map_access.next_value()?);
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
                                        "description",
                                        "temperature",
                                        "scale",
                                        "duration",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(BiologicallyDerivedProductStorage {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#description,
                        r#temperature,
                        r#scale,
                        r#duration,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A material substance originating from a biological entity intended to be transplanted or infused\ninto another (possibly the same) biological entity."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BiologicallyDerivedProduct {
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
    #[doc = "This records identifiers associated with this biologically derived product instance that are defined by business processes and/or used to refer to it when a direct URL reference to the resource itself is not appropriate (e.g. in CDA documents, or in written / printed documentation)."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Broad category of this product."]
    pub r#product_category: Option<super::super::types::Code>,
    #[doc = "A code that identifies the kind of this biologically derived product (SNOMED Ctcode)."]
    pub r#product_code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Whether the product is currently available."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "Procedure request to obtain this biologically derived product."]
    pub r#request: Vec<Box<super::super::types::Reference>>,
    #[doc = "Number of discrete units within this product."]
    pub r#quantity: Option<super::super::types::Integer>,
    #[doc = "Parent product (if any)."]
    pub r#parent: Vec<Box<super::super::types::Reference>>,
    #[doc = "How this product was collected."]
    pub r#collection: Option<BiologicallyDerivedProductCollection>,
    #[doc = "Any processing of the product during collection that does not change the fundamental nature of the product. For example adding anti-coagulants during the collection of Peripheral Blood Stem Cells."]
    pub r#processing: Vec<BiologicallyDerivedProductProcessing>,
    #[doc = "Any manipulation of product post-collection that is intended to alter the product.  For example a buffy-coat enrichment or CD8 reduction of Peripheral Blood Stem Cells to make it more suitable for infusion."]
    pub r#manipulation: Option<BiologicallyDerivedProductManipulation>,
    #[doc = "Product storage."]
    pub r#storage: Vec<BiologicallyDerivedProductStorage>,
}
impl crate::AnyResource for BiologicallyDerivedProduct {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4B;
}
impl serde::ser::Serialize for BiologicallyDerivedProduct {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "BiologicallyDerivedProduct")?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#product_category.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("productCategory", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_productCategory", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#product_category.as_ref() {
                    state.serialize_entry("productCategory", some)?;
                }
            }
            if let Some(some) = self.r#product_code.as_ref() {
                state.serialize_entry("productCode", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#status.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("status", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_status", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#status.as_ref() {
                    state.serialize_entry("status", some)?;
                }
            }
            if !self.r#request.is_empty() {
                state.serialize_entry("request", &self.r#request)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#quantity.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("quantity", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_quantity", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#quantity.as_ref() {
                    state.serialize_entry("quantity", some)?;
                }
            }
            if !self.r#parent.is_empty() {
                state.serialize_entry("parent", &self.r#parent)?;
            }
            if let Some(some) = self.r#collection.as_ref() {
                state.serialize_entry("collection", some)?;
            }
            if !self.r#processing.is_empty() {
                state.serialize_entry("processing", &self.r#processing)?;
            }
            if let Some(some) = self.r#manipulation.as_ref() {
                state.serialize_entry("manipulation", some)?;
            }
            if !self.r#storage.is_empty() {
                state.serialize_entry("storage", &self.r#storage)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for BiologicallyDerivedProduct {
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
            #[serde(rename = "productCategory")]
            ProductCategory,
            #[serde(rename = "_productCategory")]
            ProductCategoryPrimitiveElement,
            #[serde(rename = "productCode")]
            ProductCode,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "request")]
            Request,
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "_quantity")]
            QuantityPrimitiveElement,
            #[serde(rename = "parent")]
            Parent,
            #[serde(rename = "collection")]
            Collection,
            #[serde(rename = "processing")]
            Processing,
            #[serde(rename = "manipulation")]
            Manipulation,
            #[serde(rename = "storage")]
            Storage,
            Unknown(std::string::String),
        }
        fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
            Err(E::unknown_field(
                field,
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
                    "productCategory",
                    "productCode",
                    "status",
                    "request",
                    "quantity",
                    "parent",
                    "collection",
                    "processing",
                    "manipulation",
                    "storage",
                ],
            ))
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BiologicallyDerivedProduct;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("BiologicallyDerivedProduct")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<BiologicallyDerivedProduct, V::Error>
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
                let mut r#product_category: Option<super::super::types::Code> = None;
                let mut r#product_code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#request: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#quantity: Option<super::super::types::Integer> = None;
                let mut r#parent: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#collection: Option<BiologicallyDerivedProductCollection> = None;
                let mut r#processing: Option<Vec<BiologicallyDerivedProductProcessing>> = None;
                let mut r#manipulation: Option<BiologicallyDerivedProductManipulation> = None;
                let mut r#storage: Option<Vec<BiologicallyDerivedProductStorage>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "BiologicallyDerivedProduct" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"BiologicallyDerivedProduct",
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
                                    return unknown_field_error("implicitRules");
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
                                    return unknown_field_error("language");
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
                            Field::ProductCategory => {
                                if _ctx.from_json {
                                    let some = r#product_category.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "productCategory",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#product_category.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "productCategory",
                                        ));
                                    }
                                    r#product_category = Some(map_access.next_value()?);
                                }
                            }
                            Field::ProductCategoryPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#product_category.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_productCategory",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return unknown_field_error("productCategory");
                                }
                            }
                            Field::ProductCode => {
                                if r#product_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("productCode"));
                                }
                                r#product_code = Some(map_access.next_value()?);
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
                                    return unknown_field_error("status");
                                }
                            }
                            Field::Request => {
                                if _ctx.from_json {
                                    if r#request.is_some() {
                                        return Err(serde::de::Error::duplicate_field("request"));
                                    }
                                    r#request = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#request.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Quantity => {
                                if _ctx.from_json {
                                    let some = r#quantity.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("quantity"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#quantity.is_some() {
                                        return Err(serde::de::Error::duplicate_field("quantity"));
                                    }
                                    r#quantity = Some(map_access.next_value()?);
                                }
                            }
                            Field::QuantityPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#quantity.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_quantity"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return unknown_field_error("quantity");
                                }
                            }
                            Field::Parent => {
                                if _ctx.from_json {
                                    if r#parent.is_some() {
                                        return Err(serde::de::Error::duplicate_field("parent"));
                                    }
                                    r#parent = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#parent.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Collection => {
                                if r#collection.is_some() {
                                    return Err(serde::de::Error::duplicate_field("collection"));
                                }
                                r#collection = Some(map_access.next_value()?);
                            }
                            Field::Processing => {
                                if _ctx.from_json {
                                    if r#processing.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "processing",
                                        ));
                                    }
                                    r#processing = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#processing.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Manipulation => {
                                if r#manipulation.is_some() {
                                    return Err(serde::de::Error::duplicate_field("manipulation"));
                                }
                                r#manipulation = Some(map_access.next_value()?);
                            }
                            Field::Storage => {
                                if _ctx.from_json {
                                    if r#storage.is_some() {
                                        return Err(serde::de::Error::duplicate_field("storage"));
                                    }
                                    r#storage = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#storage.get_or_insert(Default::default());
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
                                        "productCategory",
                                        "productCode",
                                        "status",
                                        "request",
                                        "quantity",
                                        "parent",
                                        "collection",
                                        "processing",
                                        "manipulation",
                                        "storage",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(BiologicallyDerivedProduct {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#product_category,
                        r#product_code,
                        r#status,
                        r#request: r#request.unwrap_or(vec![]),
                        r#quantity,
                        r#parent: r#parent.unwrap_or(vec![]),
                        r#collection,
                        r#processing: r#processing.unwrap_or(vec![]),
                        r#manipulation,
                        r#storage: r#storage.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
