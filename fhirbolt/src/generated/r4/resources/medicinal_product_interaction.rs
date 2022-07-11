// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum MedicinalProductInteractionInteractantItem {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
}
#[derive(Debug, Clone)]
pub struct MedicinalProductInteractionInteractant {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#item: MedicinalProductInteractionInteractantItem,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MedicinalProductInteraction {
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#effect: Option<Box<super::super::types::CodeableConcept>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#subject: Vec<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#management: Option<Box<super::super::types::CodeableConcept>>,
    pub r#description: Option<super::super::types::String>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#interactant: Vec<MedicinalProductInteractionInteractant>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#incidence: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#language: Option<super::super::types::Code>,
}
