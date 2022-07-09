// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct CoverageEligibilityRequestInsurance {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#coverage: Box<super::super::types::Reference>,
    pub r#business_arrangement: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#focal: Option<super::super::types::Boolean>,
}
#[derive(Debug, Clone)]
pub enum CoverageEligibilityRequestItemDiagnosisDiagnosis {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct CoverageEligibilityRequestItemDiagnosis {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#diagnosis: Option<CoverageEligibilityRequestItemDiagnosisDiagnosis>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct CoverageEligibilityRequestItem {
    pub r#detail: Vec<Box<super::super::types::Reference>>,
    pub r#facility: Option<Box<super::super::types::Reference>>,
    pub r#provider: Option<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#product_or_service: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#supporting_info_sequence: Vec<super::super::types::PositiveInt>,
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    pub r#diagnosis: Vec<CoverageEligibilityRequestItemDiagnosis>,
}
#[derive(Debug, Clone)]
pub enum CoverageEligibilityRequestServiced {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub struct CoverageEligibilityRequestSupportingInfo {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#applies_to_all: Option<super::super::types::Boolean>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#information: Box<super::super::types::Reference>,
}
#[derive(Debug, Clone)]
pub struct CoverageEligibilityRequest {
    pub r#facility: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#insurer: Box<super::super::types::Reference>,
    pub r#provider: Option<Box<super::super::types::Reference>>,
    pub r#priority: Option<Box<super::super::types::CodeableConcept>>,
    pub r#status: super::super::types::Code,
    pub r#id: Option<std::string::String>,
    pub r#enterer: Option<Box<super::super::types::Reference>>,
    pub r#insurance: Vec<CoverageEligibilityRequestInsurance>,
    pub r#purpose: Vec<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#created: super::super::types::DateTime,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#item: Vec<CoverageEligibilityRequestItem>,
    pub r#serviced: Option<CoverageEligibilityRequestServiced>,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#supporting_info: Vec<CoverageEligibilityRequestSupportingInfo>,
    pub r#language: Option<super::super::types::Code>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
}
