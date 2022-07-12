// Generated on 2022-07-12 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct ProductShelfLife {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#period: Box<super::super::types::Quantity>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#special_precautions_for_storage: Vec<Box<super::super::types::CodeableConcept>>,
}
impl serde::Serialize for ProductShelfLife {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        state.serialize_entry("type", &self.r#type)?;
        state.serialize_entry("period", &self.r#period)?;
        if let Some(some) = self.r#identifier.as_ref() {
            state.serialize_entry("identifier", some)?;
        }
        if !self.r#special_precautions_for_storage.is_empty() {
            state.serialize_entry(
                "specialPrecautionsForStorage",
                &self.r#special_precautions_for_storage,
            )?;
        }
        state.end()
    }
}
