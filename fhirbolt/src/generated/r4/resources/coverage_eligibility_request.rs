// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum CoverageEligibilityRequestItemDiagnosisDiagnosis {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub enum CoverageEligibilityRequestServiced {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub struct CoverageEligibilityRequestSupportingInfo {
    pub r#applies_to_all: Option<super::super::types::Boolean>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#information: Box<super::super::types::Reference>,
}
#[derive(Debug, Clone)]
pub struct CoverageEligibilityRequestItemDiagnosis {
    pub r#diagnosis: Option<CoverageEligibilityRequestItemDiagnosisDiagnosis>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct CoverageEligibilityRequestItem {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#detail: Vec<Box<super::super::types::Reference>>,
    pub r#product_or_service: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#facility: Option<Box<super::super::types::Reference>>,
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    pub r#provider: Option<Box<super::super::types::Reference>>,
    pub r#diagnosis: Vec<CoverageEligibilityRequestItemDiagnosis>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#supporting_info_sequence: Vec<super::super::types::PositiveInt>,
}
#[derive(Debug, Clone)]
pub struct CoverageEligibilityRequestInsurance {
    pub r#id: Option<std::string::String>,
    pub r#focal: Option<super::super::types::Boolean>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#coverage: Box<super::super::types::Reference>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#business_arrangement: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct CoverageEligibilityRequest {
    pub r#created: super::super::types::DateTime,
    pub r#supporting_info: Vec<CoverageEligibilityRequestSupportingInfo>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#item: Vec<CoverageEligibilityRequestItem>,
    pub r#insurer: Box<super::super::types::Reference>,
    pub r#serviced: Option<CoverageEligibilityRequestServiced>,
    pub r#purpose: Vec<super::super::types::Code>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#facility: Option<Box<super::super::types::Reference>>,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#insurance: Vec<CoverageEligibilityRequestInsurance>,
    pub r#language: Option<super::super::types::Code>,
    pub r#priority: Option<Box<super::super::types::CodeableConcept>>,
    pub r#enterer: Option<Box<super::super::types::Reference>>,
    pub r#provider: Option<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#status: super::super::types::Code,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
}
