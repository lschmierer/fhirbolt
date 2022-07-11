// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct BodyStructure {
    pub r#image: Vec<Box<super::super::types::Attachment>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#morphology: Option<Box<super::super::types::CodeableConcept>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#active: Option<super::super::types::Boolean>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#location: Option<Box<super::super::types::CodeableConcept>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#description: Option<super::super::types::String>,
    pub r#location_qualifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
}
