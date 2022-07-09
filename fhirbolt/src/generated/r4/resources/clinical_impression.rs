// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum ClinicalImpressionEffective {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub struct ClinicalImpressionInvestigation {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#item: Vec<Box<super::super::types::Reference>>,
}
#[derive(Debug, Clone)]
pub struct ClinicalImpressionFinding {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#item_codeable_concept: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#basis: Option<super::super::types::String>,
    pub r#item_reference: Option<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ClinicalImpression {
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#assessor: Option<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#description: Option<super::super::types::String>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#effective: Option<ClinicalImpressionEffective>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#previous: Option<Box<super::super::types::Reference>>,
    pub r#problem: Vec<Box<super::super::types::Reference>>,
    pub r#investigation: Vec<ClinicalImpressionInvestigation>,
    pub r#protocol: Vec<super::super::types::Uri>,
    pub r#finding: Vec<ClinicalImpressionFinding>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#summary: Option<super::super::types::String>,
    pub r#status: super::super::types::Code,
    pub r#prognosis_reference: Vec<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#supporting_info: Vec<Box<super::super::types::Reference>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#prognosis_codeable_concept: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
}
