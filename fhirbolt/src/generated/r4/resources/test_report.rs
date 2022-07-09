// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct TestReportParticipant {
    pub r#uri: super::super::types::Uri,
    pub r#display: Option<super::super::types::String>,
    pub r#type: super::super::types::Code,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct TestReportSetupActionOperation {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#message: Option<super::super::types::Markdown>,
    pub r#detail: Option<super::super::types::Uri>,
    pub r#result: super::super::types::Code,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct TestReportSetupActionAssert {
    pub r#result: super::super::types::Code,
    pub r#detail: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#message: Option<super::super::types::Markdown>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct TestReportSetupAction {
    pub r#operation: Option<TestReportSetupActionOperation>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#assert: Option<TestReportSetupActionAssert>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct TestReportSetup {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#action: Vec<TestReportSetupAction>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct TestReportTestAction {
    pub r#id: Option<std::string::String>,
    pub r#operation: Option<TestReportSetupActionOperation>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#assert: Option<TestReportSetupActionAssert>,
}
#[derive(Debug, Clone)]
pub struct TestReportTest {
    pub r#description: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#action: Vec<TestReportTestAction>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct TestReportTeardownAction {
    pub r#id: Option<std::string::String>,
    pub r#operation: TestReportSetupActionOperation,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct TestReportTeardown {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#action: Vec<TestReportTeardownAction>,
}
#[derive(Debug, Clone)]
pub struct TestReport {
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#test_script: Box<super::super::types::Reference>,
    pub r#result: super::super::types::Code,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#participant: Vec<TestReportParticipant>,
    pub r#setup: Option<TestReportSetup>,
    pub r#id: Option<std::string::String>,
    pub r#name: Option<super::super::types::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#test: Vec<TestReportTest>,
    pub r#status: super::super::types::Code,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#teardown: Option<TestReportTeardown>,
    pub r#tester: Option<super::super::types::String>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#score: Option<super::super::types::Decimal>,
    pub r#issued: Option<super::super::types::DateTime>,
    pub r#language: Option<super::super::types::Code>,
}
