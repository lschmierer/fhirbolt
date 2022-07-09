// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct ChargeItemDefinitionPropertyGroupPriceComponent {
    pub r#type: super::super::types::Code,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#amount: Option<Box<super::super::types::Money>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#factor: Option<super::super::types::Decimal>,
}
#[derive(Debug, Clone)]
pub struct ChargeItemDefinitionPropertyGroup {
    pub r#applicability: Vec<ChargeItemDefinitionApplicability>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#price_component: Vec<ChargeItemDefinitionPropertyGroupPriceComponent>,
}
#[derive(Debug, Clone)]
pub struct ChargeItemDefinitionApplicability {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#expression: Option<super::super::types::String>,
    pub r#language: Option<super::super::types::String>,
    pub r#description: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct ChargeItemDefinition {
    pub r#part_of: Vec<super::super::types::Canonical>,
    pub r#property_group: Vec<ChargeItemDefinitionPropertyGroup>,
    pub r#language: Option<super::super::types::Code>,
    pub r#replaces: Vec<super::super::types::Canonical>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#instance: Vec<Box<super::super::types::Reference>>,
    pub r#applicability: Vec<ChargeItemDefinitionApplicability>,
    pub r#derived_from_uri: Vec<super::super::types::Uri>,
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#version: Option<super::super::types::String>,
    pub r#last_review_date: Option<super::super::types::Date>,
    pub r#id: Option<std::string::String>,
    pub r#approval_date: Option<super::super::types::Date>,
    pub r#url: super::super::types::Uri,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#title: Option<super::super::types::String>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#status: super::super::types::Code,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
