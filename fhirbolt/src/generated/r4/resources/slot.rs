// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct Slot {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#service_type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#status: super::super::types::Code,
    pub r#schedule: Box<super::super::types::Reference>,
    pub r#comment: Option<super::super::types::String>,
    pub r#appointment_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#end: super::super::types::Instant,
    pub r#specialty: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#start: super::super::types::Instant,
    pub r#overbooked: Option<super::super::types::Boolean>,
    pub r#id: Option<std::string::String>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#service_category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#language: Option<super::super::types::Code>,
}
