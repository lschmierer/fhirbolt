// Generated on 2023-05-05 by fhirbolt-codegen v0.8.0
#[doc = "Indicates the mechanism used to compare versions to determine which is more current."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum TestPlanVersionAlgorithm {
    String(Box<super::super::types::String>),
    Coding(Box<super::super::types::Coding>),
    #[default]
    Invalid,
}
#[doc = "The actual content of the cases - references to TestScripts or externally defined content."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum TestPlanTestCaseTestRunScriptSource {
    String(Box<super::super::types::String>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "Pointer to a definition of test resources - narrative or structured e.g. synthetic data generation, etc."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum TestPlanTestCaseTestDataSource {
    String(Box<super::super::types::String>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "The required criteria to execute the test plan - e.g. preconditions, previous tests..."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestPlanDependency {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A textual description of the criterium - what is needed for the dependency to be considered met."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "Predecessor test plans - those that are expected to be successfully performed as a dependency for the execution of this test plan."]
    pub r#predecessor: Option<Box<super::super::types::Reference>>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestPlanDependency {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#description: Default::default(),
            r#predecessor: Default::default(),
        }
    }
}
#[doc = "The required criteria to execute the test case - e.g. preconditions, previous tests."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestPlanTestCaseDependency {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Description of the criteria."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "Link to predecessor test plans."]
    pub r#predecessor: Option<Box<super::super::types::Reference>>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestPlanTestCaseDependency {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#description: Default::default(),
            r#predecessor: Default::default(),
        }
    }
}
#[doc = "The test cases in a structured language e.g. gherkin, Postman, or FHIR TestScript."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestPlanTestCaseTestRunScript {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The language for the test cases e.g. 'gherkin', 'testscript'."]
    pub r#language: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The actual content of the cases - references to TestScripts or externally defined content."]
    pub r#source: Option<TestPlanTestCaseTestRunScriptSource>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestPlanTestCaseTestRunScript {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#language: Default::default(),
            r#source: Default::default(),
        }
    }
}
#[doc = "The actual test to be executed."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestPlanTestCaseTestRun {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The narrative description of the tests."]
    pub r#narrative: Option<super::super::types::Markdown>,
    #[doc = "The test cases in a structured language e.g. gherkin, Postman, or FHIR TestScript."]
    pub r#script: Option<TestPlanTestCaseTestRunScript>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestPlanTestCaseTestRun {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#narrative: Default::default(),
            r#script: Default::default(),
        }
    }
}
#[doc = "The test data used in the test case."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestPlanTestCaseTestData {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The type of test data description, e.g. 'synthea'."]
    pub r#type: Box<super::super::types::Coding>,
    #[doc = "The actual test resources when they exist."]
    pub r#content: Option<Box<super::super::types::Reference>>,
    #[doc = "Pointer to a definition of test resources - narrative or structured e.g. synthetic data generation, etc."]
    pub r#source: Option<TestPlanTestCaseTestDataSource>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestPlanTestCaseTestData {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Box::new(super::super::types::Coding {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#content: Default::default(),
            r#source: Default::default(),
        }
    }
}
#[doc = "The test assertions - the expectations of test results from the execution of the test case."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestPlanTestCaseAssertion {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The test assertion type - this can be used to group assertions as 'required' or 'optional', or can be used for other classification of the assertion."]
    pub r#type: Vec<super::super::types::CodeableConcept>,
    #[doc = "The focus or object of the assertion i.e. a resource."]
    pub r#object: Vec<super::super::types::CodeableReference>,
    #[doc = "The test assertion - the expected outcome from the test case execution."]
    pub r#result: Vec<super::super::types::CodeableReference>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestPlanTestCaseAssertion {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#object: Default::default(),
            r#result: Default::default(),
        }
    }
}
#[doc = "The individual test cases that are part of this plan, when they they are made explicit."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestPlanTestCase {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Sequence of test case - an ordinal number that indicates the order for the present test case in the test plan."]
    pub r#sequence: Option<super::super::types::Integer>,
    #[doc = "The scope or artifact covered by the case, when the individual test case is associated with a testable artifact."]
    pub r#scope: Vec<super::super::types::Reference>,
    #[doc = "The required criteria to execute the test case - e.g. preconditions, previous tests."]
    pub r#dependency: Vec<TestPlanTestCaseDependency>,
    #[doc = "The actual test to be executed."]
    pub r#test_run: Vec<TestPlanTestCaseTestRun>,
    #[doc = "The test data used in the test case."]
    pub r#test_data: Vec<TestPlanTestCaseTestData>,
    #[doc = "The test assertions - the expectations of test results from the execution of the test case."]
    pub r#assertion: Vec<TestPlanTestCaseAssertion>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestPlanTestCase {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#sequence: Default::default(),
            r#scope: Default::default(),
            r#dependency: Default::default(),
            r#test_run: Default::default(),
            r#test_data: Default::default(),
            r#assertion: Default::default(),
        }
    }
}
#[doc = "A plan for executing testing on an artifact or specifications"]
#[derive(Debug, Clone, PartialEq)]
pub struct TestPlan {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<Box<super::super::types::Id>>,
    #[doc = "The metadata about the resource. This is content that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource."]
    pub r#meta: Option<Box<super::super::types::Meta>>,
    #[doc = "A reference to a set of rules that were followed when the resource was constructed, and which must be understood when processing the content. Often, this is a reference to an implementation guide that defines the special rules along with other profiles etc."]
    pub r#implicit_rules: Option<super::super::types::Uri>,
    #[doc = "The base language in which the resource is written."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "A human-readable narrative that contains a summary of the resource and can be used to represent the content of the resource to a human. The narrative need not encode all the structured data, but is required to contain sufficient detail to make it \"clinically safe\" for a human to just read the narrative. Resource definitions may define what content should be represented in the narrative to ensure clinical safety."]
    pub r#text: Option<Box<super::super::types::Narrative>>,
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, nor can they have their own independent transaction scope. This is allowed to be a Parameters resource if and only if it is referenced by a resource that provides context/meaning."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "An absolute URI that is used to identify this test plan when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which an authoritative instance of this test plan is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the test plan is stored on different servers."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "A formal identifier that is used to identify this test plan when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The identifier that is used to identify this version of the test plan when it is referenced in a specification, model, design or instance.  This is an arbitrary value managed by the test plan author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "Indicates the mechanism used to compare versions to determine which is more current."]
    pub r#version_algorithm: Option<TestPlanVersionAlgorithm>,
    #[doc = "A natural language name identifying the test plan. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for the test plan."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The status of this test plan. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this test plan is authored for testing purposes (or education/evaluation/marketing) and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "The date (and optionally time) when the test plan was last significantly changed. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the test plan changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual responsible for the release and ongoing maintenance of the test plan."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<super::super::types::ContactDetail>,
    #[doc = "A free text natural language description of the test plan from a consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate test plan instances."]
    pub r#use_context: Vec<super::super::types::UsageContext>,
    #[doc = "A legal or geographic region in which the test plan is intended to be used."]
    pub r#jurisdiction: Vec<super::super::types::CodeableConcept>,
    #[doc = "Explanation of why this test plan is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "A copyright statement relating to the test plan and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the test plan. The short copyright declaration (e.g. (c) '2015+ xyz organization' should be sent in the copyrightLabel element."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "A short string (<50 characters), suitable for inclusion in a page footer that identifies the copyright holder, effective period, and optionally whether rights are resctricted. (e.g. 'All rights reserved', 'Some rights reserved')."]
    pub r#copyright_label: Option<super::super::types::String>,
    #[doc = "The category of the Test Plan - can be acceptance, unit, performance, etc."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "What is being tested with this Test Plan - a conformance resource, or narrative criteria, or an external reference..."]
    pub r#scope: Vec<super::super::types::Reference>,
    #[doc = "A description of test tools to be used in the test plan."]
    pub r#test_tools: Option<super::super::types::Markdown>,
    #[doc = "The required criteria to execute the test plan - e.g. preconditions, previous tests..."]
    pub r#dependency: Vec<TestPlanDependency>,
    #[doc = "The threshold or criteria for the test plan to be considered successfully executed - narrative."]
    pub r#exit_criteria: Option<super::super::types::Markdown>,
    #[doc = "The individual test cases that are part of this plan, when they they are made explicit."]
    pub r#test_case: Vec<TestPlanTestCase>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestPlan {
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
            r#url: Default::default(),
            r#identifier: Default::default(),
            r#version: Default::default(),
            r#version_algorithm: Default::default(),
            r#name: Default::default(),
            r#title: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#experimental: Default::default(),
            r#date: Default::default(),
            r#publisher: Default::default(),
            r#contact: Default::default(),
            r#description: Default::default(),
            r#use_context: Default::default(),
            r#jurisdiction: Default::default(),
            r#purpose: Default::default(),
            r#copyright: Default::default(),
            r#copyright_label: Default::default(),
            r#category: Default::default(),
            r#scope: Default::default(),
            r#test_tools: Default::default(),
            r#dependency: Default::default(),
            r#exit_criteria: Default::default(),
            r#test_case: Default::default(),
        }
    }
}
