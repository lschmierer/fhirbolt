// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum PopulationAge {
    Range(Box<super::super::types::Range>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
}
#[derive(Debug, Clone)]
pub struct Population {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#race: Option<Box<super::super::types::CodeableConcept>>,
    pub r#physiological_condition: Option<Box<super::super::types::CodeableConcept>>,
    pub r#age: Option<PopulationAge>,
    pub r#gender: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
