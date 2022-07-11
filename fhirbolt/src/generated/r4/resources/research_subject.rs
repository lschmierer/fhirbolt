// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct ResearchSubject {
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#actual_arm: Option<super::super::types::String>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#study: Box<super::super::types::Reference>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#status: super::super::types::Code,
    pub r#individual: Box<super::super::types::Reference>,
    pub r#assigned_arm: Option<super::super::types::String>,
    pub r#consent: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
