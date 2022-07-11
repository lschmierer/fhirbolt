// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct EffectEvidenceSynthesisSampleSize {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#number_of_participants: Option<super::super::types::Integer>,
    pub r#id: Option<std::string::String>,
    pub r#number_of_studies: Option<super::super::types::Integer>,
    pub r#description: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct EffectEvidenceSynthesisResultsByExposure {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#exposure_state: Option<super::super::types::Code>,
    pub r#risk_evidence_synthesis: Box<super::super::types::Reference>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#variant_state: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#description: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct EffectEvidenceSynthesisCertaintyCertaintySubcomponent {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#rating: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct EffectEvidenceSynthesisCertainty {
    pub r#certainty_subcomponent: Vec<EffectEvidenceSynthesisCertaintyCertaintySubcomponent>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#id: Option<std::string::String>,
    pub r#rating: Vec<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct EffectEvidenceSynthesisEffectEstimatePrecisionEstimate {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#level: Option<super::super::types::Decimal>,
    pub r#from: Option<super::super::types::Decimal>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#to: Option<super::super::types::Decimal>,
}
#[derive(Debug, Clone)]
pub struct EffectEvidenceSynthesisEffectEstimate {
    pub r#unit_of_measure: Option<Box<super::super::types::CodeableConcept>>,
    pub r#description: Option<super::super::types::String>,
    pub r#variant_state: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#value: Option<super::super::types::Decimal>,
    pub r#precision_estimate: Vec<EffectEvidenceSynthesisEffectEstimatePrecisionEstimate>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct EffectEvidenceSynthesis {
    pub r#title: Option<super::super::types::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#population: Box<super::super::types::Reference>,
    pub r#outcome: Box<super::super::types::Reference>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#id: Option<std::string::String>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#last_review_date: Option<super::super::types::Date>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#reviewer: Vec<Box<super::super::types::ContactDetail>>,
    pub r#sample_size: Option<EffectEvidenceSynthesisSampleSize>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#results_by_exposure: Vec<EffectEvidenceSynthesisResultsByExposure>,
    pub r#endorser: Vec<Box<super::super::types::ContactDetail>>,
    pub r#study_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#name: Option<super::super::types::String>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#approval_date: Option<super::super::types::Date>,
    pub r#version: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#certainty: Vec<EffectEvidenceSynthesisCertainty>,
    pub r#editor: Vec<Box<super::super::types::ContactDetail>>,
    pub r#exposure: Box<super::super::types::Reference>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#exposure_alternative: Box<super::super::types::Reference>,
    pub r#effect_estimate: Vec<EffectEvidenceSynthesisEffectEstimate>,
    pub r#related_artifact: Vec<Box<super::super::types::RelatedArtifact>>,
    pub r#synthesis_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    pub r#status: super::super::types::Code,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#topic: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#author: Vec<Box<super::super::types::ContactDetail>>,
}
