// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for Signature Type: A signature along with supporting context. The signature may be a digital signature that is cryptographic in nature, or some other signature acceptable to the domain. This other signature may be as simple as a graphical image representing a hand-written signature, or a signature ceremony Different signature approaches have different utilities.\n\nThere are a number of places where content must be signed in healthcare."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Signature {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "An indication of the reason that the entity signed this document. This may be explicitly included as part of the signature information and can be used when determining accountability for various actions concerning the document."]
    pub r#type: Vec<Box<super::super::types::Coding>>,
    #[doc = "When the digital signature was signed."]
    pub r#when: super::super::types::Instant,
    #[doc = "A reference to an application-usable description of the identity that signed  (e.g. the signature used their private key)."]
    pub r#who: Box<super::super::types::Reference>,
    #[doc = "A reference to an application-usable description of the identity that is represented by the signature."]
    pub r#on_behalf_of: Option<Box<super::super::types::Reference>>,
    #[doc = "A mime type that indicates the technical format of the target resources signed by the signature."]
    pub r#target_format: Option<super::super::types::Code>,
    #[doc = "A mime type that indicates the technical format of the signature. Important mime types are application/signature+xml for X ML DigSig, application/jose for JWS, and image/* for a graphical image of a signature, etc."]
    pub r#sig_format: Option<super::super::types::Code>,
    #[doc = "The base64 encoding of the Signature content. When signature is not recorded electronically this element would be empty."]
    pub r#data: Option<super::super::types::Base64Binary>,
}
impl serde::ser::Serialize for Signature {
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
            if !self.r#type.is_empty() {
                state.serialize_entry("type", &self.r#type)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#when.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("when", &some)?;
                }
                if self.r#when.id.is_some() || !self.r#when.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#when.id.as_ref(),
                        extension: &self.r#when.extension,
                    };
                    state.serialize_entry("_when", &primitive_element)?;
                }
            } else {
                state.serialize_entry("when", &self.r#when)?;
            }
            state.serialize_entry("who", &self.r#who)?;
            if let Some(some) = self.r#on_behalf_of.as_ref() {
                state.serialize_entry("onBehalfOf", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#target_format.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("targetFormat", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_targetFormat", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#target_format.as_ref() {
                    state.serialize_entry("targetFormat", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#sig_format.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("sigFormat", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_sigFormat", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#sig_format.as_ref() {
                    state.serialize_entry("sigFormat", some)?;
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
            state.end()
        })
    }
}
