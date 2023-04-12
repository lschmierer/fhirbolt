// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for Attachment Type: For referring to data content defined in other formats.\n\nMany models need to include data defined in other specifications that is complex and opaque to the healthcare model. This includes documents, media recordings, structured data, etc."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Attachment {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identifies the type of the data in the attachment and allows a method to be chosen to interpret or render the data. Includes mime type parameters such as charset where appropriate."]
    pub r#content_type: Option<super::super::types::Code>,
    #[doc = "The human language of the content. The value can be any valid value according to BCP 47."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "The actual data of the attachment - a sequence of bytes, base64 encoded."]
    pub r#data: Option<super::super::types::Base64Binary>,
    #[doc = "A location where the data can be accessed."]
    pub r#url: Option<super::super::types::Url>,
    #[doc = "The number of bytes of data that make up this attachment (before base64 encoding, if that is done)."]
    pub r#size: Option<super::super::types::UnsignedInt>,
    #[doc = "The calculated hash of the data using SHA-1. Represented using base64."]
    pub r#hash: Option<super::super::types::Base64Binary>,
    #[doc = "A label or set of text to display in place of the data."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The date that the attachment was first created."]
    pub r#creation: Option<super::super::types::DateTime>,
}
impl serde::ser::Serialize for Attachment {
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
                if let Some(some) = self.r#content_type.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("contentType", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_contentType", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#content_type.as_ref() {
                    state.serialize_entry("contentType", some)?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#data.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("data", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_data", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#data.as_ref() {
                    state.serialize_entry("data", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#url.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("url", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_url", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#url.as_ref() {
                    state.serialize_entry("url", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#size.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("size", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_size", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#size.as_ref() {
                    state.serialize_entry("size", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#hash.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("hash", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_hash", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#hash.as_ref() {
                    state.serialize_entry("hash", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#title.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("title", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_title", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#title.as_ref() {
                    state.serialize_entry("title", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#creation.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("creation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_creation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#creation.as_ref() {
                    state.serialize_entry("creation", some)?;
                }
            }
            state.end()
        })
    }
}
