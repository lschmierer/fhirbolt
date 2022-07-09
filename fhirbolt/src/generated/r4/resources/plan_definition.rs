// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum PlanDefinitionGoalTargetDetail {
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
}
#[derive(Debug, Clone)]
pub struct PlanDefinitionGoalTarget {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#detail: Option<PlanDefinitionGoalTargetDetail>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#measure: Option<Box<super::super::types::CodeableConcept>>,
    pub r#due: Option<Box<super::super::types::Duration>>,
}
#[derive(Debug, Clone)]
pub struct PlanDefinitionGoal {
    pub r#addresses: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#start: Option<Box<super::super::types::CodeableConcept>>,
    pub r#target: Vec<PlanDefinitionGoalTarget>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#priority: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Box<super::super::types::CodeableConcept>,
    pub r#documentation: Vec<Box<super::super::types::RelatedArtifact>>,
    pub r#id: Option<std::string::String>,
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct PlanDefinitionActionDynamicValue {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#path: Option<super::super::types::String>,
    pub r#expression: Option<Box<super::super::types::Expression>>,
}
#[derive(Debug, Clone)]
pub enum PlanDefinitionActionDefinition {
    Canonical(Box<super::super::types::Canonical>),
    Uri(Box<super::super::types::Uri>),
}
#[derive(Debug, Clone)]
pub struct PlanDefinitionActionParticipant {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: super::super::types::Code,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
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
pub enum PlanDefinitionActionSubject {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct PlanDefinitionActionCondition {
    pub r#kind: super::super::types::Code,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#expression: Option<Box<super::super::types::Expression>>,
}
#[derive(Debug, Clone)]
pub enum PlanDefinitionActionRelatedActionOffset {
    Duration(Box<super::super::types::Duration>),
    Range(Box<super::super::types::Range>),
}
#[derive(Debug, Clone)]
pub struct PlanDefinitionActionRelatedAction {
    pub r#id: Option<std::string::String>,
    pub r#relationship: super::super::types::Code,
    pub r#offset: Option<PlanDefinitionActionRelatedActionOffset>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#action_id: super::super::types::Id,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct PlanDefinitionAction {
    pub r#documentation: Vec<Box<super::super::types::RelatedArtifact>>,
    pub r#text_equivalent: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#input: Vec<Box<super::super::types::DataRequirement>>,
    pub r#grouping_behavior: Option<super::super::types::Code>,
    pub r#dynamic_value: Vec<PlanDefinitionActionDynamicValue>,
    pub r#action: Vec<PlanDefinitionAction>,
    pub r#selection_behavior: Option<super::super::types::Code>,
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#definition: Option<PlanDefinitionActionDefinition>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#cardinality_behavior: Option<super::super::types::Code>,
    pub r#participant: Vec<PlanDefinitionActionParticipant>,
    pub r#timing: Option<PlanDefinitionActionTiming>,
    pub r#priority: Option<super::super::types::Code>,
    pub r#output: Vec<Box<super::super::types::DataRequirement>>,
    pub r#required_behavior: Option<super::super::types::Code>,
    pub r#subject: Option<PlanDefinitionActionSubject>,
    pub r#description: Option<super::super::types::String>,
    pub r#reason: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#goal_id: Vec<super::super::types::Id>,
    pub r#condition: Vec<PlanDefinitionActionCondition>,
    pub r#title: Option<super::super::types::String>,
    pub r#related_action: Vec<PlanDefinitionActionRelatedAction>,
    pub r#trigger: Vec<Box<super::super::types::TriggerDefinition>>,
    pub r#id: Option<std::string::String>,
    pub r#transform: Option<super::super::types::Canonical>,
    pub r#prefix: Option<super::super::types::String>,
    pub r#precheck_behavior: Option<super::super::types::Code>,
}
#[derive(Debug, Clone)]
pub enum PlanDefinitionSubject {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct PlanDefinition {
    pub r#description: Option<super::super::types::Markdown>,
    pub r#language: Option<super::super::types::Code>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#name: Option<super::super::types::String>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#purpose: Option<super::super::types::Markdown>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#status: super::super::types::Code,
    pub r#goal: Vec<PlanDefinitionGoal>,
    pub r#reviewer: Vec<Box<super::super::types::ContactDetail>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#approval_date: Option<super::super::types::Date>,
    pub r#topic: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#usage: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#version: Option<super::super::types::String>,
    pub r#action: Vec<PlanDefinitionAction>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#endorser: Vec<Box<super::super::types::ContactDetail>>,
    pub r#last_review_date: Option<super::super::types::Date>,
    pub r#editor: Vec<Box<super::super::types::ContactDetail>>,
    pub r#related_artifact: Vec<Box<super::super::types::RelatedArtifact>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#title: Option<super::super::types::String>,
    pub r#subtitle: Option<super::super::types::String>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#subject: Option<PlanDefinitionSubject>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#author: Vec<Box<super::super::types::ContactDetail>>,
    pub r#library: Vec<super::super::types::Canonical>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
}
