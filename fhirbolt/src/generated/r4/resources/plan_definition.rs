// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum PlanDefinitionGoalTargetDetail {
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
}
#[derive(Debug, Clone)]
pub enum PlanDefinitionSubject {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub enum PlanDefinitionActionRelatedActionOffset {
    Duration(Box<super::super::types::Duration>),
    Range(Box<super::super::types::Range>),
}
#[derive(Debug, Clone)]
pub enum PlanDefinitionActionSubject {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub enum PlanDefinitionActionTiming {
    DateTime(Box<super::super::types::DateTime>),
    Age(Box<super::super::types::Age>),
    Period(Box<super::super::types::Period>),
    Duration(Box<super::super::types::Duration>),
    Range(Box<super::super::types::Range>),
    Timing(Box<super::super::types::Timing>),
}
#[derive(Debug, Clone)]
pub enum PlanDefinitionActionDefinition {
    Canonical(Box<super::super::types::Canonical>),
    Uri(Box<super::super::types::Uri>),
}
#[derive(Debug, Clone)]
pub struct PlanDefinitionGoalTarget {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#detail: Option<PlanDefinitionGoalTargetDetail>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#measure: Option<Box<super::super::types::CodeableConcept>>,
    pub r#due: Option<Box<super::super::types::Duration>>,
}
#[derive(Debug, Clone)]
pub struct PlanDefinitionGoal {
    pub r#priority: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Box<super::super::types::CodeableConcept>,
    pub r#target: Vec<PlanDefinitionGoalTarget>,
    pub r#start: Option<Box<super::super::types::CodeableConcept>>,
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    pub r#documentation: Vec<Box<super::super::types::RelatedArtifact>>,
    pub r#addresses: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct PlanDefinitionActionRelatedAction {
    pub r#action_id: super::super::types::Id,
    pub r#offset: Option<PlanDefinitionActionRelatedActionOffset>,
    pub r#relationship: super::super::types::Code,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct PlanDefinitionActionDynamicValue {
    pub r#expression: Option<Box<super::super::types::Expression>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#path: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct PlanDefinitionActionParticipant {
    pub r#id: Option<std::string::String>,
    pub r#type: super::super::types::Code,
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct PlanDefinitionActionCondition {
    pub r#expression: Option<Box<super::super::types::Expression>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#kind: super::super::types::Code,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct PlanDefinitionAction {
    pub r#output: Vec<Box<super::super::types::DataRequirement>>,
    pub r#related_action: Vec<PlanDefinitionActionRelatedAction>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#priority: Option<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#transform: Option<super::super::types::Canonical>,
    pub r#precheck_behavior: Option<super::super::types::Code>,
    pub r#selection_behavior: Option<super::super::types::Code>,
    pub r#description: Option<super::super::types::String>,
    pub r#required_behavior: Option<super::super::types::Code>,
    pub r#subject: Option<PlanDefinitionActionSubject>,
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#dynamic_value: Vec<PlanDefinitionActionDynamicValue>,
    pub r#text_equivalent: Option<super::super::types::String>,
    pub r#documentation: Vec<Box<super::super::types::RelatedArtifact>>,
    pub r#goal_id: Vec<super::super::types::Id>,
    pub r#timing: Option<PlanDefinitionActionTiming>,
    pub r#grouping_behavior: Option<super::super::types::Code>,
    pub r#participant: Vec<PlanDefinitionActionParticipant>,
    pub r#prefix: Option<super::super::types::String>,
    pub r#trigger: Vec<Box<super::super::types::TriggerDefinition>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#reason: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#input: Vec<Box<super::super::types::DataRequirement>>,
    pub r#definition: Option<PlanDefinitionActionDefinition>,
    pub r#action: Vec<PlanDefinitionAction>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#title: Option<super::super::types::String>,
    pub r#condition: Vec<PlanDefinitionActionCondition>,
    pub r#cardinality_behavior: Option<super::super::types::Code>,
}
#[derive(Debug, Clone)]
pub struct PlanDefinition {
    pub r#title: Option<super::super::types::String>,
    pub r#reviewer: Vec<Box<super::super::types::ContactDetail>>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#library: Vec<super::super::types::Canonical>,
    pub r#related_artifact: Vec<Box<super::super::types::RelatedArtifact>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    pub r#goal: Vec<PlanDefinitionGoal>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#subject: Option<PlanDefinitionSubject>,
    pub r#endorser: Vec<Box<super::super::types::ContactDetail>>,
    pub r#subtitle: Option<super::super::types::String>,
    pub r#version: Option<super::super::types::String>,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#action: Vec<PlanDefinitionAction>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#approval_date: Option<super::super::types::Date>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#author: Vec<Box<super::super::types::ContactDetail>>,
    pub r#id: Option<std::string::String>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#editor: Vec<Box<super::super::types::ContactDetail>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#status: super::super::types::Code,
    pub r#usage: Option<super::super::types::String>,
    pub r#topic: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#purpose: Option<super::super::types::Markdown>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#name: Option<super::super::types::String>,
    pub r#last_review_date: Option<super::super::types::Date>,
}
