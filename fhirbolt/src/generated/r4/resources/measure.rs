// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct MeasureSupplementalData {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#usage: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#criteria: Box<super::super::types::Expression>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#description: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct MeasureGroupStratifierComponent {
    pub r#description: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#criteria: Box<super::super::types::Expression>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct MeasureGroupStratifier {
    pub r#criteria: Option<Box<super::super::types::Expression>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#component: Vec<MeasureGroupStratifierComponent>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#description: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct MeasureGroupPopulation {
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#description: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#criteria: Box<super::super::types::Expression>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MeasureGroup {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#stratifier: Vec<MeasureGroupStratifier>,
    pub r#population: Vec<MeasureGroupPopulation>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub enum MeasureSubject {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct Measure {
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#title: Option<super::super::types::String>,
    pub r#subtitle: Option<super::super::types::String>,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#endorser: Vec<Box<super::super::types::ContactDetail>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#supplemental_data: Vec<MeasureSupplementalData>,
    pub r#clinical_recommendation_statement: Option<super::super::types::Markdown>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#version: Option<super::super::types::String>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#definition: Vec<super::super::types::Markdown>,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#status: super::super::types::Code,
    pub r#disclaimer: Option<super::super::types::Markdown>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#id: Option<std::string::String>,
    pub r#usage: Option<super::super::types::String>,
    pub r#guidance: Option<super::super::types::Markdown>,
    pub r#last_review_date: Option<super::super::types::Date>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#approval_date: Option<super::super::types::Date>,
    pub r#library: Vec<super::super::types::Canonical>,
    pub r#group: Vec<MeasureGroup>,
    pub r#reviewer: Vec<Box<super::super::types::ContactDetail>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#improvement_notation: Option<Box<super::super::types::CodeableConcept>>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#author: Vec<Box<super::super::types::ContactDetail>>,
    pub r#related_artifact: Vec<Box<super::super::types::RelatedArtifact>>,
    pub r#name: Option<super::super::types::String>,
    pub r#risk_adjustment: Option<super::super::types::String>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#purpose: Option<super::super::types::Markdown>,
    pub r#topic: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#editor: Vec<Box<super::super::types::ContactDetail>>,
    pub r#composite_scoring: Option<Box<super::super::types::CodeableConcept>>,
    pub r#rate_aggregation: Option<super::super::types::String>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#subject: Option<MeasureSubject>,
    pub r#rationale: Option<super::super::types::Markdown>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    pub r#scoring: Option<Box<super::super::types::CodeableConcept>>,
}
