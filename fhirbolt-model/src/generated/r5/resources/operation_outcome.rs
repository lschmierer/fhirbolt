// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "An error, warning, or information message that results from a system action."]
#[derive(Debug, Clone, PartialEq)]
pub struct OperationOutcomeIssue {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Indicates whether the issue indicates a variation from successful processing."]
    pub r#severity: super::super::types::Code,
    #[doc = "Describes the type of the issue. The system that creates an OperationOutcome SHALL choose the most applicable code from the IssueType value set, and may additional provide its own code for the error in the details element."]
    pub r#code: super::super::types::Code,
    #[doc = "Additional details about the error. This may be a text description of the error or a system code that identifies the error."]
    pub r#details: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Additional diagnostic information about the issue."]
    pub r#diagnostics: Option<super::super::types::String>,
    #[doc = "This element is deprecated because it is XML specific. It is replaced by issue.expression, which is format independent, and simpler to parse. \n\nFor resource issues, this will be a simple XPath limited to element names, repetition indicators and the default child accessor that identifies one of the elements in the resource that caused this issue to be raised.  For HTTP errors, will be \"http.\" + the parameter name."]
    pub r#location: Vec<super::super::types::String>,
    #[doc = "A [simple subset of FHIRPath](fhirpath.html#simple) limited to element names, repetition indicators and the default child accessor that identifies one of the elements in the resource that caused this issue to be raised."]
    pub r#expression: Vec<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for OperationOutcomeIssue {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#severity: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#code: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#details: Default::default(),
            r#diagnostics: Default::default(),
            r#location: Default::default(),
            r#expression: Default::default(),
        }
    }
}
#[doc = "A collection of error, warning, or information messages that result from a system action."]
#[derive(Debug, Clone, PartialEq)]
pub struct OperationOutcome {
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, nor can they have their own independent transaction scope. This is allowed to be a Parameters resource if and only if it is referenced by a resource that provides context/meaning."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "An error, warning, or information message that results from a system action."]
    pub r#issue: Vec<OperationOutcomeIssue>,
}
#[allow(clippy::derivable_impls)]
impl Default for OperationOutcome {
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
            r#issue: Default::default(),
        }
    }
}
