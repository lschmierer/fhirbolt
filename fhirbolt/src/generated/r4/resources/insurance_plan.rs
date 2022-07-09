// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct InsurancePlanCoverageBenefitLimit {
    pub r#value: Option<Box<super::super::types::Quantity>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct InsurancePlanCoverageBenefit {
    pub r#limit: Vec<InsurancePlanCoverageBenefitLimit>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#requirement: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct InsurancePlanCoverage {
    pub r#network: Vec<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#benefit: Vec<InsurancePlanCoverageBenefit>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct InsurancePlanPlanGeneralCost {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#comment: Option<super::super::types::String>,
    pub r#group_size: Option<super::super::types::PositiveInt>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#cost: Option<Box<super::super::types::Money>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct InsurancePlanPlanSpecificCostBenefitCost {
    pub r#id: Option<std::string::String>,
    pub r#qualifiers: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#value: Option<Box<super::super::types::Quantity>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#applicability: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct InsurancePlanPlanSpecificCostBenefit {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#cost: Vec<InsurancePlanPlanSpecificCostBenefitCost>,
}
#[derive(Debug, Clone)]
pub struct InsurancePlanPlanSpecificCost {
    pub r#benefit: Vec<InsurancePlanPlanSpecificCostBenefit>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#category: Box<super::super::types::CodeableConcept>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct InsurancePlanPlan {
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#general_cost: Vec<InsurancePlanPlanGeneralCost>,
    pub r#coverage_area: Vec<Box<super::super::types::Reference>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#specific_cost: Vec<InsurancePlanPlanSpecificCost>,
    pub r#network: Vec<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct InsurancePlanContact {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#purpose: Option<Box<super::super::types::CodeableConcept>>,
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    pub r#id: Option<std::string::String>,
    pub r#name: Option<Box<super::super::types::HumanName>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#address: Option<Box<super::super::types::Address>>,
}
#[derive(Debug, Clone)]
pub struct InsurancePlan {
    pub r#coverage: Vec<InsurancePlanCoverage>,
    pub r#status: Option<super::super::types::Code>,
    pub r#alias: Vec<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#network: Vec<Box<super::super::types::Reference>>,
    pub r#name: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#coverage_area: Vec<Box<super::super::types::Reference>>,
    pub r#plan: Vec<InsurancePlanPlan>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#contact: Vec<InsurancePlanContact>,
    pub r#language: Option<super::super::types::Code>,
    pub r#administered_by: Option<Box<super::super::types::Reference>>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#owned_by: Option<Box<super::super::types::Reference>>,
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
}
