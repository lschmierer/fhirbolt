// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct ExampleScenarioActor {
    pub r#name: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#actor_id: super::super::types::String,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: super::super::types::Code,
}
#[derive(Debug, Clone)]
pub struct ExampleScenarioInstanceContainedInstance {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#resource_id: super::super::types::String,
    pub r#version_id: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct ExampleScenarioInstanceVersion {
    pub r#version_id: super::super::types::String,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: super::super::types::Markdown,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ExampleScenarioInstance {
    pub r#resource_type: super::super::types::Code,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: Option<super::super::types::String>,
    pub r#contained_instance: Vec<ExampleScenarioInstanceContainedInstance>,
    pub r#version: Vec<ExampleScenarioInstanceVersion>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#resource_id: super::super::types::String,
}
#[derive(Debug, Clone)]
pub struct ExampleScenarioProcessStepOperation {
    pub r#name: Option<super::super::types::String>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#type: Option<super::super::types::String>,
    pub r#initiator_active: Option<super::super::types::Boolean>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#response: Option<ExampleScenarioInstanceContainedInstance>,
    pub r#receiver_active: Option<super::super::types::Boolean>,
    pub r#request: Option<ExampleScenarioInstanceContainedInstance>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#receiver: Option<super::super::types::String>,
    pub r#initiator: Option<super::super::types::String>,
    pub r#number: super::super::types::String,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct ExampleScenarioProcessStepAlternative {
    pub r#description: Option<super::super::types::Markdown>,
    pub r#step: Vec<ExampleScenarioProcessStep>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#title: super::super::types::String,
}
#[derive(Debug, Clone)]
pub struct ExampleScenarioProcessStep {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#process: Vec<ExampleScenarioProcess>,
    pub r#pause: Option<super::super::types::Boolean>,
    pub r#operation: Option<ExampleScenarioProcessStepOperation>,
    pub r#alternative: Vec<ExampleScenarioProcessStepAlternative>,
}
#[derive(Debug, Clone)]
pub struct ExampleScenarioProcess {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#post_conditions: Option<super::super::types::Markdown>,
    pub r#step: Vec<ExampleScenarioProcessStep>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#title: super::super::types::String,
    pub r#pre_conditions: Option<super::super::types::Markdown>,
}
#[derive(Debug, Clone)]
pub struct ExampleScenario {
    pub r#workflow: Vec<super::super::types::Canonical>,
    pub r#actor: Vec<ExampleScenarioActor>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#purpose: Option<super::super::types::Markdown>,
    pub r#instance: Vec<ExampleScenarioInstance>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#process: Vec<ExampleScenarioProcess>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#status: super::super::types::Code,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#version: Option<super::super::types::String>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#experimental: Option<super::super::types::Boolean>,
}
