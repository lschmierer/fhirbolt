// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "A participant in the test execution, either the execution engine, a client, or a server."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestReportParticipant {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The type of participant."]
    pub r#type: super::super::types::Code,
    #[doc = "The uri of the participant. An absolute URL is preferred."]
    pub r#uri: super::super::types::Uri,
    #[doc = "The display name of the participant."]
    pub r#display: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestReportParticipant {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#uri: super::super::types::Uri {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#display: Default::default(),
        }
    }
}
#[doc = "The operation performed."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestReportSetupActionOperation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The result of this operation."]
    pub r#result: super::super::types::Code,
    #[doc = "An explanatory message associated with the result."]
    pub r#message: Option<super::super::types::Markdown>,
    #[doc = "A link to further details on the result."]
    pub r#detail: Option<super::super::types::Uri>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestReportSetupActionOperation {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#result: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#message: Default::default(),
            r#detail: Default::default(),
        }
    }
}
#[doc = "The results of the assertion performed on the previous operations."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestReportSetupActionAssert {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The result of this assertion."]
    pub r#result: super::super::types::Code,
    #[doc = "An explanatory message associated with the result."]
    pub r#message: Option<super::super::types::Markdown>,
    #[doc = "A link to further details on the result."]
    pub r#detail: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestReportSetupActionAssert {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#result: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#message: Default::default(),
            r#detail: Default::default(),
        }
    }
}
#[doc = "Action would contain either an operation or an assertion."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestReportSetupAction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The operation performed."]
    pub r#operation: Option<TestReportSetupActionOperation>,
    #[doc = "The results of the assertion performed on the previous operations."]
    pub r#assert: Option<TestReportSetupActionAssert>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestReportSetupAction {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#operation: Default::default(),
            r#assert: Default::default(),
        }
    }
}
#[doc = "The results of the series of required setup operations before the tests were executed."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestReportSetup {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Action would contain either an operation or an assertion."]
    pub r#action: Vec<TestReportSetupAction>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestReportSetup {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#action: Default::default(),
        }
    }
}
#[doc = "Action would contain either an operation or an assertion."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestReportTestAction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "An operation would involve a REST request to a server."]
    pub r#operation: Option<TestReportSetupActionOperation>,
    #[doc = "The results of the assertion performed on the previous operations."]
    pub r#assert: Option<TestReportSetupActionAssert>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestReportTestAction {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#operation: Default::default(),
            r#assert: Default::default(),
        }
    }
}
#[doc = "A test executed from the test script."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestReportTest {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The name of this test used for tracking/logging purposes by test engines."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short description of the test used by test engines for tracking and reporting purposes."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Action would contain either an operation or an assertion."]
    pub r#action: Vec<TestReportTestAction>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestReportTest {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#name: Default::default(),
            r#description: Default::default(),
            r#action: Default::default(),
        }
    }
}
#[doc = "The teardown action will only contain an operation."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestReportTeardownAction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "An operation would involve a REST request to a server."]
    pub r#operation: TestReportSetupActionOperation,
}
#[allow(clippy::derivable_impls)]
impl Default for TestReportTeardownAction {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#operation: TestReportSetupActionOperation {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "The results of the series of operations required to clean up after all the tests were executed (successfully or otherwise)."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestReportTeardown {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The teardown action will only contain an operation."]
    pub r#action: Vec<TestReportTeardownAction>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestReportTeardown {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#action: Default::default(),
        }
    }
}
#[doc = "A summary of information based on the results of executing a TestScript."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestReport {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<super::super::types::Id>,
    #[doc = "The metadata about the resource. This is content that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource."]
    pub r#meta: Option<Box<super::super::types::Meta>>,
    #[doc = "A reference to a set of rules that were followed when the resource was constructed, and which must be understood when processing the content. Often, this is a reference to an implementation guide that defines the special rules along with other profiles etc."]
    pub r#implicit_rules: Option<super::super::types::Uri>,
    #[doc = "The base language in which the resource is written."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "A human-readable narrative that contains a summary of the resource and can be used to represent the content of the resource to a human. The narrative need not encode all the structured data, but is required to contain sufficient detail to make it \"clinically safe\" for a human to just read the narrative. Resource definitions may define what content should be represented in the narrative to ensure clinical safety."]
    pub r#text: Option<Box<super::super::types::Narrative>>,
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Identifier for the TestScript assigned for external purposes outside the context of FHIR."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "A free text natural language name identifying the executed TestScript."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "The current state of this test report."]
    pub r#status: super::super::types::Code,
    #[doc = "Ideally this is an absolute URL that is used to identify the version-specific TestScript that was executed, matching the `TestScript.url`."]
    pub r#test_script: Box<super::super::types::Reference>,
    #[doc = "The overall result from the execution of the TestScript."]
    pub r#result: super::super::types::Code,
    #[doc = "The final score (percentage of tests passed) resulting from the execution of the TestScript."]
    pub r#score: Option<super::super::types::Decimal>,
    #[doc = "Name of the tester producing this report (Organization or individual)."]
    pub r#tester: Option<super::super::types::String>,
    #[doc = "When the TestScript was executed and this TestReport was generated."]
    pub r#issued: Option<super::super::types::DateTime>,
    #[doc = "A participant in the test execution, either the execution engine, a client, or a server."]
    pub r#participant: Vec<TestReportParticipant>,
    #[doc = "The results of the series of required setup operations before the tests were executed."]
    pub r#setup: Option<TestReportSetup>,
    #[doc = "A test executed from the test script."]
    pub r#test: Vec<TestReportTest>,
    #[doc = "The results of the series of operations required to clean up after all the tests were executed (successfully or otherwise)."]
    pub r#teardown: Option<TestReportTeardown>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestReport {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#meta: Default::default(),
            r#implicit_rules: Default::default(),
            r#language: Default::default(),
            r#text: Default::default(),
            r#contained: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#identifier: Default::default(),
            r#name: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#test_script: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#result: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#score: Default::default(),
            r#tester: Default::default(),
            r#issued: Default::default(),
            r#participant: Default::default(),
            r#setup: Default::default(),
            r#test: Default::default(),
            r#teardown: Default::default(),
        }
    }
}
