// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum CoverageCostToBeneficiaryValue {
    Quantity(Box<super::super::types::Quantity>),
    Money(Box<super::super::types::Money>),
}
#[derive(Debug, Clone)]
pub struct CoverageCostToBeneficiaryException {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct CoverageCostToBeneficiary {
    pub r#id: Option<std::string::String>,
    pub r#exception: Vec<CoverageCostToBeneficiaryException>,
    pub r#value: CoverageCostToBeneficiaryValue,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct CoverageClass {
    pub r#value: super::super::types::String,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: Option<super::super::types::String>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct Coverage {
    pub r#status: super::super::types::Code,
    pub r#language: Option<super::super::types::Code>,
    pub r#payor: Vec<Box<super::super::types::Reference>>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#subrogation: Option<super::super::types::Boolean>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#dependent: Option<super::super::types::String>,
    pub r#cost_to_beneficiary: Vec<CoverageCostToBeneficiary>,
    pub r#beneficiary: Box<super::super::types::Reference>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#network: Option<super::super::types::String>,
    pub r#contract: Vec<Box<super::super::types::Reference>>,
    pub r#subscriber_id: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#relationship: Option<Box<super::super::types::CodeableConcept>>,
    pub r#class: Vec<CoverageClass>,
    pub r#order: Option<super::super::types::PositiveInt>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#subscriber: Option<Box<super::super::types::Reference>>,
    pub r#policy_holder: Option<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
}
