// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct ExampleScenarioActor {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: Option<super::super::types::String>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#actor_id: super::super::types::String,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: super::super::types::Code,
}
#[derive(Debug, Clone)]
pub struct ExampleScenarioProcessStepOperation {
    pub r#description: Option<super::super::types::Markdown>,
    pub r#request: Option<ExampleScenarioInstanceContainedInstance>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: Option<super::super::types::String>,
    pub r#number: super::super::types::String,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<super::super::types::String>,
    pub r#initiator: Option<super::super::types::String>,
    pub r#receiver: Option<super::super::types::String>,
    pub r#initiator_active: Option<super::super::types::Boolean>,
    pub r#response: Option<ExampleScenarioInstanceContainedInstance>,
    pub r#receiver_active: Option<super::super::types::Boolean>,
}
#[derive(Debug, Clone)]
pub struct ExampleScenarioProcessStepAlternative {
    pub r#id: Option<std::string::String>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#title: super::super::types::String,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#step: Vec<ExampleScenarioProcessStep>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ExampleScenarioProcessStep {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#process: Vec<ExampleScenarioProcess>,
    pub r#pause: Option<super::super::types::Boolean>,
    pub r#operation: Option<ExampleScenarioProcessStepOperation>,
    pub r#alternative: Vec<ExampleScenarioProcessStepAlternative>,
}
#[derive(Debug, Clone)]
pub struct ExampleScenarioProcess {
    pub r#title: super::super::types::String,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#step: Vec<ExampleScenarioProcessStep>,
    pub r#id: Option<std::string::String>,
    pub r#post_conditions: Option<super::super::types::Markdown>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#pre_conditions: Option<super::super::types::Markdown>,
}
#[derive(Debug, Clone)]
pub struct ExampleScenarioInstanceContainedInstance {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#version_id: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#resource_id: super::super::types::String,
}
#[derive(Debug, Clone)]
pub struct ExampleScenarioInstanceVersion {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#version_id: super::super::types::String,
    pub r#description: super::super::types::Markdown,
}
#[derive(Debug, Clone)]
pub struct ExampleScenarioInstance {
    pub r#resource_id: super::super::types::String,
    pub r#contained_instance: Vec<ExampleScenarioInstanceContainedInstance>,
    pub r#resource_type: super::super::types::Code,
    pub r#version: Vec<ExampleScenarioInstanceVersion>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#id: Option<std::string::String>,
    pub r#name: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ExampleScenario {
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#id: Option<std::string::String>,
    pub r#status: super::super::types::Code,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#purpose: Option<super::super::types::Markdown>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#version: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#name: Option<super::super::types::String>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#actor: Vec<ExampleScenarioActor>,
    pub r#process: Vec<ExampleScenarioProcess>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#instance: Vec<ExampleScenarioInstance>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#workflow: Vec<super::super::types::Canonical>,
}
