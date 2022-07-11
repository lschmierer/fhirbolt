// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum CoverageEligibilityResponseServiced {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub enum CoverageEligibilityResponseInsuranceItemBenefitAllowed {
    UnsignedInt(Box<super::super::types::UnsignedInt>),
    String(Box<super::super::types::String>),
    Money(Box<super::super::types::Money>),
}
#[derive(Debug, Clone)]
pub enum CoverageEligibilityResponseInsuranceItemBenefitUsed {
    UnsignedInt(Box<super::super::types::UnsignedInt>),
    String(Box<super::super::types::String>),
    Money(Box<super::super::types::Money>),
}
#[derive(Debug, Clone)]
pub struct CoverageEligibilityResponseError {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#code: Box<super::super::types::CodeableConcept>,
}
#[derive(Debug, Clone)]
pub struct CoverageEligibilityResponseInsuranceItemBenefit {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#allowed: Option<CoverageEligibilityResponseInsuranceItemBenefitAllowed>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#used: Option<CoverageEligibilityResponseInsuranceItemBenefitUsed>,
}
#[derive(Debug, Clone)]
pub struct CoverageEligibilityResponseInsuranceItem {
    pub r#benefit: Vec<CoverageEligibilityResponseInsuranceItemBenefit>,
    pub r#name: Option<super::super::types::String>,
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#authorization_url: Option<super::super::types::Uri>,
    pub r#id: Option<std::string::String>,
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#authorization_supporting: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#excluded: Option<super::super::types::Boolean>,
    pub r#provider: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Option<super::super::types::String>,
    pub r#network: Option<Box<super::super::types::CodeableConcept>>,
    pub r#unit: Option<Box<super::super::types::CodeableConcept>>,
    pub r#term: Option<Box<super::super::types::CodeableConcept>>,
    pub r#product_or_service: Option<Box<super::super::types::CodeableConcept>>,
    pub r#authorization_required: Option<super::super::types::Boolean>,
}
#[derive(Debug, Clone)]
pub struct CoverageEligibilityResponseInsurance {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#inforce: Option<super::super::types::Boolean>,
    pub r#benefit_period: Option<Box<super::super::types::Period>>,
    pub r#coverage: Box<super::super::types::Reference>,
    pub r#item: Vec<CoverageEligibilityResponseInsuranceItem>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct CoverageEligibilityResponse {
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#id: Option<std::string::String>,
    pub r#pre_auth_ref: Option<super::super::types::String>,
    pub r#form: Option<Box<super::super::types::CodeableConcept>>,
    pub r#error: Vec<CoverageEligibilityResponseError>,
    pub r#status: super::super::types::Code,
    pub r#requestor: Option<Box<super::super::types::Reference>>,
    pub r#serviced: Option<CoverageEligibilityResponseServiced>,
    pub r#language: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#request: Box<super::super::types::Reference>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#insurer: Box<super::super::types::Reference>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#created: super::super::types::DateTime,
    pub r#outcome: super::super::types::Code,
    pub r#insurance: Vec<CoverageEligibilityResponseInsurance>,
    pub r#purpose: Vec<super::super::types::Code>,
    pub r#disposition: Option<super::super::types::String>,
}
