// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct ChargeItemDefinitionPropertyGroupPriceComponent {
    pub r#id: Option<std::string::String>,
    pub r#amount: Option<Box<super::super::types::Money>>,
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#type: super::super::types::Code,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct ChargeItemDefinitionPropertyGroup {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#price_component: Vec<ChargeItemDefinitionPropertyGroupPriceComponent>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#applicability: Vec<ChargeItemDefinitionApplicability>,
}
#[derive(Debug, Clone)]
pub struct ChargeItemDefinitionApplicability {
    pub r#description: Option<super::super::types::String>,
    pub r#expression: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#language: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct ChargeItemDefinition {
    pub r#property_group: Vec<ChargeItemDefinitionPropertyGroup>,
    pub r#part_of: Vec<super::super::types::Canonical>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#replaces: Vec<super::super::types::Canonical>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#title: Option<super::super::types::String>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#last_review_date: Option<super::super::types::Date>,
    pub r#applicability: Vec<ChargeItemDefinitionApplicability>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#instance: Vec<Box<super::super::types::Reference>>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#status: super::super::types::Code,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#id: Option<std::string::String>,
    pub r#url: super::super::types::Uri,
    pub r#version: Option<super::super::types::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#derived_from_uri: Vec<super::super::types::Uri>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#approval_date: Option<super::super::types::Date>,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
