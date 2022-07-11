// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct TestReportSetupActionOperation {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#message: Option<super::super::types::Markdown>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#detail: Option<super::super::types::Uri>,
    pub r#result: super::super::types::Code,
}
#[derive(Debug, Clone)]
pub struct TestReportSetupActionAssert {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#message: Option<super::super::types::Markdown>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#result: super::super::types::Code,
    pub r#detail: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct TestReportSetupAction {
    pub r#id: Option<std::string::String>,
    pub r#operation: Option<TestReportSetupActionOperation>,
    pub r#assert: Option<TestReportSetupActionAssert>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct TestReportSetup {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#action: Vec<TestReportSetupAction>,
}
#[derive(Debug, Clone)]
pub struct TestReportTestAction {
    pub r#assert: Option<TestReportSetupActionAssert>,
    pub r#operation: Option<TestReportSetupActionOperation>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct TestReportTest {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#name: Option<super::super::types::String>,
    pub r#description: Option<super::super::types::String>,
    pub r#action: Vec<TestReportTestAction>,
}
#[derive(Debug, Clone)]
pub struct TestReportParticipant {
    pub r#type: super::super::types::Code,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#uri: super::super::types::Uri,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#display: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct TestReportTeardownAction {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#operation: TestReportSetupActionOperation,
}
#[derive(Debug, Clone)]
pub struct TestReportTeardown {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#action: Vec<TestReportTeardownAction>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct TestReport {
    pub r#issued: Option<super::super::types::DateTime>,
    pub r#setup: Option<TestReportSetup>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#test_script: Box<super::super::types::Reference>,
    pub r#status: super::super::types::Code,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#test: Vec<TestReportTest>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#participant: Vec<TestReportParticipant>,
    pub r#teardown: Option<TestReportTeardown>,
    pub r#result: super::super::types::Code,
    pub r#name: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#score: Option<super::super::types::Decimal>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#tester: Option<super::super::types::String>,
}
