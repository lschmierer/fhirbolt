// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "The individual responsible for making the annotation."]
#[derive(Debug, Clone, PartialEq)]
pub enum AnnotationAuthor {
    Reference(Box<super::super::types::Reference>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for AnnotationAuthor {
    fn default() -> AnnotationAuthor {
        AnnotationAuthor::Invalid
    }
}
#[doc = "Base StructureDefinition for Annotation Type: A  text note which also  contains information about who made the statement and when."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Annotation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The individual responsible for making the annotation."]
    pub r#author: Option<AnnotationAuthor>,
    #[doc = "Indicates when this particular annotation was made."]
    pub r#time: Option<super::super::types::DateTime>,
    #[doc = "The text of the annotation in markdown format."]
    pub r#text: super::super::types::Markdown,
}
impl serde::ser::Serialize for Annotation {
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
            if let Some(some) = self.r#author.as_ref() {
                match some {
                    AnnotationAuthor::Reference(ref value) => {
                        state.serialize_entry("authorReference", value)?;
                    }
                    AnnotationAuthor::String(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("authorString", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_authorString", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("authorString", value)?;
                        }
                    }
                    AnnotationAuthor::Invalid => {
                        return Err(serde::ser::Error::custom("author is invalid"))
                    }
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#time.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("time", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_time", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#time.as_ref() {
                    state.serialize_entry("time", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#text.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("text", &some)?;
                }
                if self.r#text.id.is_some() || !self.r#text.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#text.id.as_ref(),
                        extension: &self.r#text.extension,
                    };
                    state.serialize_entry("_text", &primitive_element)?;
                }
            } else {
                state.serialize_entry("text", &self.r#text)?;
            }
            state.end()
        })
    }
}
