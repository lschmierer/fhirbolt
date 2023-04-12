// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for ParameterDefinition Type: The parameters to the module. This collection specifies both the input and output parameters. Input parameters are provided by the caller as part of the $evaluate operation. Output parameters are included in the GuidanceResponse."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ParameterDefinition {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The name of the parameter used to allow access to the value of the parameter in evaluation contexts."]
    pub r#name: Option<super::super::types::Code>,
    #[doc = "Whether the parameter is input or output for the module."]
    pub r#use: super::super::types::Code,
    #[doc = "The minimum number of times this parameter SHALL appear in the request or response."]
    pub r#min: Option<super::super::types::Integer>,
    #[doc = "The maximum number of times this element is permitted to appear in the request or response."]
    pub r#max: Option<super::super::types::String>,
    #[doc = "A brief discussion of what the parameter is for and how it is used by the module."]
    pub r#documentation: Option<super::super::types::String>,
    #[doc = "The type of the parameter."]
    pub r#type: super::super::types::Code,
    #[doc = "If specified, this indicates a profile that the input data must conform to, or that the output data will conform to."]
    pub r#profile: Option<super::super::types::Canonical>,
}
impl serde::ser::Serialize for ParameterDefinition {
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
                if let Some(some) = self.r#name.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("name", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_name", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#name.as_ref() {
                    state.serialize_entry("name", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#use.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("use", &some)?;
                }
                if self.r#use.id.is_some() || !self.r#use.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#use.id.as_ref(),
                        extension: &self.r#use.extension,
                    };
                    state.serialize_entry("_use", &primitive_element)?;
                }
            } else {
                state.serialize_entry("use", &self.r#use)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#min.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("min", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_min", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#min.as_ref() {
                    state.serialize_entry("min", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#max.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("max", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_max", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#max.as_ref() {
                    state.serialize_entry("max", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#documentation.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("documentation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_documentation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#documentation.as_ref() {
                    state.serialize_entry("documentation", some)?;
                }
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
                if let Some(some) = self.r#profile.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("profile", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_profile", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#profile.as_ref() {
                    state.serialize_entry("profile", some)?;
                }
            }
            state.end()
        })
    }
}
