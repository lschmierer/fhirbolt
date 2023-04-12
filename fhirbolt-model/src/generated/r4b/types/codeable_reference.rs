// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for CodeableReference Type: A reference to a resource (by instance), or instead, a reference to a concept defined in a terminology or ontology (by class).\n\nThis is a common pattern in record keeping - a reference may be made to a specific condition, observation, plan, or definition, or a reference may be made to a general concept defined in a knowledge base somewhere."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CodeableReference {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A reference to a concept - e.g. the information is identified by its general class to the degree of precision found in the terminology."]
    pub r#concept: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A reference to a resource the provides exact details about the information being referenced."]
    pub r#reference: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for CodeableReference {
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
            if let Some(some) = self.r#concept.as_ref() {
                state.serialize_entry("concept", some)?;
            }
            if let Some(some) = self.r#reference.as_ref() {
                state.serialize_entry("reference", some)?;
            }
            state.end()
        })
    }
}
