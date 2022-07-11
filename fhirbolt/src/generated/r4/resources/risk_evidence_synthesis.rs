// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct RiskEvidenceSynthesisRiskEstimatePrecisionEstimate {
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#level: Option<super::super::types::Decimal>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#from: Option<super::super::types::Decimal>,
    pub r#to: Option<super::super::types::Decimal>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct RiskEvidenceSynthesisRiskEstimate {
    pub r#numerator_count: Option<super::super::types::Integer>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#unit_of_measure: Option<Box<super::super::types::CodeableConcept>>,
    pub r#denominator_count: Option<super::super::types::Integer>,
    pub r#precision_estimate: Vec<RiskEvidenceSynthesisRiskEstimatePrecisionEstimate>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#value: Option<super::super::types::Decimal>,
    pub r#description: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct RiskEvidenceSynthesisSampleSize {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#number_of_participants: Option<super::super::types::Integer>,
    pub r#number_of_studies: Option<super::super::types::Integer>,
}
#[derive(Debug, Clone)]
pub struct RiskEvidenceSynthesisCertaintyCertaintySubcomponent {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#rating: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
#[derive(Debug, Clone)]
pub struct RiskEvidenceSynthesisCertainty {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#id: Option<std::string::String>,
    pub r#certainty_subcomponent: Vec<RiskEvidenceSynthesisCertaintyCertaintySubcomponent>,
    pub r#rating: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct RiskEvidenceSynthesis {
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#endorser: Vec<Box<super::super::types::ContactDetail>>,
    pub r#name: Option<super::super::types::String>,
    pub r#related_artifact: Vec<Box<super::super::types::RelatedArtifact>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#version: Option<super::super::types::String>,
    pub r#author: Vec<Box<super::super::types::ContactDetail>>,
    pub r#synthesis_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#outcome: Box<super::super::types::Reference>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#risk_estimate: Option<RiskEvidenceSynthesisRiskEstimate>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#sample_size: Option<RiskEvidenceSynthesisSampleSize>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#title: Option<super::super::types::String>,
    pub r#editor: Vec<Box<super::super::types::ContactDetail>>,
    pub r#population: Box<super::super::types::Reference>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#exposure: Option<Box<super::super::types::Reference>>,
    pub r#topic: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#certainty: Vec<RiskEvidenceSynthesisCertainty>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#approval_date: Option<super::super::types::Date>,
    pub r#last_review_date: Option<super::super::types::Date>,
    pub r#reviewer: Vec<Box<super::super::types::ContactDetail>>,
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#study_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#status: super::super::types::Code,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#id: Option<std::string::String>,
}
