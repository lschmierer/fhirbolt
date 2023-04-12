// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for RelatedArtifact Type: Related artifacts such as additional documentation, justification, or bibliographic references.\n\nKnowledge resources must be able to provide enough information for consumers of the content (and/or interventions or results produced by the content) to be able to determine and understand the justification for and evidence in support of the content."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RelatedArtifact {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of relationship to the related artifact."]
    pub r#type: super::super::types::Code,
    #[doc = "A short label that can be used to reference the citation from elsewhere in the containing artifact, such as a footnote index."]
    pub r#label: Option<super::super::types::String>,
    #[doc = "A brief description of the document or knowledge resource being referenced, suitable for display to a consumer."]
    pub r#display: Option<super::super::types::String>,
    #[doc = "A bibliographic citation for the related artifact. This text SHOULD be formatted according to an accepted citation format."]
    pub r#citation: Option<super::super::types::Markdown>,
    #[doc = "A url for the artifact that can be followed to access the actual content."]
    pub r#url: Option<super::super::types::Url>,
    #[doc = "The document being referenced, represented as an attachment. This is exclusive with the resource element."]
    pub r#document: Option<Box<super::super::types::Attachment>>,
    #[doc = "The related resource, such as a library, value set, profile, or other knowledge resource."]
    pub r#resource: Option<super::super::types::Canonical>,
}
impl serde::ser::Serialize for RelatedArtifact {
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
                if let Some(some) = self.r#label.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("label", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_label", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#label.as_ref() {
                    state.serialize_entry("label", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#display.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("display", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_display", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#display.as_ref() {
                    state.serialize_entry("display", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#citation.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("citation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_citation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#citation.as_ref() {
                    state.serialize_entry("citation", some)?;
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
            if let Some(some) = self.r#document.as_ref() {
                state.serialize_entry("document", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#resource.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("resource", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_resource", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#resource.as_ref() {
                    state.serialize_entry("resource", some)?;
                }
            }
            state.end()
        })
    }
}
