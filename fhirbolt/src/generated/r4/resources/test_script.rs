// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct TestScriptTeardownAction {
    pub r#operation: TestScriptSetupActionOperation,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct TestScriptTeardown {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#action: Vec<TestScriptTeardownAction>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct TestScriptDestination {
    pub r#id: Option<std::string::String>,
    pub r#index: super::super::types::Integer,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#profile: Box<super::super::types::Coding>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct TestScriptFixture {
    pub r#resource: Option<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#autocreate: super::super::types::Boolean,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#autodelete: super::super::types::Boolean,
}
#[derive(Debug, Clone)]
pub struct TestScriptSetupActionAssert {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#minimum_id: Option<super::super::types::String>,
    pub r#description: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#request_method: Option<super::super::types::Code>,
    pub r#label: Option<super::super::types::String>,
    pub r#compare_to_source_path: Option<super::super::types::String>,
    pub r#resource: Option<super::super::types::Code>,
    pub r#compare_to_source_expression: Option<super::super::types::String>,
    pub r#compare_to_source_id: Option<super::super::types::String>,
    pub r#expression: Option<super::super::types::String>,
    pub r#navigation_links: Option<super::super::types::Boolean>,
    pub r#path: Option<super::super::types::String>,
    pub r#response_code: Option<super::super::types::String>,
    pub r#direction: Option<super::super::types::Code>,
    pub r#value: Option<super::super::types::String>,
    pub r#validate_profile_id: Option<super::super::types::Id>,
    pub r#request_url: Option<super::super::types::String>,
    pub r#content_type: Option<super::super::types::Code>,
    pub r#header_field: Option<super::super::types::String>,
    pub r#response: Option<super::super::types::Code>,
    pub r#source_id: Option<super::super::types::Id>,
    pub r#operator: Option<super::super::types::Code>,
    pub r#warning_only: super::super::types::Boolean,
}
#[derive(Debug, Clone)]
pub struct TestScriptSetupActionOperationRequestHeader {
    pub r#field: super::super::types::String,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#value: super::super::types::String,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct TestScriptSetupActionOperation {
    pub r#type: Option<Box<super::super::types::Coding>>,
    pub r#label: Option<super::super::types::String>,
    pub r#description: Option<super::super::types::String>,
    pub r#accept: Option<super::super::types::Code>,
    pub r#request_id: Option<super::super::types::Id>,
    pub r#method: Option<super::super::types::Code>,
    pub r#content_type: Option<super::super::types::Code>,
    pub r#resource: Option<super::super::types::Code>,
    pub r#target_id: Option<super::super::types::Id>,
    pub r#destination: Option<super::super::types::Integer>,
    pub r#params: Option<super::super::types::String>,
    pub r#url: Option<super::super::types::String>,
    pub r#response_id: Option<super::super::types::Id>,
    pub r#request_header: Vec<TestScriptSetupActionOperationRequestHeader>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#source_id: Option<super::super::types::Id>,
    pub r#encode_request_url: super::super::types::Boolean,
    pub r#origin: Option<super::super::types::Integer>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct TestScriptSetupAction {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#assert: Option<TestScriptSetupActionAssert>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#operation: Option<TestScriptSetupActionOperation>,
}
#[derive(Debug, Clone)]
pub struct TestScriptSetup {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#action: Vec<TestScriptSetupAction>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct TestScriptMetadataLink {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#url: super::super::types::Uri,
    pub r#description: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct TestScriptMetadataCapability {
    pub r#description: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#destination: Option<super::super::types::Integer>,
    pub r#validated: super::super::types::Boolean,
    pub r#origin: Vec<super::super::types::Integer>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#link: Vec<super::super::types::Uri>,
    pub r#id: Option<std::string::String>,
    pub r#capabilities: super::super::types::Canonical,
    pub r#required: super::super::types::Boolean,
}
#[derive(Debug, Clone)]
pub struct TestScriptMetadata {
    pub r#link: Vec<TestScriptMetadataLink>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#capability: Vec<TestScriptMetadataCapability>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct TestScriptVariable {
    pub r#source_id: Option<super::super::types::Id>,
    pub r#name: super::super::types::String,
    pub r#header_field: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#default_value: Option<super::super::types::String>,
    pub r#description: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#expression: Option<super::super::types::String>,
    pub r#hint: Option<super::super::types::String>,
    pub r#path: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct TestScriptOrigin {
    pub r#profile: Box<super::super::types::Coding>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#index: super::super::types::Integer,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct TestScriptTestAction {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#operation: Option<TestScriptSetupActionOperation>,
    pub r#assert: Option<TestScriptSetupActionAssert>,
}
#[derive(Debug, Clone)]
pub struct TestScriptTest {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#action: Vec<TestScriptTestAction>,
    pub r#name: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct TestScript {
    pub r#version: Option<super::super::types::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#teardown: Option<TestScriptTeardown>,
    pub r#title: Option<super::super::types::String>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#destination: Vec<TestScriptDestination>,
    pub r#fixture: Vec<TestScriptFixture>,
    pub r#setup: Option<TestScriptSetup>,
    pub r#purpose: Option<super::super::types::Markdown>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#metadata: Option<TestScriptMetadata>,
    pub r#variable: Vec<TestScriptVariable>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#profile: Vec<Box<super::super::types::Reference>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#url: super::super::types::Uri,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#id: Option<std::string::String>,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: super::super::types::String,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#origin: Vec<TestScriptOrigin>,
    pub r#status: super::super::types::Code,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#test: Vec<TestScriptTest>,
}
