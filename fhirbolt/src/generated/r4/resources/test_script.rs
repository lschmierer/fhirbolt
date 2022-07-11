// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct TestScriptOrigin {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#profile: Box<super::super::types::Coding>,
    pub r#index: super::super::types::Integer,
}
#[derive(Debug, Clone)]
pub struct TestScriptDestination {
    pub r#index: super::super::types::Integer,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#profile: Box<super::super::types::Coding>,
}
#[derive(Debug, Clone)]
pub struct TestScriptSetupActionAssert {
    pub r#source_id: Option<super::super::types::Id>,
    pub r#validate_profile_id: Option<super::super::types::Id>,
    pub r#value: Option<super::super::types::String>,
    pub r#warning_only: super::super::types::Boolean,
    pub r#compare_to_source_id: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#operator: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#compare_to_source_expression: Option<super::super::types::String>,
    pub r#header_field: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#navigation_links: Option<super::super::types::Boolean>,
    pub r#resource: Option<super::super::types::Code>,
    pub r#label: Option<super::super::types::String>,
    pub r#response: Option<super::super::types::Code>,
    pub r#direction: Option<super::super::types::Code>,
    pub r#minimum_id: Option<super::super::types::String>,
    pub r#content_type: Option<super::super::types::Code>,
    pub r#description: Option<super::super::types::String>,
    pub r#request_method: Option<super::super::types::Code>,
    pub r#compare_to_source_path: Option<super::super::types::String>,
    pub r#expression: Option<super::super::types::String>,
    pub r#request_url: Option<super::super::types::String>,
    pub r#path: Option<super::super::types::String>,
    pub r#response_code: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct TestScriptSetupActionOperationRequestHeader {
    pub r#field: super::super::types::String,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#value: super::super::types::String,
}
#[derive(Debug, Clone)]
pub struct TestScriptSetupActionOperation {
    pub r#method: Option<super::super::types::Code>,
    pub r#encode_request_url: super::super::types::Boolean,
    pub r#url: Option<super::super::types::String>,
    pub r#destination: Option<super::super::types::Integer>,
    pub r#origin: Option<super::super::types::Integer>,
    pub r#description: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#content_type: Option<super::super::types::Code>,
    pub r#request_id: Option<super::super::types::Id>,
    pub r#response_id: Option<super::super::types::Id>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#source_id: Option<super::super::types::Id>,
    pub r#type: Option<Box<super::super::types::Coding>>,
    pub r#id: Option<std::string::String>,
    pub r#accept: Option<super::super::types::Code>,
    pub r#params: Option<super::super::types::String>,
    pub r#label: Option<super::super::types::String>,
    pub r#request_header: Vec<TestScriptSetupActionOperationRequestHeader>,
    pub r#target_id: Option<super::super::types::Id>,
    pub r#resource: Option<super::super::types::Code>,
}
#[derive(Debug, Clone)]
pub struct TestScriptSetupAction {
    pub r#assert: Option<TestScriptSetupActionAssert>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#operation: Option<TestScriptSetupActionOperation>,
}
#[derive(Debug, Clone)]
pub struct TestScriptSetup {
    pub r#id: Option<std::string::String>,
    pub r#action: Vec<TestScriptSetupAction>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct TestScriptFixture {
    pub r#autocreate: super::super::types::Boolean,
    pub r#resource: Option<Box<super::super::types::Reference>>,
    pub r#autodelete: super::super::types::Boolean,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct TestScriptTestAction {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#assert: Option<TestScriptSetupActionAssert>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#operation: Option<TestScriptSetupActionOperation>,
}
#[derive(Debug, Clone)]
pub struct TestScriptTest {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#action: Vec<TestScriptTestAction>,
    pub r#description: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct TestScriptMetadataCapability {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#required: super::super::types::Boolean,
    pub r#id: Option<std::string::String>,
    pub r#link: Vec<super::super::types::Uri>,
    pub r#description: Option<super::super::types::String>,
    pub r#destination: Option<super::super::types::Integer>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#capabilities: super::super::types::Canonical,
    pub r#origin: Vec<super::super::types::Integer>,
    pub r#validated: super::super::types::Boolean,
}
#[derive(Debug, Clone)]
pub struct TestScriptMetadataLink {
    pub r#url: super::super::types::Uri,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct TestScriptMetadata {
    pub r#capability: Vec<TestScriptMetadataCapability>,
    pub r#id: Option<std::string::String>,
    pub r#link: Vec<TestScriptMetadataLink>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct TestScriptVariable {
    pub r#expression: Option<super::super::types::String>,
    pub r#source_id: Option<super::super::types::Id>,
    pub r#description: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#path: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: super::super::types::String,
    pub r#header_field: Option<super::super::types::String>,
    pub r#default_value: Option<super::super::types::String>,
    pub r#hint: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct TestScriptTeardownAction {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#operation: TestScriptSetupActionOperation,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct TestScriptTeardown {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#action: Vec<TestScriptTeardownAction>,
}
#[derive(Debug, Clone)]
pub struct TestScript {
    pub r#url: super::super::types::Uri,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#profile: Vec<Box<super::super::types::Reference>>,
    pub r#status: super::super::types::Code,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#purpose: Option<super::super::types::Markdown>,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#origin: Vec<TestScriptOrigin>,
    pub r#id: Option<std::string::String>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#destination: Vec<TestScriptDestination>,
    pub r#setup: Option<TestScriptSetup>,
    pub r#fixture: Vec<TestScriptFixture>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#test: Vec<TestScriptTest>,
    pub r#metadata: Option<TestScriptMetadata>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#name: super::super::types::String,
    pub r#publisher: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#version: Option<super::super::types::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#title: Option<super::super::types::String>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#variable: Vec<TestScriptVariable>,
    pub r#teardown: Option<TestScriptTeardown>,
}
