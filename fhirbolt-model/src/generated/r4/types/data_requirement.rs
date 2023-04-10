// Generated on 2023-04-10 by fhirbolt-codegen v0.1.0
#[doc = "The intended subjects of the data requirement. If this element is not provided, a Patient subject is assumed."]
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Default, Debug, Clone, PartialEq)]
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
impl serde::ser::Serialize for DataRequirementCodeFilter {
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
            if _ctx.output_json {
                if let Some(some) = self.r#path.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("path", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_path", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#path.as_ref() {
                    state.serialize_entry("path", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#search_param.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("searchParam", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_searchParam", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#search_param.as_ref() {
                    state.serialize_entry("searchParam", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#value_set.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueSet", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_valueSet", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#value_set.as_ref() {
                    state.serialize_entry("valueSet", some)?;
                }
            }
            if !self.r#code.is_empty() {
                state.serialize_entry("code", &self.r#code)?;
            }
            state.end()
        })
    }
}
#[doc = "Date filters specify additional constraints on the data in terms of the applicable date range for specific elements. Each date filter specifies an additional constraint on the data, i.e. date filters are AND'ed, not OR'ed."]
#[derive(Default, Debug, Clone, PartialEq)]
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
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#path.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("path", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_path", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#path.as_ref() {
                    state.serialize_entry("path", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#search_param.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("searchParam", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_searchParam", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#search_param.as_ref() {
                    state.serialize_entry("searchParam", some)?;
                }
            }
            if let Some(some) = self.r#value.as_ref() {
                match some {
                    DataRequirementDateFilterValue::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valueDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueDateTime", value)?;
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
        })
    }
}
#[doc = "Specifies the order of the results to be returned."]
#[derive(Default, Debug, Clone, PartialEq)]
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
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#path.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("path", &some)?;
                }
                if self.r#path.id.is_some() || !self.r#path.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#path.id.as_ref(),
                        extension: &self.r#path.extension,
                    };
                    state.serialize_entry("_path", &primitive_element)?;
                }
            } else {
                state.serialize_entry("path", &self.r#path)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#direction.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("direction", &some)?;
                }
                if self.r#direction.id.is_some() || !self.r#direction.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#direction.id.as_ref(),
                        extension: &self.r#direction.extension,
                    };
                    state.serialize_entry("_direction", &primitive_element)?;
                }
            } else {
                state.serialize_entry("direction", &self.r#direction)?;
            }
            state.end()
        })
    }
}
#[doc = "Base StructureDefinition for DataRequirement Type: Describes a required data item for evaluation in terms of the type of data, and optional code or date-based filters of the data."]
#[derive(Default, Debug, Clone, PartialEq)]
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
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#type.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("type", &some)?;
                }
                if self.r#type.id.is_some() || !self.r#type.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#type.id.as_ref(),
                        extension: &self.r#type.extension,
                    };
                    state.serialize_entry("_type", &primitive_element)?;
                }
            } else {
                state.serialize_entry("type", &self.r#type)?;
            }
            if _ctx.output_json {
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#profile.is_empty() {
                    state.serialize_entry("profile", &self.r#profile)?;
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
            if _ctx.output_json {
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#must_support.is_empty() {
                    state.serialize_entry("mustSupport", &self.r#must_support)?;
                }
            }
            if !self.r#code_filter.is_empty() {
                state.serialize_entry("codeFilter", &self.r#code_filter)?;
            }
            if !self.r#date_filter.is_empty() {
                state.serialize_entry("dateFilter", &self.r#date_filter)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#limit.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("limit", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_limit", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#limit.as_ref() {
                    state.serialize_entry("limit", some)?;
                }
            }
            if !self.r#sort.is_empty() {
                state.serialize_entry("sort", &self.r#sort)?;
            }
            state.end()
        })
    }
}
