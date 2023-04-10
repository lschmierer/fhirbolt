// Generated on 2023-04-10 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for positiveInt type: An integer with a value that is positive (e.g. >0)"]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PositiveInt {
    #[doc = "unique id for the element within a resource (for internal references)"]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Primitive value for positiveInt"]
    pub r#value: Option<u32>,
}
impl serde::ser::Serialize for PositiveInt {
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
            if let Some(some) = self.r#value.as_ref() {
                state.serialize_entry("value", some)?;
            }
            state.end()
        })
    }
}
