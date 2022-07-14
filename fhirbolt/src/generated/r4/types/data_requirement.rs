// Generated on 2022-07-14 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum DataRequirementSubject {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for DataRequirementSubject {
    fn default() -> DataRequirementSubject {
        DataRequirementSubject::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum DataRequirementDateFilterValue {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Duration(Box<super::super::types::Duration>),
    Invalid,
}
impl Default for DataRequirementDateFilterValue {
    fn default() -> DataRequirementDateFilterValue {
        DataRequirementDateFilterValue::Invalid
    }
}
#[derive(Default, Debug, Clone)]
pub struct DataRequirementCodeFilter {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#path: Option<super::super::types::String>,
    pub r#search_param: Option<super::super::types::String>,
    pub r#value_set: Option<super::super::types::Canonical>,
    pub r#code: Vec<Box<super::super::types::Coding>>,
}
impl serde::ser::Serialize for DataRequirementCodeFilter {
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
        if let Some(some) = self.r#path.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("path", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_path", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#search_param.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("searchParam", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_searchParam", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#value_set.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("valueSet", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_valueSet", &primitive_element)?;
            }
        }
        if !self.r#code.is_empty() {
            state.serialize_entry("code", &self.r#code)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for DataRequirementCodeFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DataRequirementCodeFilter;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DataRequirementCodeFilter")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DataRequirementCodeFilter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#path: Option<super::super::types::String> = None;
                let mut r#search_param: Option<super::super::types::String> = None;
                let mut r#value_set: Option<super::super::types::Canonical> = None;
                let mut r#code: Option<Vec<Box<super::super::types::Coding>>> = None;
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
                        "path" => {
                            let some = r#path.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_path" => {
                            let some = r#path.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_path"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "searchParam" => {
                            let some = r#search_param.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("searchParam"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_searchParam" => {
                            let some = r#search_param.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_searchParam"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "valueSet" => {
                            let some = r#value_set.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueSet"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_valueSet" => {
                            let some = r#value_set.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_valueSet"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "code" => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "path",
                                    "search_param",
                                    "value_set",
                                    "code",
                                ],
                            ))
                        }
                    }
                }
                Ok(DataRequirementCodeFilter {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#path,
                    r#search_param,
                    r#value_set,
                    r#code: r#code.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct DataRequirementDateFilter {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#path: Option<super::super::types::String>,
    pub r#search_param: Option<super::super::types::String>,
    pub r#value: Option<DataRequirementDateFilterValue>,
}
impl serde::ser::Serialize for DataRequirementDateFilter {
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
        if let Some(some) = self.r#path.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("path", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_path", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#search_param.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("searchParam", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_searchParam", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#value.as_ref() {
            match some {
                DataRequirementDateFilterValue::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueDateTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueDateTime", &primitive_element)?;
                    }
                }
                DataRequirementDateFilterValue::Period(ref value) => {
                    state.serialize_entry("valuePeriod", value)?;
                }
                DataRequirementDateFilterValue::Duration(ref value) => {
                    state.serialize_entry("valueDuration", value)?;
                }
                DataRequirementDateFilterValue::Invalid => {
                    return Err(serde::ser::Error::custom("value is invalid"))
                }
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for DataRequirementDateFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DataRequirementDateFilter;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DataRequirementDateFilter")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DataRequirementDateFilter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#path: Option<super::super::types::String> = None;
                let mut r#search_param: Option<super::super::types::String> = None;
                let mut r#value: Option<DataRequirementDateFilterValue> = None;
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
                        "path" => {
                            let some = r#path.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_path" => {
                            let some = r#path.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_path"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "searchParam" => {
                            let some = r#search_param.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("searchParam"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_searchParam" => {
                            let some = r#search_param.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_searchParam"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "valueDateTime" => {
                            let r#enum = r#value.get_or_insert(
                                DataRequirementDateFilterValue::DateTime(Default::default()),
                            );
                            if let DataRequirementDateFilterValue::DateTime(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDateTime"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueDateTime" => {
                            let r#enum = r#value.get_or_insert(
                                DataRequirementDateFilterValue::DateTime(Default::default()),
                            );
                            if let DataRequirementDateFilterValue::DateTime(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_valueDateTime",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valuePeriod" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valuePeriod"));
                            }
                            r#value = Some(DataRequirementDateFilterValue::Period(
                                map_access.next_value()?,
                            ));
                        }
                        "valueDuration" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDuration"));
                            }
                            r#value = Some(DataRequirementDateFilterValue::Duration(
                                map_access.next_value()?,
                            ));
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &["id", "extension", "path", "search_param", "value"],
                            ))
                        }
                    }
                }
                Ok(DataRequirementDateFilter {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#path,
                    r#search_param,
                    r#value,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct DataRequirementSort {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#path: super::super::types::String,
    pub r#direction: super::super::types::Code,
}
impl serde::ser::Serialize for DataRequirementSort {
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
        if let Some(some) = self.r#path.value.as_ref() {
            state.serialize_entry("path", some)?;
        }
        if self.r#path.id.is_some() || !self.r#path.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#path.id,
                extension: &self.r#path.extension,
            };
            state.serialize_entry("_path", &primitive_element)?;
        }
        if let Some(some) = self.r#direction.value.as_ref() {
            state.serialize_entry("direction", some)?;
        }
        if self.r#direction.id.is_some() || !self.r#direction.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#direction.id,
                extension: &self.r#direction.extension,
            };
            state.serialize_entry("_direction", &primitive_element)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for DataRequirementSort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DataRequirementSort;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DataRequirementSort")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DataRequirementSort, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#path: Option<super::super::types::String> = None;
                let mut r#direction: Option<super::super::types::Code> = None;
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
                        "path" => {
                            let some = r#path.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_path" => {
                            let some = r#path.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_path"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "direction" => {
                            let some = r#direction.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("direction"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_direction" => {
                            let some = r#direction.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_direction"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &["id", "extension", "path", "direction"],
                            ))
                        }
                    }
                }
                Ok(DataRequirementSort {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#path: r#path.ok_or(serde::de::Error::missing_field("path"))?,
                    r#direction: r#direction.ok_or(serde::de::Error::missing_field("direction"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct DataRequirement {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: super::super::types::Code,
    pub r#profile: Vec<super::super::types::Canonical>,
    pub r#subject: Option<DataRequirementSubject>,
    pub r#must_support: Vec<super::super::types::String>,
    pub r#code_filter: Vec<Box<super::super::types::Element>>,
    pub r#date_filter: Vec<Box<super::super::types::Element>>,
    pub r#limit: Option<super::super::types::PositiveInt>,
    pub r#sort: Vec<Box<super::super::types::Element>>,
}
impl serde::ser::Serialize for DataRequirement {
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
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#type.id,
                extension: &self.r#type.extension,
            };
            state.serialize_entry("_type", &primitive_element)?;
        }
        if !self.r#profile.is_empty() {
            let values: Vec<_> = self.r#profile.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("profile", &values)?;
            }
            let requires_elements = self
                .r#profile
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#profile
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
                state.serialize_entry("_profile", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#subject.as_ref() {
            match some {
                DataRequirementSubject::CodeableConcept(ref value) => {
                    state.serialize_entry("subjectCodeableConcept", value)?;
                }
                DataRequirementSubject::Reference(ref value) => {
                    state.serialize_entry("subjectReference", value)?;
                }
                DataRequirementSubject::Invalid => {
                    return Err(serde::ser::Error::custom("subject is invalid"))
                }
            }
        }
        if !self.r#must_support.is_empty() {
            let values: Vec<_> = self.r#must_support.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("mustSupport", &values)?;
            }
            let requires_elements = self
                .r#must_support
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#must_support
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
                state.serialize_entry("_mustSupport", &primitive_elements)?;
            }
        }
        if !self.r#code_filter.is_empty() {
            state.serialize_entry("codeFilter", &self.r#code_filter)?;
        }
        if !self.r#date_filter.is_empty() {
            state.serialize_entry("dateFilter", &self.r#date_filter)?;
        }
        if let Some(some) = self.r#limit.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("limit", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_limit", &primitive_element)?;
            }
        }
        if !self.r#sort.is_empty() {
            state.serialize_entry("sort", &self.r#sort)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for DataRequirement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DataRequirement;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DataRequirement")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DataRequirement, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#type: Option<super::super::types::Code> = None;
                let mut r#profile: Option<Vec<super::super::types::Canonical>> = None;
                let mut r#subject: Option<DataRequirementSubject> = None;
                let mut r#must_support: Option<Vec<super::super::types::String>> = None;
                let mut r#code_filter: Option<Vec<Box<super::super::types::Element>>> = None;
                let mut r#date_filter: Option<Vec<Box<super::super::types::Element>>> = None;
                let mut r#limit: Option<super::super::types::PositiveInt> = None;
                let mut r#sort: Option<Vec<Box<super::super::types::Element>>> = None;
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
                        "profile" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#profile.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("profile"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_profile" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#profile.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_profile"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "subjectCodeableConcept" => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "subjectCodeableConcept",
                                ));
                            }
                            r#subject = Some(DataRequirementSubject::CodeableConcept(
                                map_access.next_value()?,
                            ));
                        }
                        "subjectReference" => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectReference"));
                            }
                            r#subject =
                                Some(DataRequirementSubject::Reference(map_access.next_value()?));
                        }
                        "mustSupport" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec =
                                r#must_support.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("mustSupport"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_mustSupport" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#must_support.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_mustSupport"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "codeFilter" => {
                            if r#code_filter.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeFilter"));
                            }
                            r#code_filter = Some(map_access.next_value()?);
                        }
                        "dateFilter" => {
                            if r#date_filter.is_some() {
                                return Err(serde::de::Error::duplicate_field("dateFilter"));
                            }
                            r#date_filter = Some(map_access.next_value()?);
                        }
                        "limit" => {
                            let some = r#limit.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_limit" => {
                            let some = r#limit.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_limit"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "sort" => {
                            if r#sort.is_some() {
                                return Err(serde::de::Error::duplicate_field("sort"));
                            }
                            r#sort = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "type",
                                    "profile",
                                    "subject",
                                    "must_support",
                                    "code_filter",
                                    "date_filter",
                                    "limit",
                                    "sort",
                                ],
                            ))
                        }
                    }
                }
                Ok(DataRequirement {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#profile: r#profile.unwrap_or(vec![]),
                    r#subject,
                    r#must_support: r#must_support.unwrap_or(vec![]),
                    r#code_filter: r#code_filter.unwrap_or(vec![]),
                    r#date_filter: r#date_filter.unwrap_or(vec![]),
                    r#limit,
                    r#sort: r#sort.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
