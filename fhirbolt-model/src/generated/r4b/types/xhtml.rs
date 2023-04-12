// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for xhtml Type"]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Xhtml {
    #[doc = "unique id for the element within a resource (for internal references)"]
    pub r#id: Option<std::string::String>,
    #[doc = "Actual xhtml"]
    pub r#value: std::string::String,
}
impl serde::ser::Serialize for Xhtml {
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
            state.serialize_entry("value", &self.r#value)?;
            state.end()
        })
    }
}
