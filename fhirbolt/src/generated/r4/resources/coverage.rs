// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum CoverageCostToBeneficiaryValue {
    Quantity(Box<super::super::types::Quantity>),
    Money(Box<super::super::types::Money>),
}
#[derive(Debug, Clone)]
pub struct CoverageCostToBeneficiaryException {
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct CoverageCostToBeneficiary {
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#value: CoverageCostToBeneficiaryValue,
    pub r#exception: Vec<CoverageCostToBeneficiaryException>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct CoverageClass {
    pub r#name: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#value: super::super::types::String,
}
#[derive(Debug, Clone)]
pub struct Coverage {
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#status: super::super::types::Code,
    pub r#dependent: Option<super::super::types::String>,
    pub r#order: Option<super::super::types::PositiveInt>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#subscriber: Option<Box<super::super::types::Reference>>,
    pub r#subscriber_id: Option<super::super::types::String>,
    pub r#relationship: Option<Box<super::super::types::CodeableConcept>>,
    pub r#network: Option<super::super::types::String>,
    pub r#cost_to_beneficiary: Vec<CoverageCostToBeneficiary>,
    pub r#payor: Vec<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#beneficiary: Box<super::super::types::Reference>,
    pub r#class: Vec<CoverageClass>,
    pub r#contract: Vec<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#subrogation: Option<super::super::types::Boolean>,
    pub r#language: Option<super::super::types::Code>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#policy_holder: Option<Box<super::super::types::Reference>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#id: Option<std::string::String>,
}
