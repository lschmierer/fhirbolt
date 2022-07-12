// Generated on 2022-07-12 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum PopulationAge {
    Range(Box<super::super::types::Range>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
}
#[derive(Debug, Clone)]
pub struct Population {
    pub r#id: Option<std::string::String>,
    pub r#race: Option<Box<super::super::types::CodeableConcept>>,
    pub r#physiological_condition: Option<Box<super::super::types::CodeableConcept>>,
    pub r#gender: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#age: Option<PopulationAge>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
impl serde::Serialize for Population {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#race.as_ref() {
            state.serialize_entry("race", some)?;
        }
        if let Some(some) = self.r#physiological_condition.as_ref() {
            state.serialize_entry("physiologicalCondition", some)?;
        }
        if let Some(some) = self.r#gender.as_ref() {
            state.serialize_entry("gender", some)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#age.as_ref() {
            match some {
                PopulationAge::Range(ref value) => {
                    state.serialize_entry("ageRange", value)?;
                }
                PopulationAge::CodeableConcept(ref value) => {
                    state.serialize_entry("ageCodeableConcept", value)?;
                }
            }
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        state.end()
    }
}
