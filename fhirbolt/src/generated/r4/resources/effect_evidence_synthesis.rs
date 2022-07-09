// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct EffectEvidenceSynthesisSampleSize {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#number_of_studies: Option<super::super::types::Integer>,
    pub r#number_of_participants: Option<super::super::types::Integer>,
    pub r#description: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct EffectEvidenceSynthesisCertaintyCertaintySubcomponent {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#rating: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
#[derive(Debug, Clone)]
pub struct EffectEvidenceSynthesisCertainty {
    pub r#rating: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#id: Option<std::string::String>,
    pub r#certainty_subcomponent: Vec<EffectEvidenceSynthesisCertaintyCertaintySubcomponent>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct EffectEvidenceSynthesisResultsByExposure {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Option<super::super::types::String>,
    pub r#risk_evidence_synthesis: Box<super::super::types::Reference>,
    pub r#exposure_state: Option<super::super::types::Code>,
    pub r#variant_state: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct EffectEvidenceSynthesisEffectEstimatePrecisionEstimate {
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#from: Option<super::super::types::Decimal>,
    pub r#level: Option<super::super::types::Decimal>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#to: Option<super::super::types::Decimal>,
}
#[derive(Debug, Clone)]
pub struct EffectEvidenceSynthesisEffectEstimate {
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#variant_state: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#unit_of_measure: Option<Box<super::super::types::CodeableConcept>>,
    pub r#precision_estimate: Vec<EffectEvidenceSynthesisEffectEstimatePrecisionEstimate>,
    pub r#description: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#value: Option<super::super::types::Decimal>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct EffectEvidenceSynthesis {
    pub r#endorser: Vec<Box<super::super::types::ContactDetail>>,
    pub r#last_review_date: Option<super::super::types::Date>,
    pub r#reviewer: Vec<Box<super::super::types::ContactDetail>>,
    pub r#synthesis_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#related_artifact: Vec<Box<super::super::types::RelatedArtifact>>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#exposure_alternative: Box<super::super::types::Reference>,
    pub r#approval_date: Option<super::super::types::Date>,
    pub r#name: Option<super::super::types::String>,
    pub r#exposure: Box<super::super::types::Reference>,
    pub r#language: Option<super::super::types::Code>,
    pub r#editor: Vec<Box<super::super::types::ContactDetail>>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#status: super::super::types::Code,
    pub r#population: Box<super::super::types::Reference>,
    pub r#sample_size: Option<EffectEvidenceSynthesisSampleSize>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#certainty: Vec<EffectEvidenceSynthesisCertainty>,
    pub r#results_by_exposure: Vec<EffectEvidenceSynthesisResultsByExposure>,
    pub r#title: Option<super::super::types::String>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#study_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#id: Option<std::string::String>,
    pub r#topic: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#outcome: Box<super::super::types::Reference>,
    pub r#author: Vec<Box<super::super::types::ContactDetail>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#version: Option<super::super::types::String>,
    pub r#effect_estimate: Vec<EffectEvidenceSynthesisEffectEstimate>,
}
