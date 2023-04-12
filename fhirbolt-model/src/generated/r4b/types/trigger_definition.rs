// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "The timing of the event (if this is a periodic trigger)."]
#[derive(Debug, Clone, PartialEq)]
pub enum TriggerDefinitionTiming {
    Timing(Box<super::super::types::Timing>),
    Reference(Box<super::super::types::Reference>),
    Date(Box<super::super::types::Date>),
    DateTime(Box<super::super::types::DateTime>),
    Invalid,
}
impl Default for TriggerDefinitionTiming {
    fn default() -> TriggerDefinitionTiming {
        TriggerDefinitionTiming::Invalid
    }
}
#[doc = "Base StructureDefinition for TriggerDefinition Type: A description of a triggering event. Triggering events can be named events, data events, or periodic, as determined by the type element."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TriggerDefinition {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of triggering event."]
    pub r#type: super::super::types::Code,
    #[doc = "A formal name for the event. This may be an absolute URI that identifies the event formally (e.g. from a trigger registry), or a simple relative URI that identifies the event in a local context."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "The timing of the event (if this is a periodic trigger)."]
    pub r#timing: Option<TriggerDefinitionTiming>,
    #[doc = "The triggering data of the event (if this is a data trigger). If more than one data is requirement is specified, then all the data requirements must be true."]
    pub r#data: Vec<Box<super::super::types::DataRequirement>>,
    #[doc = "A boolean-valued expression that is evaluated in the context of the container of the trigger definition and returns whether or not the trigger fires."]
    pub r#condition: Option<Box<super::super::types::Expression>>,
}
impl serde::ser::Serialize for TriggerDefinition {
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
            if let Some(some) = self.r#timing.as_ref() {
                match some {
                    TriggerDefinitionTiming::Timing(ref value) => {
                        state.serialize_entry("timingTiming", value)?;
                    }
                    TriggerDefinitionTiming::Reference(ref value) => {
                        state.serialize_entry("timingReference", value)?;
                    }
                    TriggerDefinitionTiming::Date(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("timingDate", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_timingDate", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("timingDate", value)?;
                        }
                    }
                    TriggerDefinitionTiming::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("timingDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_timingDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("timingDateTime", value)?;
                        }
                    }
                    TriggerDefinitionTiming::Invalid => {
                        return Err(serde::ser::Error::custom("timing is invalid"))
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
        })
    }
}
