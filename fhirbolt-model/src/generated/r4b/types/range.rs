// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for Range Type: A set of ordered Quantities defined by a low and high limit.\n\nNeed to be able to specify ranges of values."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Range {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The low limit. The boundary is inclusive."]
    pub r#low: Option<Box<super::super::types::Quantity>>,
    #[doc = "The high limit. The boundary is inclusive."]
    pub r#high: Option<Box<super::super::types::Quantity>>,
}
impl serde::ser::Serialize for Range {
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
            if let Some(some) = self.r#low.as_ref() {
                state.serialize_entry("low", some)?;
            }
            if let Some(some) = self.r#high.as_ref() {
                state.serialize_entry("high", some)?;
            }
            state.end()
        })
    }
}
