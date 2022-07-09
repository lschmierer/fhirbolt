// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum GuidanceResponseModule {
    Uri(Box<super::super::types::Uri>),
    Canonical(Box<super::super::types::Canonical>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
}
#[derive(Debug, Clone)]
pub struct GuidanceResponse {
    pub r#evaluation_message: Vec<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#result: Option<Box<super::super::types::Reference>>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#output_parameters: Option<Box<super::super::types::Reference>>,
    pub r#data_requirement: Vec<Box<super::super::types::DataRequirement>>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#performer: Option<Box<super::super::types::Reference>>,
    pub r#status: super::super::types::Code,
    pub r#occurrence_date_time: Option<super::super::types::DateTime>,
    pub r#module: GuidanceResponseModule,
    pub r#language: Option<super::super::types::Code>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#request_identifier: Option<Box<super::super::types::Identifier>>,
    pub r#contained: Vec<Box<super::Resource>>,
}
