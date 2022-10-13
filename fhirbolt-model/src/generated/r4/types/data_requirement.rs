// Generated on 2022-10-13 by fhirbolt-codegen v0.1.0
#[doc = "The intended subjects of the data requirement. If this element is not provided, a Patient subject is assumed."]
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
#[doc = "The value of the filter. If period is specified, the filter will return only those data items that fall within the bounds determined by the Period, inclusive of the period boundaries. If dateTime is specified, the filter will return only those data items that are equal to the specified dateTime. If a Duration is specified, the filter will return only those data items that fall within Duration before now."]
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
#[doc = "Code filters specify additional constraints on the data, specifying the value set of interest for a particular element of the data. Each code filter defines an additional constraint on the data, i.e. code filters are AND'ed, not OR'ed."]
#[derive(Default, Debug, Clone)]
pub struct DataRequirementCodeFilter {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The code-valued attribute of the filter. The specified path SHALL be a FHIRPath resolveable on the specified type of the DataRequirement, and SHALL consist only of identifiers, constant indexers, and .resolve(). The path is allowed to contain qualifiers (.) to traverse sub-elements, as well as indexers (\\[x\\]) to traverse multiple-cardinality sub-elements (see the [Simple FHIRPath Profile](fhirpath.html#simple) for full details). Note that the index must be an integer constant. The path must resolve to an element of type code, Coding, or CodeableConcept."]
    pub r#path: Option<super::super::types::String>,
    #[doc = "A token parameter that refers to a search parameter defined on the specified type of the DataRequirement, and which searches on elements of type code, Coding, or CodeableConcept."]
    pub r#search_param: Option<super::super::types::String>,
    #[doc = "The valueset for the code filter. The valueSet and code elements are additive. If valueSet is specified, the filter will return only those data items for which the value of the code-valued element specified in the path is a member of the specified valueset."]
    pub r#value_set: Option<super::super::types::Canonical>,
    #[doc = "The codes for the code filter. If values are given, the filter will return only those data items for which the code-valued attribute specified by the path has a value that is one of the specified codes. If codes are specified in addition to a value set, the filter returns items matching a code in the value set or one of the specified codes."]
    pub r#code: Vec<Box<super::super::types::Coding>>,
}
impl crate::AnyResource for DataRequirementCodeFilter {}
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
                let some = Ok(some)?;
                state.serialize_entry("path", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("searchParam", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("valueSet", &some)?;
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
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "path")]
            Path,
            #[serde(rename = "_path")]
            PathPrimitiveElement,
            #[serde(rename = "searchParam")]
            SearchParam,
            #[serde(rename = "_searchParam")]
            SearchParamPrimitiveElement,
            #[serde(rename = "valueSet")]
            ValueSet,
            #[serde(rename = "_valueSet")]
            ValueSetPrimitiveElement,
            #[serde(rename = "code")]
            Code,
            Unknown(String),
        }
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
                fhirbolt_shared::DESERIALIZATION_CONFIG.with(|config| {
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
                            Field::Path => {
                                let some = r#path.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("path"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::PathPrimitiveElement => {
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
                            Field::SearchParam => {
                                let some = r#search_param.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("searchParam"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::SearchParamPrimitiveElement => {
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
                            Field::ValueSet => {
                                let some = r#value_set.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueSet"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ValueSetPrimitiveElement => {
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
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "path",
                                            "searchParam",
                                            "valueSet",
                                            "code",
                                        ],
                                    ));
                                }
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Date filters specify additional constraints on the data in terms of the applicable date range for specific elements. Each date filter specifies an additional constraint on the data, i.e. date filters are AND'ed, not OR'ed."]
#[derive(Default, Debug, Clone)]
pub struct DataRequirementDateFilter {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The date-valued attribute of the filter. The specified path SHALL be a FHIRPath resolveable on the specified type of the DataRequirement, and SHALL consist only of identifiers, constant indexers, and .resolve(). The path is allowed to contain qualifiers (.) to traverse sub-elements, as well as indexers (\\[x\\]) to traverse multiple-cardinality sub-elements (see the [Simple FHIRPath Profile](fhirpath.html#simple) for full details). Note that the index must be an integer constant. The path must resolve to an element of type date, dateTime, Period, Schedule, or Timing."]
    pub r#path: Option<super::super::types::String>,
    #[doc = "A date parameter that refers to a search parameter defined on the specified type of the DataRequirement, and which searches on elements of type date, dateTime, Period, Schedule, or Timing."]
    pub r#search_param: Option<super::super::types::String>,
    #[doc = "The value of the filter. If period is specified, the filter will return only those data items that fall within the bounds determined by the Period, inclusive of the period boundaries. If dateTime is specified, the filter will return only those data items that are equal to the specified dateTime. If a Duration is specified, the filter will return only those data items that fall within Duration before now."]
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
                let some = Ok(some)?;
                state.serialize_entry("path", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("searchParam", &some)?;
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
                        let some = Ok(some)?;
                        state.serialize_entry("valueDateTime", &some)?;
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
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "path")]
            Path,
            #[serde(rename = "_path")]
            PathPrimitiveElement,
            #[serde(rename = "searchParam")]
            SearchParam,
            #[serde(rename = "_searchParam")]
            SearchParamPrimitiveElement,
            #[serde(rename = "valueDateTime")]
            ValueDateTime,
            #[serde(rename = "_valueDateTime")]
            ValueDateTimePrimitiveElement,
            #[serde(rename = "valuePeriod")]
            ValuePeriod,
            #[serde(rename = "valueDuration")]
            ValueDuration,
            Unknown(String),
        }
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
                fhirbolt_shared::DESERIALIZATION_CONFIG.with(|config| {
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
                            Field::Path => {
                                let some = r#path.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("path"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::PathPrimitiveElement => {
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
                            Field::SearchParam => {
                                let some = r#search_param.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("searchParam"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::SearchParamPrimitiveElement => {
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
                            Field::ValueDateTime => {
                                let r#enum = r#value.get_or_insert(
                                    DataRequirementDateFilterValue::DateTime(Default::default()),
                                );
                                if let DataRequirementDateFilterValue::DateTime(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueDateTime",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            }
                            Field::ValueDateTimePrimitiveElement => {
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
                            Field::ValuePeriod => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valuePeriod"));
                                }
                                r#value = Some(DataRequirementDateFilterValue::Period(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueDuration => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDuration"));
                                }
                                r#value = Some(DataRequirementDateFilterValue::Duration(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "path",
                                            "searchParam",
                                            "valueDateTime",
                                            "valuePeriod",
                                            "valueDuration",
                                        ],
                                    ));
                                }
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Specifies the order of the results to be returned."]
#[derive(Default, Debug, Clone)]
pub struct DataRequirementSort {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The attribute of the sort. The specified path must be resolvable from the type of the required data. The path is allowed to contain qualifiers (.) to traverse sub-elements, as well as indexers (\\[x\\]) to traverse multiple-cardinality sub-elements. Note that the index must be an integer constant."]
    pub r#path: super::super::types::String,
    #[doc = "The direction of the sort, ascending or descending."]
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
            let some = Ok(some)?;
            state.serialize_entry("path", &some)?;
        }
        if self.r#path.id.is_some() || !self.r#path.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#path.id,
                extension: &self.r#path.extension,
            };
            state.serialize_entry("_path", &primitive_element)?;
        }
        if let Some(some) = self.r#direction.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("direction", &some)?;
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
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "path")]
            Path,
            #[serde(rename = "_path")]
            PathPrimitiveElement,
            #[serde(rename = "direction")]
            Direction,
            #[serde(rename = "_direction")]
            DirectionPrimitiveElement,
            Unknown(String),
        }
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
                fhirbolt_shared::DESERIALIZATION_CONFIG.with(|config| {
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
                            Field::Path => {
                                let some = r#path.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("path"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::PathPrimitiveElement => {
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
                            Field::Direction => {
                                let some = r#direction.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("direction"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::DirectionPrimitiveElement => {
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
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &["id", "extension", "path", "direction"],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(DataRequirementSort {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#path: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#path.unwrap_or(Default::default())
                        } else {
                            r#path.ok_or(serde::de::Error::missing_field("path"))?
                        },
                        r#direction: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#direction.unwrap_or(Default::default())
                        } else {
                            r#direction.ok_or(serde::de::Error::missing_field("direction"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Base StructureDefinition for DataRequirement Type: Describes a required data item for evaluation in terms of the type of data, and optional code or date-based filters of the data."]
#[derive(Default, Debug, Clone)]
pub struct DataRequirement {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of the required data, specified as the type name of a resource. For profiles, this value is set to the type of the base resource of the profile."]
    pub r#type: super::super::types::Code,
    #[doc = "The profile of the required data, specified as the uri of the profile definition."]
    pub r#profile: Vec<super::super::types::Canonical>,
    #[doc = "The intended subjects of the data requirement. If this element is not provided, a Patient subject is assumed."]
    pub r#subject: Option<DataRequirementSubject>,
    #[doc = "Indicates that specific elements of the type are referenced by the knowledge module and must be supported by the consumer in order to obtain an effective evaluation. This does not mean that a value is required for this element, only that the consuming system must understand the element and be able to provide values for it if they are available. \n\nThe value of mustSupport SHALL be a FHIRPath resolveable on the type of the DataRequirement. The path SHALL consist only of identifiers, constant indexers, and .resolve() (see the [Simple FHIRPath Profile](fhirpath.html#simple) for full details)."]
    pub r#must_support: Vec<super::super::types::String>,
    #[doc = "Code filters specify additional constraints on the data, specifying the value set of interest for a particular element of the data. Each code filter defines an additional constraint on the data, i.e. code filters are AND'ed, not OR'ed."]
    pub r#code_filter: Vec<DataRequirementCodeFilter>,
    #[doc = "Date filters specify additional constraints on the data in terms of the applicable date range for specific elements. Each date filter specifies an additional constraint on the data, i.e. date filters are AND'ed, not OR'ed."]
    pub r#date_filter: Vec<DataRequirementDateFilter>,
    #[doc = "Specifies a maximum number of results that are required (uses the _count search parameter)."]
    pub r#limit: Option<super::super::types::PositiveInt>,
    #[doc = "Specifies the order of the results to be returned."]
    pub r#sort: Vec<DataRequirementSort>,
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
        if !self.r#profile.is_empty() {
            let values = self
                .r#profile
                .iter()
                .map(|v| &v.value)
                .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                .collect::<Result<Vec<_>, _>>()?;
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
            let values = self
                .r#must_support
                .iter()
                .map(|v| &v.value)
                .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                .collect::<Result<Vec<_>, _>>()?;
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
                let some = Ok(some)?;
                state.serialize_entry("limit", &some)?;
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
            #[serde(rename = "profile")]
            Profile,
            #[serde(rename = "_profile")]
            ProfilePrimitiveElement,
            #[serde(rename = "subjectCodeableConcept")]
            SubjectCodeableConcept,
            #[serde(rename = "subjectReference")]
            SubjectReference,
            #[serde(rename = "mustSupport")]
            MustSupport,
            #[serde(rename = "_mustSupport")]
            MustSupportPrimitiveElement,
            #[serde(rename = "codeFilter")]
            CodeFilter,
            #[serde(rename = "dateFilter")]
            DateFilter,
            #[serde(rename = "limit")]
            Limit,
            #[serde(rename = "_limit")]
            LimitPrimitiveElement,
            #[serde(rename = "sort")]
            Sort,
            Unknown(String),
        }
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
                let mut r#code_filter: Option<Vec<DataRequirementCodeFilter>> = None;
                let mut r#date_filter: Option<Vec<DataRequirementDateFilter>> = None;
                let mut r#limit: Option<super::super::types::PositiveInt> = None;
                let mut r#sort: Option<Vec<DataRequirementSort>> = None;
                fhirbolt_shared::DESERIALIZATION_CONFIG.with(|config| {
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
                            Field::Profile => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#profile.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("profile"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            }
                            Field::ProfilePrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
                                let vec = r#profile.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_profile"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            }
                            Field::SubjectCodeableConcept => {
                                if r#subject.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "subjectCodeableConcept",
                                    ));
                                }
                                r#subject = Some(DataRequirementSubject::CodeableConcept(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::SubjectReference => {
                                if r#subject.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "subjectReference",
                                    ));
                                }
                                r#subject = Some(DataRequirementSubject::Reference(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::MustSupport => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#must_support.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("mustSupport"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            }
                            Field::MustSupportPrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
                                let vec = r#must_support.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_mustSupport"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            }
                            Field::CodeFilter => {
                                if r#code_filter.is_some() {
                                    return Err(serde::de::Error::duplicate_field("codeFilter"));
                                }
                                r#code_filter = Some(map_access.next_value()?);
                            }
                            Field::DateFilter => {
                                if r#date_filter.is_some() {
                                    return Err(serde::de::Error::duplicate_field("dateFilter"));
                                }
                                r#date_filter = Some(map_access.next_value()?);
                            }
                            Field::Limit => {
                                let some = r#limit.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("limit"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::LimitPrimitiveElement => {
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
                            Field::Sort => {
                                if r#sort.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sort"));
                                }
                                r#sort = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "type",
                                            "profile",
                                            "subjectCodeableConcept",
                                            "subjectReference",
                                            "mustSupport",
                                            "codeFilter",
                                            "dateFilter",
                                            "limit",
                                            "sort",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(DataRequirement {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#type: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#profile: r#profile.unwrap_or(vec![]),
                        r#subject,
                        r#must_support: r#must_support.unwrap_or(vec![]),
                        r#code_filter: r#code_filter.unwrap_or(vec![]),
                        r#date_filter: r#date_filter.unwrap_or(vec![]),
                        r#limit,
                        r#sort: r#sort.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
