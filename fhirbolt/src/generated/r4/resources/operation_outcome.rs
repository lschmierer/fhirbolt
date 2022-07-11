// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct OperationOutcomeIssue {
    pub r#location: Vec<super::super::types::String>,
    pub r#diagnostics: Option<super::super::types::String>,
    pub r#severity: super::super::types::Code,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: super::super::types::Code,
    pub r#details: Option<Box<super::super::types::CodeableConcept>>,
    pub r#expression: Vec<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct OperationOutcome {
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#issue: Vec<OperationOutcomeIssue>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
}
