// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for HumanName Type: A human's name with the ability to identify parts and usage.\n\nNeed to be able to record names, along with notes about their use."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct HumanName {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identifies the purpose for this name."]
    pub r#use: Option<super::super::types::Code>,
    #[doc = "Specifies the entire name as it should be displayed e.g. on an application UI. This may be provided instead of or as well as the specific parts."]
    pub r#text: Option<super::super::types::String>,
    #[doc = "The part of a name that links to the genealogy. In some cultures (e.g. Eritrea) the family name of a son is the first name of his father."]
    pub r#family: Option<super::super::types::String>,
    #[doc = "Given name."]
    pub r#given: Vec<super::super::types::String>,
    #[doc = "Part of the name that is acquired as a title due to academic, legal, employment or nobility status, etc. and that appears at the start of the name."]
    pub r#prefix: Vec<super::super::types::String>,
    #[doc = "Part of the name that is acquired as a title due to academic, legal, employment or nobility status, etc. and that appears at the end of the name."]
    pub r#suffix: Vec<super::super::types::String>,
    #[doc = "Indicates the period of time when this name was valid for the named person."]
    pub r#period: Option<Box<super::super::types::Period>>,
}
impl serde::ser::Serialize for HumanName {
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
                if let Some(some) = self.r#use.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("use", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_use", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#use.as_ref() {
                    state.serialize_entry("use", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#text.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("text", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_text", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#text.as_ref() {
                    state.serialize_entry("text", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#family.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("family", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_family", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#family.as_ref() {
                    state.serialize_entry("family", some)?;
                }
            }
            if _ctx.output_json {
                if !self.r#given.is_empty() {
                    let values = self
                        .r#given
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("given", &values)?;
                    }
                    let requires_elements = self
                        .r#given
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#given
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
                        state.serialize_entry("_given", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#given.is_empty() {
                    state.serialize_entry("given", &self.r#given)?;
                }
            }
            if _ctx.output_json {
                if !self.r#prefix.is_empty() {
                    let values = self
                        .r#prefix
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("prefix", &values)?;
                    }
                    let requires_elements = self
                        .r#prefix
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#prefix
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
                        state.serialize_entry("_prefix", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#prefix.is_empty() {
                    state.serialize_entry("prefix", &self.r#prefix)?;
                }
            }
            if _ctx.output_json {
                if !self.r#suffix.is_empty() {
                    let values = self
                        .r#suffix
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("suffix", &values)?;
                    }
                    let requires_elements = self
                        .r#suffix
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#suffix
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
                        state.serialize_entry("_suffix", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#suffix.is_empty() {
                    state.serialize_entry("suffix", &self.r#suffix)?;
                }
            }
            if let Some(some) = self.r#period.as_ref() {
                state.serialize_entry("period", some)?;
            }
            state.end()
        })
    }
}
