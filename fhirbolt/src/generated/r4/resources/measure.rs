// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum MeasureSubject {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct MeasureGroupPopulation {
    pub r#id: Option<std::string::String>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#criteria: Box<super::super::types::Expression>,
    pub r#description: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MeasureGroupStratifierComponent {
    pub r#criteria: Box<super::super::types::Expression>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Option<super::super::types::String>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct MeasureGroupStratifier {
    pub r#description: Option<super::super::types::String>,
    pub r#criteria: Option<Box<super::super::types::Expression>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#component: Vec<MeasureGroupStratifierComponent>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct MeasureGroup {
    pub r#description: Option<super::super::types::String>,
    pub r#population: Vec<MeasureGroupPopulation>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#stratifier: Vec<MeasureGroupStratifier>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct MeasureSupplementalData {
    pub r#description: Option<super::super::types::String>,
    pub r#usage: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#criteria: Box<super::super::types::Expression>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct Measure {
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#clinical_recommendation_statement: Option<super::super::types::Markdown>,
    pub r#composite_scoring: Option<Box<super::super::types::CodeableConcept>>,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    pub r#version: Option<super::super::types::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#approval_date: Option<super::super::types::Date>,
    pub r#risk_adjustment: Option<super::super::types::String>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#title: Option<super::super::types::String>,
    pub r#purpose: Option<super::super::types::Markdown>,
    pub r#scoring: Option<Box<super::super::types::CodeableConcept>>,
    pub r#rate_aggregation: Option<super::super::types::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#name: Option<super::super::types::String>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#guidance: Option<super::super::types::Markdown>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#subtitle: Option<super::super::types::String>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#usage: Option<super::super::types::String>,
    pub r#last_review_date: Option<super::super::types::Date>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#author: Vec<Box<super::super::types::ContactDetail>>,
    pub r#endorser: Vec<Box<super::super::types::ContactDetail>>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#related_artifact: Vec<Box<super::super::types::RelatedArtifact>>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#reviewer: Vec<Box<super::super::types::ContactDetail>>,
    pub r#rationale: Option<super::super::types::Markdown>,
    pub r#editor: Vec<Box<super::super::types::ContactDetail>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#definition: Vec<super::super::types::Markdown>,
    pub r#group: Vec<MeasureGroup>,
    pub r#id: Option<std::string::String>,
    pub r#library: Vec<super::super::types::Canonical>,
    pub r#improvement_notation: Option<Box<super::super::types::CodeableConcept>>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#subject: Option<MeasureSubject>,
    pub r#disclaimer: Option<super::super::types::Markdown>,
    pub r#supplemental_data: Vec<MeasureSupplementalData>,
    pub r#status: super::super::types::Code,
    pub r#topic: Vec<Box<super::super::types::CodeableConcept>>,
}
