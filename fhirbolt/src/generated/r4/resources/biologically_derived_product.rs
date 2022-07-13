// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum BiologicallyDerivedProductCollectionCollected {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
}
impl Default for BiologicallyDerivedProductCollectionCollected {
    fn default() -> BiologicallyDerivedProductCollectionCollected {
        unimplemented!()
    }
}
#[derive(Debug, Clone)]
pub enum BiologicallyDerivedProductProcessingTime {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
}
impl Default for BiologicallyDerivedProductProcessingTime {
    fn default() -> BiologicallyDerivedProductProcessingTime {
        unimplemented!()
    }
}
#[derive(Debug, Clone)]
pub enum BiologicallyDerivedProductManipulationTime {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
}
impl Default for BiologicallyDerivedProductManipulationTime {
    fn default() -> BiologicallyDerivedProductManipulationTime {
        unimplemented!()
    }
}
#[derive(Default, Debug, Clone)]
pub struct BiologicallyDerivedProductCollection {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#collector: Option<Box<super::super::types::Reference>>,
    pub r#source: Option<Box<super::super::types::Reference>>,
    pub r#collected: Option<BiologicallyDerivedProductCollectionCollected>,
}
impl serde::ser::Serialize for BiologicallyDerivedProductCollection {
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
        if let Some(some) = self.r#source.as_ref() {
            state.serialize_entry("source", some)?;
        }
        if let Some(some) = self.r#collected.as_ref() {
            match some {
                BiologicallyDerivedProductCollectionCollected::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("collectedDateTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_collectedDateTime", &primitive_element)?;
                    }
                }
                BiologicallyDerivedProductCollectionCollected::Period(ref value) => {
                    state.serialize_entry("collectedPeriod", value)?;
                }
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for BiologicallyDerivedProductCollection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
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
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "extension" => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        "modifierExtension" => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        "collector" => {
                            if r#collector.is_some() {
                                return Err(serde::de::Error::duplicate_field("collector"));
                            }
                            r#collector = Some(map_access.next_value()?);
                        }
                        "source" => {
                            if r#source.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            r#source = Some(map_access.next_value()?);
                        }
                        "collectedDateTime" => {
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
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("collected[x]"));
                            }
                        }
                        "_collectedDateTime" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
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
                                let PrimtiveElement { id, extension } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_collected[x]"));
                            }
                        }
                        "collectedPeriod" => {
                            if r#collected.is_some() {
                                return Err(serde::de::Error::duplicate_field("collectedPeriod"));
                            }
                            r#collected =
                                Some(BiologicallyDerivedProductCollectionCollected::Period(
                                    map_access.next_value()?,
                                ));
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "collector",
                                    "source",
                                    "collected",
                                ],
                            ))
                        }
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct BiologicallyDerivedProductProcessing {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Option<super::super::types::String>,
    pub r#procedure: Option<Box<super::super::types::CodeableConcept>>,
    pub r#additive: Option<Box<super::super::types::Reference>>,
    pub r#time: Option<BiologicallyDerivedProductProcessingTime>,
}
impl serde::ser::Serialize for BiologicallyDerivedProductProcessing {
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
                state.serialize_entry("description", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_description", &primitive_element)?;
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
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("timeDateTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_timeDateTime", &primitive_element)?;
                    }
                }
                BiologicallyDerivedProductProcessingTime::Period(ref value) => {
                    state.serialize_entry("timePeriod", value)?;
                }
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for BiologicallyDerivedProductProcessing {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
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
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "extension" => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        "modifierExtension" => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        "description" => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_description" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#description.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_description"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "procedure" => {
                            if r#procedure.is_some() {
                                return Err(serde::de::Error::duplicate_field("procedure"));
                            }
                            r#procedure = Some(map_access.next_value()?);
                        }
                        "additive" => {
                            if r#additive.is_some() {
                                return Err(serde::de::Error::duplicate_field("additive"));
                            }
                            r#additive = Some(map_access.next_value()?);
                        }
                        "timeDateTime" => {
                            let r#enum = r#time.get_or_insert(
                                BiologicallyDerivedProductProcessingTime::DateTime(
                                    Default::default(),
                                ),
                            );
                            if let BiologicallyDerivedProductProcessingTime::DateTime(variant) =
                                r#enum
                            {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("timeDateTime"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("time[x]"));
                            }
                        }
                        "_timeDateTime" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let r#enum = r#time.get_or_insert(
                                BiologicallyDerivedProductProcessingTime::DateTime(
                                    Default::default(),
                                ),
                            );
                            if let BiologicallyDerivedProductProcessingTime::DateTime(variant) =
                                r#enum
                            {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_timeDateTime"));
                                }
                                let PrimtiveElement { id, extension } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_time[x]"));
                            }
                        }
                        "timePeriod" => {
                            if r#time.is_some() {
                                return Err(serde::de::Error::duplicate_field("timePeriod"));
                            }
                            r#time = Some(BiologicallyDerivedProductProcessingTime::Period(
                                map_access.next_value()?,
                            ));
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "description",
                                    "procedure",
                                    "additive",
                                    "time",
                                ],
                            ))
                        }
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct BiologicallyDerivedProductManipulation {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Option<super::super::types::String>,
    pub r#time: Option<BiologicallyDerivedProductManipulationTime>,
}
impl serde::ser::Serialize for BiologicallyDerivedProductManipulation {
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
                state.serialize_entry("description", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_description", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#time.as_ref() {
            match some {
                BiologicallyDerivedProductManipulationTime::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("timeDateTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_timeDateTime", &primitive_element)?;
                    }
                }
                BiologicallyDerivedProductManipulationTime::Period(ref value) => {
                    state.serialize_entry("timePeriod", value)?;
                }
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for BiologicallyDerivedProductManipulation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
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
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "extension" => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        "modifierExtension" => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        "description" => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_description" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#description.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_description"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "timeDateTime" => {
                            let r#enum = r#time.get_or_insert(
                                BiologicallyDerivedProductManipulationTime::DateTime(
                                    Default::default(),
                                ),
                            );
                            if let BiologicallyDerivedProductManipulationTime::DateTime(variant) =
                                r#enum
                            {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("timeDateTime"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("time[x]"));
                            }
                        }
                        "_timeDateTime" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let r#enum = r#time.get_or_insert(
                                BiologicallyDerivedProductManipulationTime::DateTime(
                                    Default::default(),
                                ),
                            );
                            if let BiologicallyDerivedProductManipulationTime::DateTime(variant) =
                                r#enum
                            {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_timeDateTime"));
                                }
                                let PrimtiveElement { id, extension } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_time[x]"));
                            }
                        }
                        "timePeriod" => {
                            if r#time.is_some() {
                                return Err(serde::de::Error::duplicate_field("timePeriod"));
                            }
                            r#time = Some(BiologicallyDerivedProductManipulationTime::Period(
                                map_access.next_value()?,
                            ));
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "description",
                                    "time",
                                ],
                            ))
                        }
                    }
                }
                Ok(BiologicallyDerivedProductManipulation {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#description,
                    r#time,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct BiologicallyDerivedProductStorage {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Option<super::super::types::String>,
    pub r#temperature: Option<super::super::types::Decimal>,
    pub r#scale: Option<super::super::types::Code>,
    pub r#duration: Option<Box<super::super::types::Period>>,
}
impl serde::ser::Serialize for BiologicallyDerivedProductStorage {
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
                state.serialize_entry("description", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_description", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#temperature.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("temperature", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_temperature", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#scale.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("scale", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_scale", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#duration.as_ref() {
            state.serialize_entry("duration", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for BiologicallyDerivedProductStorage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
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
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "extension" => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        "modifierExtension" => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        "description" => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_description" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#description.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_description"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "temperature" => {
                            let some = r#temperature.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("temperature"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_temperature" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#temperature.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_temperature"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "scale" => {
                            let some = r#scale.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("scale"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_scale" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#scale.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_scale"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "duration" => {
                            if r#duration.is_some() {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            r#duration = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "description",
                                    "temperature",
                                    "scale",
                                    "duration",
                                ],
                            ))
                        }
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct BiologicallyDerivedProduct {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#product_category: Option<super::super::types::Code>,
    pub r#product_code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#status: Option<super::super::types::Code>,
    pub r#request: Vec<Box<super::super::types::Reference>>,
    pub r#quantity: Option<super::super::types::Integer>,
    pub r#parent: Vec<Box<super::super::types::Reference>>,
    pub r#collection: Option<BiologicallyDerivedProductCollection>,
    pub r#processing: Vec<BiologicallyDerivedProductProcessing>,
    pub r#manipulation: Option<BiologicallyDerivedProductManipulation>,
    pub r#storage: Vec<BiologicallyDerivedProductStorage>,
}
impl serde::ser::Serialize for BiologicallyDerivedProduct {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "BiologicallyDerivedProduct")?;
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
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
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
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
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
        if let Some(some) = self.r#product_category.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("productCategory", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_productCategory", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#product_code.as_ref() {
            state.serialize_entry("productCode", some)?;
        }
        if let Some(some) = self.r#status.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("status", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_status", &primitive_element)?;
            }
        }
        if !self.r#request.is_empty() {
            state.serialize_entry("request", &self.r#request)?;
        }
        if let Some(some) = self.r#quantity.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("quantity", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_quantity", &primitive_element)?;
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
    }
}
impl<'de> serde::de::Deserialize<'de> for BiologicallyDerivedProduct {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
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
                let mut r#contained: Option<Vec<Box<super::Resource>>> = None;
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
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "meta" => {
                            if r#meta.is_some() {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            r#meta = Some(map_access.next_value()?);
                        }
                        "implicitRules" => {
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("implicitRules"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_implicitRules" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_implicitRules"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "language" => {
                            let some = r#language.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_language" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#language.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_language"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "text" => {
                            if r#text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            r#text = Some(map_access.next_value()?);
                        }
                        "contained" => {
                            if r#contained.is_some() {
                                return Err(serde::de::Error::duplicate_field("contained"));
                            }
                            r#contained = Some(map_access.next_value()?);
                        }
                        "extension" => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        "modifierExtension" => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        "identifier" => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        "productCategory" => {
                            let some = r#product_category.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("productCategory"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_productCategory" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#product_category.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_productCategory"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "productCode" => {
                            if r#product_code.is_some() {
                                return Err(serde::de::Error::duplicate_field("productCode"));
                            }
                            r#product_code = Some(map_access.next_value()?);
                        }
                        "status" => {
                            let some = r#status.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_status" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#status.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_status"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "request" => {
                            if r#request.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            r#request = Some(map_access.next_value()?);
                        }
                        "quantity" => {
                            let some = r#quantity.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_quantity" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#quantity.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_quantity"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "parent" => {
                            if r#parent.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            r#parent = Some(map_access.next_value()?);
                        }
                        "collection" => {
                            if r#collection.is_some() {
                                return Err(serde::de::Error::duplicate_field("collection"));
                            }
                            r#collection = Some(map_access.next_value()?);
                        }
                        "processing" => {
                            if r#processing.is_some() {
                                return Err(serde::de::Error::duplicate_field("processing"));
                            }
                            r#processing = Some(map_access.next_value()?);
                        }
                        "manipulation" => {
                            if r#manipulation.is_some() {
                                return Err(serde::de::Error::duplicate_field("manipulation"));
                            }
                            r#manipulation = Some(map_access.next_value()?);
                        }
                        "storage" => {
                            if r#storage.is_some() {
                                return Err(serde::de::Error::duplicate_field("storage"));
                            }
                            r#storage = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "meta",
                                    "implicit_rules",
                                    "language",
                                    "text",
                                    "contained",
                                    "extension",
                                    "modifier_extension",
                                    "identifier",
                                    "product_category",
                                    "product_code",
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
