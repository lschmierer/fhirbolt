// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum CoverageEligibilityResponseServiced {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub struct CoverageEligibilityResponseError {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Box<super::super::types::CodeableConcept>,
}
#[derive(Debug, Clone)]
pub enum CoverageEligibilityResponseInsuranceItemBenefitUsed {
    UnsignedInt(Box<super::super::types::UnsignedInt>),
    String(Box<super::super::types::String>),
    Money(Box<super::super::types::Money>),
}
#[derive(Debug, Clone)]
pub enum CoverageEligibilityResponseInsuranceItemBenefitAllowed {
    UnsignedInt(Box<super::super::types::UnsignedInt>),
    String(Box<super::super::types::String>),
    Money(Box<super::super::types::Money>),
}
#[derive(Debug, Clone)]
pub struct CoverageEligibilityResponseInsuranceItemBenefit {
    pub r#used: Option<CoverageEligibilityResponseInsuranceItemBenefitUsed>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#allowed: Option<CoverageEligibilityResponseInsuranceItemBenefitAllowed>,
}
#[derive(Debug, Clone)]
pub struct CoverageEligibilityResponseInsuranceItem {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#network: Option<Box<super::super::types::CodeableConcept>>,
    pub r#excluded: Option<super::super::types::Boolean>,
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#name: Option<super::super::types::String>,
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    pub r#product_or_service: Option<Box<super::super::types::CodeableConcept>>,
    pub r#term: Option<Box<super::super::types::CodeableConcept>>,
    pub r#authorization_url: Option<super::super::types::Uri>,
    pub r#authorization_supporting: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#authorization_required: Option<super::super::types::Boolean>,
    pub r#provider: Option<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Option<super::super::types::String>,
    pub r#benefit: Vec<CoverageEligibilityResponseInsuranceItemBenefit>,
    pub r#unit: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct CoverageEligibilityResponseInsurance {
    pub r#inforce: Option<super::super::types::Boolean>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#benefit_period: Option<Box<super::super::types::Period>>,
    pub r#id: Option<std::string::String>,
    pub r#item: Vec<CoverageEligibilityResponseInsuranceItem>,
    pub r#coverage: Box<super::super::types::Reference>,
}
#[derive(Debug, Clone)]
pub struct CoverageEligibilityResponse {
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#disposition: Option<super::super::types::String>,
    pub r#serviced: Option<CoverageEligibilityResponseServiced>,
    pub r#request: Box<super::super::types::Reference>,
    pub r#insurer: Box<super::super::types::Reference>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#error: Vec<CoverageEligibilityResponseError>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#created: super::super::types::DateTime,
    pub r#id: Option<std::string::String>,
    pub r#outcome: super::super::types::Code,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#status: super::super::types::Code,
    pub r#purpose: Vec<super::super::types::Code>,
    pub r#pre_auth_ref: Option<super::super::types::String>,
    pub r#form: Option<Box<super::super::types::CodeableConcept>>,
    pub r#requestor: Option<Box<super::super::types::Reference>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#insurance: Vec<CoverageEligibilityResponseInsurance>,
}
