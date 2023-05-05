// Generated on 2023-05-05 by fhirbolt-codegen v0.8.0
#[doc = "Indicates the mechanism used to compare versions to determine which is more current."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum TestScriptVersionAlgorithm {
    String(Box<super::super::types::String>),
    Coding(Box<super::super::types::Coding>),
    #[default]
    Invalid,
}
#[doc = "Link or reference providing traceability to the testing requirement for this test."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum TestScriptSetupActionAssertRequirementLink {
    Uri(Box<super::super::types::Uri>),
    Canonical(Box<super::super::types::Canonical>),
    #[default]
    Invalid,
}
#[doc = "An abstract server used in operations within this test script in the origin element."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptOrigin {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Abstract name given to an origin server in this test script.  The name is provided as a number starting at 1."]
    pub r#index: super::super::types::Integer,
    #[doc = "The type of origin profile the test system supports."]
    pub r#profile: Box<super::super::types::Coding>,
    #[doc = "The explicit url path of the origin server used in this test script."]
    pub r#url: Option<super::super::types::Url>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestScriptOrigin {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#index: super::super::types::Integer {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#profile: Box::new(super::super::types::Coding {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#url: Default::default(),
        }
    }
}
#[doc = "An abstract server used in operations within this test script in the destination element."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptDestination {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Abstract name given to a destination server in this test script.  The name is provided as a number starting at 1."]
    pub r#index: super::super::types::Integer,
    #[doc = "The type of destination profile the test system supports."]
    pub r#profile: Box<super::super::types::Coding>,
    #[doc = "The explicit url path of the destination server used in this test script."]
    pub r#url: Option<super::super::types::Url>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestScriptDestination {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#index: super::super::types::Integer {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#profile: Box::new(super::super::types::Coding {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#url: Default::default(),
        }
    }
}
#[doc = "A link to the FHIR specification that this test is covering."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptMetadataLink {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "URL to a particular requirement or feature within the FHIR specification."]
    pub r#url: super::super::types::Uri,
    #[doc = "Short description of the link."]
    pub r#description: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestScriptMetadataLink {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#url: super::super::types::Uri {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#description: Default::default(),
        }
    }
}
#[doc = "Capabilities that must exist and are assumed to function correctly on the FHIR server being tested."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptMetadataCapability {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Whether or not the test execution will require the given capabilities of the server in order for this test script to execute."]
    pub r#required: super::super::types::Boolean,
    #[doc = "Whether or not the test execution will validate the given capabilities of the server in order for this test script to execute."]
    pub r#validated: super::super::types::Boolean,
    #[doc = "Description of the capabilities that this test script is requiring the server to support."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Which origin server these requirements apply to."]
    pub r#origin: Vec<super::super::types::Integer>,
    #[doc = "Which server these requirements apply to."]
    pub r#destination: Option<super::super::types::Integer>,
    #[doc = "Links to the FHIR specification that describes this interaction and the resources involved in more detail."]
    pub r#link: Vec<super::super::types::Uri>,
    #[doc = "Minimum capabilities required of server for test script to execute successfully.   If server does not meet at a minimum the referenced capability statement, then all tests in this script are skipped."]
    pub r#capabilities: super::super::types::Canonical,
}
#[allow(clippy::derivable_impls)]
impl Default for TestScriptMetadataCapability {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#required: super::super::types::Boolean {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#validated: super::super::types::Boolean {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#description: Default::default(),
            r#origin: Default::default(),
            r#destination: Default::default(),
            r#link: Default::default(),
            r#capabilities: super::super::types::Canonical {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "The required capability must exist and are assumed to function correctly on the FHIR server being tested."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptMetadata {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A link to the FHIR specification that this test is covering."]
    pub r#link: Vec<TestScriptMetadataLink>,
    #[doc = "Capabilities that must exist and are assumed to function correctly on the FHIR server being tested."]
    pub r#capability: Vec<TestScriptMetadataCapability>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestScriptMetadata {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#link: Default::default(),
            r#capability: Default::default(),
        }
    }
}
#[doc = "The scope indicates a conformance artifact that is tested by the test(s) within this test case and the expectation of the test outcome(s) as well as the intended test phase inclusion."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptScope {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The specific conformance artifact being tested. The canonical reference can be version-specific."]
    pub r#artifact: super::super::types::Canonical,
    #[doc = "The expectation of whether the test must pass for the system to be considered conformant with the artifact: required - all tests are expected to pass, optional - all test are expected to pass but non-pass status may be allowed, strict - all tests are expected to pass and warnings are treated as a failure."]
    pub r#conformance: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The phase of testing for this artifact: unit - development / implementation phase, integration - internal system to system phase, production - live system to system phase (Note, this may involve pii/phi data)."]
    pub r#phase: Option<Box<super::super::types::CodeableConcept>>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestScriptScope {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#artifact: super::super::types::Canonical {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#conformance: Default::default(),
            r#phase: Default::default(),
        }
    }
}
#[doc = "Fixture in the test script - by reference (uri). All fixtures are required for the test script to execute."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptFixture {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Whether or not to implicitly create the fixture during setup. If true, the fixture is automatically created on each server being tested during setup, therefore no create operation is required for this fixture in the TestScript.setup section."]
    pub r#autocreate: super::super::types::Boolean,
    #[doc = "Whether or not to implicitly delete the fixture during teardown. If true, the fixture is automatically deleted on each server being tested during teardown, therefore no delete operation is required for this fixture in the TestScript.teardown section."]
    pub r#autodelete: super::super::types::Boolean,
    #[doc = "Reference to the resource (containing the contents of the resource needed for operations). This is allowed to be a Parameters resource."]
    pub r#resource: Option<Box<super::super::types::Reference>>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestScriptFixture {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#autocreate: super::super::types::Boolean {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#autodelete: super::super::types::Boolean {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#resource: Default::default(),
        }
    }
}
#[doc = "Variable is set based either on element value in response body or on header field value in the response headers."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptVariable {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Descriptive name for this variable."]
    pub r#name: super::super::types::String,
    #[doc = "A default, hard-coded, or user-defined value for this variable."]
    pub r#default_value: Option<super::super::types::String>,
    #[doc = "A free text natural language description of the variable and its purpose."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "The FHIRPath expression for a specific value to evaluate against the fixture body. When variables are defined, only one of either expression, headerField or path must be specified."]
    pub r#expression: Option<super::super::types::String>,
    #[doc = "Will be used to grab the HTTP header field value from the headers that sourceId is pointing to."]
    pub r#header_field: Option<super::super::types::String>,
    #[doc = "Displayable text string with hint help information to the user when entering a default value."]
    pub r#hint: Option<super::super::types::String>,
    #[doc = "XPath or JSONPath to evaluate against the fixture body.  When variables are defined, only one of either expression, headerField or path must be specified."]
    pub r#path: Option<super::super::types::String>,
    #[doc = "Fixture to evaluate the XPath/JSONPath expression or the headerField  against within this variable."]
    pub r#source_id: Option<super::super::types::Id>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestScriptVariable {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#name: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#default_value: Default::default(),
            r#description: Default::default(),
            r#expression: Default::default(),
            r#header_field: Default::default(),
            r#hint: Default::default(),
            r#path: Default::default(),
            r#source_id: Default::default(),
        }
    }
}
#[doc = "Header elements would be used to set HTTP headers."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptSetupActionOperationRequestHeader {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The HTTP header field e.g. \"Accept\"."]
    pub r#field: super::super::types::String,
    #[doc = "The value of the header e.g. \"application/fhir+xml\"."]
    pub r#value: super::super::types::String,
}
#[allow(clippy::derivable_impls)]
impl Default for TestScriptSetupActionOperationRequestHeader {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#field: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#value: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "The operation to perform."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptSetupActionOperation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Server interaction or operation type."]
    pub r#type: Option<Box<super::super::types::Coding>>,
    #[doc = "The type of the FHIR resource. See the [resource list](https://hl7.org/FHIR/resourcelist.html)). Data type of uri is needed when non-HL7 artifacts are identified."]
    pub r#resource: Option<super::super::types::Uri>,
    #[doc = "The label would be used for tracking/logging purposes by test engines."]
    pub r#label: Option<super::super::types::String>,
    #[doc = "The description would be used by test engines for tracking and reporting purposes."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "The mime-type to use for RESTful operation in the 'Accept' header."]
    pub r#accept: Option<super::super::types::Code>,
    #[doc = "The mime-type to use for RESTful operation in the 'Content-Type' header."]
    pub r#content_type: Option<super::super::types::Code>,
    #[doc = "The server where the request message is destined for.  Must be one of the server numbers listed in TestScript.destination section."]
    pub r#destination: Option<super::super::types::Integer>,
    #[doc = "Whether or not to implicitly send the request url in encoded format. The default is true to match the standard RESTful client behavior. Set to false when communicating with a server that does not support encoded url paths."]
    pub r#encode_request_url: super::super::types::Boolean,
    #[doc = "The HTTP method the test engine MUST use for this operation regardless of any other operation details."]
    pub r#method: Option<super::super::types::Code>,
    #[doc = "The server where the request message originates from.  Must be one of the server numbers listed in TestScript.origin section."]
    pub r#origin: Option<super::super::types::Integer>,
    #[doc = "Path plus parameters after `type`.  Used to set parts of the request URL explicitly."]
    pub r#params: Option<super::super::types::String>,
    #[doc = "Header elements would be used to set HTTP headers."]
    pub r#request_header: Vec<TestScriptSetupActionOperationRequestHeader>,
    #[doc = "The fixture id (maybe new) to map to the request."]
    pub r#request_id: Option<super::super::types::Id>,
    #[doc = "The fixture id (maybe new) to map to the response."]
    pub r#response_id: Option<super::super::types::Id>,
    #[doc = "The id of the fixture used as the body of a PUT or POST request."]
    pub r#source_id: Option<super::super::types::Id>,
    #[doc = "Id of fixture used for extracting the `id`,  `type`, and `vid` for GET requests."]
    pub r#target_id: Option<super::super::types::Id>,
    #[doc = "Complete request URL."]
    pub r#url: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestScriptSetupActionOperation {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#resource: Default::default(),
            r#label: Default::default(),
            r#description: Default::default(),
            r#accept: Default::default(),
            r#content_type: Default::default(),
            r#destination: Default::default(),
            r#encode_request_url: super::super::types::Boolean {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#method: Default::default(),
            r#origin: Default::default(),
            r#params: Default::default(),
            r#request_header: Default::default(),
            r#request_id: Default::default(),
            r#response_id: Default::default(),
            r#source_id: Default::default(),
            r#target_id: Default::default(),
            r#url: Default::default(),
        }
    }
}
#[doc = "Links or references providing traceability to the testing requirements for this assert."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptSetupActionAssertRequirement {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Link or reference providing traceability to the testing requirement for this test."]
    pub r#link: Option<TestScriptSetupActionAssertRequirementLink>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestScriptSetupActionAssertRequirement {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#link: Default::default(),
        }
    }
}
#[doc = "Evaluates the results of previous operations to determine if the server under test behaves appropriately."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptSetupActionAssert {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The label would be used for tracking/logging purposes by test engines."]
    pub r#label: Option<super::super::types::String>,
    #[doc = "The description would be used by test engines for tracking and reporting purposes."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "The direction to use for the assertion."]
    pub r#direction: Option<super::super::types::Code>,
    #[doc = "Id of the source fixture used as the contents to be evaluated by either the \"source/expression\" or \"sourceId/path\" definition."]
    pub r#compare_to_source_id: Option<super::super::types::String>,
    #[doc = "The FHIRPath expression for a specific value to evaluate against the source fixture. When compareToSourceId is defined, either compareToSourceExpression or compareToSourcePath must be defined, but not both."]
    pub r#compare_to_source_expression: Option<super::super::types::String>,
    #[doc = "XPath or JSONPath expression to evaluate against the source fixture. When compareToSourceId is defined, either compareToSourceExpression or compareToSourcePath must be defined, but not both."]
    pub r#compare_to_source_path: Option<super::super::types::String>,
    #[doc = "The mime-type contents to compare against the request or response message 'Content-Type' header."]
    pub r#content_type: Option<super::super::types::Code>,
    #[doc = "The default manual completion outcome applied to this assertion."]
    pub r#default_manual_completion: Option<super::super::types::Code>,
    #[doc = "The FHIRPath expression to be evaluated against the request or response message contents - HTTP headers and payload."]
    pub r#expression: Option<super::super::types::String>,
    #[doc = "The HTTP header field name e.g. 'Location'."]
    pub r#header_field: Option<super::super::types::String>,
    #[doc = "The ID of a fixture. Asserts that the response contains at a minimum the fixture specified by minimumId."]
    pub r#minimum_id: Option<super::super::types::String>,
    #[doc = "Whether or not the test execution performs validation on the bundle navigation links."]
    pub r#navigation_links: Option<super::super::types::Boolean>,
    #[doc = "The operator type defines the conditional behavior of the assert."]
    pub r#operator: Option<super::super::types::Code>,
    #[doc = "The XPath or JSONPath expression to be evaluated against the fixture representing the response received from server."]
    pub r#path: Option<super::super::types::String>,
    #[doc = "The request method or HTTP operation code to compare against that used by the client system under test."]
    pub r#request_method: Option<super::super::types::Code>,
    #[doc = "The value to use in a comparison against the request URL path string."]
    pub r#request_url: Option<super::super::types::String>,
    #[doc = "The type of the resource.  See the [resource list](https://hl7.org/FHIR/resourcelist.html))."]
    pub r#resource: Option<super::super::types::Uri>,
    #[doc = "continue | switchingProtocols | okay | created | accepted | nonAuthoritativeInformation | noContent | resetContent | partialContent | multipleChoices | movedPermanently | found | seeOther | notModified | useProxy | temporaryRedirect | permanentRedirect | badRequest | unauthorized | paymentRequired | forbidden | notFound | methodNotAllowed | notAcceptable | proxyAuthenticationRequired | requestTimeout | conflict | gone | lengthRequired | preconditionFailed | contentTooLarge | uriTooLong | unsupportedMediaType | rangeNotSatisfiable | expectationFailed | misdirectedRequest | unprocessableContent | upgradeRequired | internalServerError | notImplemented | badGateway | serviceUnavailable | gatewayTimeout | <httpVersionNotSupported>."]
    pub r#response: Option<super::super::types::Code>,
    #[doc = "The value of the HTTP response code to be tested."]
    pub r#response_code: Option<super::super::types::String>,
    #[doc = "Fixture to evaluate the XPath/JSONPath expression or the headerField  against."]
    pub r#source_id: Option<super::super::types::Id>,
    #[doc = "Whether or not the current test execution will stop on failure for this assert."]
    pub r#stop_test_on_fail: super::super::types::Boolean,
    #[doc = "The ID of the Profile to validate against."]
    pub r#validate_profile_id: Option<super::super::types::Id>,
    #[doc = "The value to compare to."]
    pub r#value: Option<super::super::types::String>,
    #[doc = "Whether or not the test execution will produce a warning only on error for this assert."]
    pub r#warning_only: super::super::types::Boolean,
    #[doc = "Links or references providing traceability to the testing requirements for this assert."]
    pub r#requirement: Vec<TestScriptSetupActionAssertRequirement>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestScriptSetupActionAssert {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#label: Default::default(),
            r#description: Default::default(),
            r#direction: Default::default(),
            r#compare_to_source_id: Default::default(),
            r#compare_to_source_expression: Default::default(),
            r#compare_to_source_path: Default::default(),
            r#content_type: Default::default(),
            r#default_manual_completion: Default::default(),
            r#expression: Default::default(),
            r#header_field: Default::default(),
            r#minimum_id: Default::default(),
            r#navigation_links: Default::default(),
            r#operator: Default::default(),
            r#path: Default::default(),
            r#request_method: Default::default(),
            r#request_url: Default::default(),
            r#resource: Default::default(),
            r#response: Default::default(),
            r#response_code: Default::default(),
            r#source_id: Default::default(),
            r#stop_test_on_fail: super::super::types::Boolean {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#validate_profile_id: Default::default(),
            r#value: Default::default(),
            r#warning_only: super::super::types::Boolean {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#requirement: Default::default(),
        }
    }
}
#[doc = "Action would contain either an operation or an assertion."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptSetupAction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The operation to perform."]
    pub r#operation: Option<TestScriptSetupActionOperation>,
    #[doc = "Evaluates the results of previous operations to determine if the server under test behaves appropriately."]
    pub r#assert: Option<TestScriptSetupActionAssert>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestScriptSetupAction {
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
#[doc = "A series of required setup operations before tests are executed."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptSetup {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Action would contain either an operation or an assertion."]
    pub r#action: Vec<TestScriptSetupAction>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestScriptSetup {
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
pub struct TestScriptTestAction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "An operation would involve a REST request to a server."]
    pub r#operation: Option<TestScriptSetupActionOperation>,
    #[doc = "Evaluates the results of previous operations to determine if the server under test behaves appropriately."]
    pub r#assert: Option<TestScriptSetupActionAssert>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestScriptTestAction {
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
#[doc = "A test in this script."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptTest {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The name of this test used for tracking/logging purposes by test engines."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short description of the test used by test engines for tracking and reporting purposes."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Action would contain either an operation or an assertion."]
    pub r#action: Vec<TestScriptTestAction>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestScriptTest {
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
pub struct TestScriptTeardownAction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "An operation would involve a REST request to a server."]
    pub r#operation: TestScriptSetupActionOperation,
}
#[allow(clippy::derivable_impls)]
impl Default for TestScriptTeardownAction {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#operation: TestScriptSetupActionOperation {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "A series of operations required to clean up after all the tests are executed (successfully or otherwise)."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptTeardown {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The teardown action will only contain an operation."]
    pub r#action: Vec<TestScriptTeardownAction>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestScriptTeardown {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#action: Default::default(),
        }
    }
}
#[doc = "A structured set of tests against a FHIR server or client implementation to determine compliance against the FHIR specification."]
#[derive(Debug, Clone, PartialEq)]
pub struct TestScript {
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
    #[doc = "An absolute URI that is used to identify this test script when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which an authoritative instance of this test script is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the test script is stored on different servers."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "A formal identifier that is used to identify this test script when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The identifier that is used to identify this version of the test script when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the test script author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "Indicates the mechanism used to compare versions to determine which is more current."]
    pub r#version_algorithm: Option<TestScriptVersionAlgorithm>,
    #[doc = "A natural language name identifying the test script. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: super::super::types::String,
    #[doc = "A short, descriptive, user-friendly title for the test script."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The status of this test script. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this test script is authored for testing purposes (or education/evaluation/marketing) and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "The date (and optionally time) when the test script was last significantly changed. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the test script changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual responsible for the release and ongoing maintenance of the test script."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<super::super::types::ContactDetail>,
    #[doc = "A free text natural language description of the test script from a consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate test script instances."]
    pub r#use_context: Vec<super::super::types::UsageContext>,
    #[doc = "A legal or geographic region in which the test script is intended to be used."]
    pub r#jurisdiction: Vec<super::super::types::CodeableConcept>,
    #[doc = "Explanation of why this test script is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "A copyright statement relating to the test script and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the test script."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "A short string (<50 characters), suitable for inclusion in a page footer that identifies the copyright holder, effective period, and optionally whether rights are resctricted. (e.g. 'All rights reserved', 'Some rights reserved')."]
    pub r#copyright_label: Option<super::super::types::String>,
    #[doc = "An abstract server used in operations within this test script in the origin element."]
    pub r#origin: Vec<TestScriptOrigin>,
    #[doc = "An abstract server used in operations within this test script in the destination element."]
    pub r#destination: Vec<TestScriptDestination>,
    #[doc = "The required capability must exist and are assumed to function correctly on the FHIR server being tested."]
    pub r#metadata: Option<TestScriptMetadata>,
    #[doc = "The scope indicates a conformance artifact that is tested by the test(s) within this test case and the expectation of the test outcome(s) as well as the intended test phase inclusion."]
    pub r#scope: Vec<TestScriptScope>,
    #[doc = "Fixture in the test script - by reference (uri). All fixtures are required for the test script to execute."]
    pub r#fixture: Vec<TestScriptFixture>,
    #[doc = "Reference to the profile to be used for validation."]
    pub r#profile: Vec<super::super::types::Canonical>,
    #[doc = "Variable is set based either on element value in response body or on header field value in the response headers."]
    pub r#variable: Vec<TestScriptVariable>,
    #[doc = "A series of required setup operations before tests are executed."]
    pub r#setup: Option<TestScriptSetup>,
    #[doc = "A test in this script."]
    pub r#test: Vec<TestScriptTest>,
    #[doc = "A series of operations required to clean up after all the tests are executed (successfully or otherwise)."]
    pub r#teardown: Option<TestScriptTeardown>,
}
#[allow(clippy::derivable_impls)]
impl Default for TestScript {
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
            r#name: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
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
            r#origin: Default::default(),
            r#destination: Default::default(),
            r#metadata: Default::default(),
            r#scope: Default::default(),
            r#fixture: Default::default(),
            r#profile: Default::default(),
            r#variable: Default::default(),
            r#setup: Default::default(),
            r#test: Default::default(),
            r#teardown: Default::default(),
        }
    }
}
