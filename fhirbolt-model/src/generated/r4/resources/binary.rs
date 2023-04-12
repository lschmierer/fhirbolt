// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "A resource that represents the data of a single raw artifact as digital content accessible in its native format.  A Binary resource can contain any content, whether text, image, pdf, zip archive, etc.\n\nThere are situations where it is useful or required to handle pure binary content using the same framework as other resources."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Binary {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<std::string::String>,
    #[doc = "The metadata about the resource. This is content that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource."]
    pub r#meta: Option<Box<super::super::types::Meta>>,
    #[doc = "A reference to a set of rules that were followed when the resource was constructed, and which must be understood when processing the content. Often, this is a reference to an implementation guide that defines the special rules along with other profiles etc."]
    pub r#implicit_rules: Option<super::super::types::Uri>,
    #[doc = "The base language in which the resource is written."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "MimeType of the binary content represented as a standard MimeType (BCP 13)."]
    pub r#content_type: super::super::types::Code,
    #[doc = "This element identifies another resource that can be used as a proxy of the security sensitivity to use when deciding and enforcing access control rules for the Binary resource. Given that the Binary resource contains very few elements that can be used to determine the sensitivity of the data and relationships to individuals, the referenced resource stands in as a proxy equivalent for this purpose. This referenced resource may be related to the Binary (e.g. Media, DocumentReference), or may be some non-related Resource purely as a security proxy. E.g. to identify that the binary resource relates to a patient, and access should only be granted to applications that have access to the patient."]
    pub r#security_context: Option<Box<super::super::types::Reference>>,
    #[doc = "The actual content, base64 encoded."]
    pub r#data: Option<super::super::types::Base64Binary>,
}
impl serde::ser::Serialize for Binary {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "Binary")?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if let Some(some) = self.r#meta.as_ref() {
                state.serialize_entry("meta", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#implicit_rules.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("implicitRules", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_implicitRules", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#implicit_rules.as_ref() {
                    state.serialize_entry("implicitRules", some)?;
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
                if let Some(some) = self.r#content_type.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("contentType", &some)?;
                }
                if self.r#content_type.id.is_some() || !self.r#content_type.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#content_type.id.as_ref(),
                        extension: &self.r#content_type.extension,
                    };
                    state.serialize_entry("_contentType", &primitive_element)?;
                }
            } else {
                state.serialize_entry("contentType", &self.r#content_type)?;
            }
            if let Some(some) = self.r#security_context.as_ref() {
                state.serialize_entry("securityContext", some)?;
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
