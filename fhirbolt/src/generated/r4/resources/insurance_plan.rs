// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct InsurancePlanPlanSpecificCostBenefitCost {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#qualifiers: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#value: Option<Box<super::super::types::Quantity>>,
    pub r#applicability: Option<Box<super::super::types::CodeableConcept>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct InsurancePlanPlanSpecificCostBenefit {
    pub r#cost: Vec<InsurancePlanPlanSpecificCostBenefitCost>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct InsurancePlanPlanSpecificCost {
    pub r#category: Box<super::super::types::CodeableConcept>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#benefit: Vec<InsurancePlanPlanSpecificCostBenefit>,
}
#[derive(Debug, Clone)]
pub struct InsurancePlanPlanGeneralCost {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#group_size: Option<super::super::types::PositiveInt>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#comment: Option<super::super::types::String>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#cost: Option<Box<super::super::types::Money>>,
}
#[derive(Debug, Clone)]
pub struct InsurancePlanPlan {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#coverage_area: Vec<Box<super::super::types::Reference>>,
    pub r#network: Vec<Box<super::super::types::Reference>>,
    pub r#specific_cost: Vec<InsurancePlanPlanSpecificCost>,
    pub r#id: Option<std::string::String>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#general_cost: Vec<InsurancePlanPlanGeneralCost>,
}
#[derive(Debug, Clone)]
pub struct InsurancePlanContact {
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    pub r#address: Option<Box<super::super::types::Address>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#purpose: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: Option<Box<super::super::types::HumanName>>,
}
#[derive(Debug, Clone)]
pub struct InsurancePlanCoverageBenefitLimit {
    pub r#value: Option<Box<super::super::types::Quantity>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct InsurancePlanCoverageBenefit {
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#requirement: Option<super::super::types::String>,
    pub r#limit: Vec<InsurancePlanCoverageBenefitLimit>,
}
#[derive(Debug, Clone)]
pub struct InsurancePlanCoverage {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#network: Vec<Box<super::super::types::Reference>>,
    pub r#benefit: Vec<InsurancePlanCoverageBenefit>,
}
#[derive(Debug, Clone)]
pub struct InsurancePlan {
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#name: Option<super::super::types::String>,
    pub r#plan: Vec<InsurancePlanPlan>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#owned_by: Option<Box<super::super::types::Reference>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#alias: Vec<super::super::types::String>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#administered_by: Option<Box<super::super::types::Reference>>,
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
    pub r#network: Vec<Box<super::super::types::Reference>>,
    pub r#coverage_area: Vec<Box<super::super::types::Reference>>,
    pub r#status: Option<super::super::types::Code>,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#contact: Vec<InsurancePlanContact>,
    pub r#coverage: Vec<InsurancePlanCoverage>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
}
