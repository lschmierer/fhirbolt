// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for Identifier Type: An identifier - identifies some entity uniquely and unambiguously. Typically this is used for business identifiers.\n\nNeed to be able to identify things with confidence and be sure that the identification is not subject to misinterpretation."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Identifier {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The purpose of this identifier."]
    pub r#use: Option<super::super::types::Code>,
    #[doc = "A coded type for the identifier that can be used to determine which identifier to use for a specific purpose."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Establishes the namespace for the value - that is, a URL that describes a set values that are unique."]
    pub r#system: Option<super::super::types::Uri>,
    #[doc = "The portion of the identifier typically relevant to the user and which is unique within the context of the system."]
    pub r#value: Option<super::super::types::String>,
    #[doc = "Time period during which identifier is/was valid for use."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "Organization that issued/manages the identifier."]
    pub r#assigner: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for Identifier {
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
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#system.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("system", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_system", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#system.as_ref() {
                    state.serialize_entry("system", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#value.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("value", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_value", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#value.as_ref() {
                    state.serialize_entry("value", some)?;
                }
            }
            if let Some(some) = self.r#period.as_ref() {
                state.serialize_entry("period", some)?;
            }
            if let Some(some) = self.r#assigner.as_ref() {
                state.serialize_entry("assigner", some)?;
            }
            state.end()
        })
    }
}
