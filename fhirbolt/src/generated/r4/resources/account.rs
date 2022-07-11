// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct AccountGuarantor {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#on_hold: Option<super::super::types::Boolean>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#party: Box<super::super::types::Reference>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct AccountCoverage {
    pub r#coverage: Box<super::super::types::Reference>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#priority: Option<super::super::types::PositiveInt>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct Account {
    pub r#guarantor: Vec<AccountGuarantor>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#coverage: Vec<AccountCoverage>,
    pub r#description: Option<super::super::types::String>,
    pub r#name: Option<super::super::types::String>,
    pub r#service_period: Option<Box<super::super::types::Period>>,
    pub r#owner: Option<Box<super::super::types::Reference>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#subject: Vec<Box<super::super::types::Reference>>,
    pub r#part_of: Option<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#status: super::super::types::Code,
}
