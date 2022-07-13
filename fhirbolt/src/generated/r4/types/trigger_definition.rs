// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum TriggerDefinitionTiming {
    Timing(Box<super::super::types::Timing>),
    Reference(Box<super::super::types::Reference>),
    Date(Box<super::super::types::Date>),
    DateTime(Box<super::super::types::DateTime>),
}
impl Default for TriggerDefinitionTiming {
    fn default() -> TriggerDefinitionTiming {
        unimplemented!()
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
            state.serialize_entry("type", some)?;
        }
        if self.r#type.id.is_some() || !self.r#type.extension.is_empty() {
            #[derive(serde :: Serialize)]
            struct PrimtiveElement<'a> {
                #[serde(skip_serializing_if = "Option::is_none")]
                id: &'a Option<std::string::String>,
                #[serde(skip_serializing_if = "<[_]>::is_empty")]
                extension: &'a [Box<super::super::types::Extension>],
            }
            let primitive_element = PrimtiveElement {
                id: &self.r#type.id,
                extension: &self.r#type.extension,
            };
            state.serialize_entry("_type", &primitive_element)?;
        }
        if let Some(some) = self.r#name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("name", some)?;
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
                        state.serialize_entry("timingDate", some)?;
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
                        state.serialize_entry("_timingDate", &primitive_element)?;
                    }
                }
                TriggerDefinitionTiming::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("timingDateTime", some)?;
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
                        state.serialize_entry("_timingDateTime", &primitive_element)?;
                    }
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
                        "type" => {
                            let some = r#type.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_type" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#type.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_type"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "name" => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_name" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#name.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "timingTiming" => {
                            if r#timing.is_some() {
                                return Err(serde::de::Error::duplicate_field("timingTiming"));
                            }
                            r#timing =
                                Some(TriggerDefinitionTiming::Timing(map_access.next_value()?));
                        }
                        "timingReference" => {
                            if r#timing.is_some() {
                                return Err(serde::de::Error::duplicate_field("timingReference"));
                            }
                            r#timing =
                                Some(TriggerDefinitionTiming::Reference(map_access.next_value()?));
                        }
                        "timingDate" => {
                            let r#enum = r#timing
                                .get_or_insert(TriggerDefinitionTiming::Date(Default::default()));
                            if let TriggerDefinitionTiming::Date(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("timingDate"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("timing[x]"));
                            }
                        }
                        "_timingDate" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let r#enum = r#timing
                                .get_or_insert(TriggerDefinitionTiming::Date(Default::default()));
                            if let TriggerDefinitionTiming::Date(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_timingDate"));
                                }
                                let PrimtiveElement { id, extension } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_timing[x]"));
                            }
                        }
                        "timingDateTime" => {
                            let r#enum = r#timing.get_or_insert(TriggerDefinitionTiming::DateTime(
                                Default::default(),
                            ));
                            if let TriggerDefinitionTiming::DateTime(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "timingDateTime",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("timing[x]"));
                            }
                        }
                        "_timingDateTime" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let r#enum = r#timing.get_or_insert(TriggerDefinitionTiming::DateTime(
                                Default::default(),
                            ));
                            if let TriggerDefinitionTiming::DateTime(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_timingDateTime",
                                    ));
                                }
                                let PrimtiveElement { id, extension } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_timing[x]"));
                            }
                        }
                        "data" => {
                            if r#data.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            r#data = Some(map_access.next_value()?);
                        }
                        "condition" => {
                            if r#condition.is_some() {
                                return Err(serde::de::Error::duplicate_field("condition"));
                            }
                            r#condition = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "type",
                                    "name",
                                    "timing",
                                    "data",
                                    "condition",
                                ],
                            ))
                        }
                    }
                }
                Ok(TriggerDefinition {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#name,
                    r#timing,
                    r#data: r#data.unwrap_or(vec![]),
                    r#condition,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
