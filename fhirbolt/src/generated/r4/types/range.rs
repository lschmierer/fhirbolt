// Generated on 2022-07-12 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct Range {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#low: Option<Box<super::super::types::Quantity>>,
    pub r#high: Option<Box<super::super::types::Quantity>>,
    pub r#id: Option<std::string::String>,
}
impl serde::Serialize for Range {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if let Some(some) = self.r#low.as_ref() {
            state.serialize_entry("low", some)?;
        }
        if let Some(some) = self.r#high.as_ref() {
            state.serialize_entry("high", some)?;
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        state.end()
    }
}
