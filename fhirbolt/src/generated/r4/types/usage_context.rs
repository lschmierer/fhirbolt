// Generated on 2022-07-12 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum UsageContextValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct UsageContext {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#value: UsageContextValue,
    pub r#code: Box<super::super::types::Coding>,
}
impl serde::Serialize for UsageContext {
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
        match self.r#value {
            UsageContextValue::CodeableConcept(ref value) => {
                state.serialize_entry("valueCodeableConcept", value)?;
            }
            UsageContextValue::Quantity(ref value) => {
                state.serialize_entry("valueQuantity", value)?;
            }
            UsageContextValue::Range(ref value) => {
                state.serialize_entry("valueRange", value)?;
            }
            UsageContextValue::Reference(ref value) => {
                state.serialize_entry("valueReference", value)?;
            }
        }
        state.serialize_entry("code", &self.r#code)?;
        state.end()
    }
}
