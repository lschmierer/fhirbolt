// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "A duration or range of durations to apply to the relationship. For example, 30-60 minutes before."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum RequestGroupActionRelatedActionOffset {
    Duration(Box<super::super::types::Duration>),
    Range(Box<super::super::types::Range>),
    #[default]
    Invalid,
}
#[doc = "An optional value describing when the action should be performed."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum RequestGroupActionTiming {
    DateTime(super::super::types::DateTime),
    Age(Box<super::super::types::Age>),
    Period(Box<super::super::types::Period>),
    Duration(Box<super::super::types::Duration>),
    Range(Box<super::super::types::Range>),
    Timing(Box<super::super::types::Timing>),
    #[default]
    Invalid,
}
#[doc = "An expression that describes applicability criteria, or start/stop conditions for the action."]
#[derive(Debug, Clone, PartialEq)]
pub struct RequestGroupActionCondition {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The kind of condition."]
    pub r#kind: super::super::types::Code,
    #[doc = "An expression that returns true or false, indicating whether or not the condition is satisfied."]
    pub r#expression: Option<Box<super::super::types::Expression>>,
}
#[allow(clippy::derivable_impls)]
impl Default for RequestGroupActionCondition {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#kind: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#expression: Default::default(),
        }
    }
}
#[doc = "A relationship to another action such as \"before\" or \"30-60 minutes after start of\"."]
#[derive(Debug, Clone, PartialEq)]
pub struct RequestGroupActionRelatedAction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The element id of the action this is related to."]
    pub r#action_id: super::super::types::Id,
    #[doc = "The relationship of this action to the related action."]
    pub r#relationship: super::super::types::Code,
    #[doc = "A duration or range of durations to apply to the relationship. For example, 30-60 minutes before."]
    pub r#offset: Option<RequestGroupActionRelatedActionOffset>,
}
#[allow(clippy::derivable_impls)]
impl Default for RequestGroupActionRelatedAction {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#action_id: super::super::types::Id {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#relationship: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#offset: Default::default(),
        }
    }
}
#[doc = "The actions, if any, produced by the evaluation of the artifact."]
#[derive(Debug, Clone, PartialEq)]
pub struct RequestGroupAction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A user-visible prefix for the action."]
    pub r#prefix: Option<super::super::types::String>,
    #[doc = "The title of the action displayed to a user."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "A short description of the action used to provide a summary to display to the user."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "A text equivalent of the action to be performed. This provides a human-interpretable description of the action when the definition is consumed by a system that might not be capable of interpreting it dynamically."]
    pub r#text_equivalent: Option<super::super::types::String>,
    #[doc = "Indicates how quickly the action should be addressed with respect to other actions."]
    pub r#priority: Option<super::super::types::Code>,
    #[doc = "A code that provides meaning for the action or action group. For example, a section may have a LOINC code for a section of a documentation template."]
    pub r#code: Vec<super::super::types::CodeableConcept>,
    #[doc = "Didactic or other informational resources associated with the action that can be provided to the CDS recipient. Information resources can include inline text commentary and links to web resources."]
    pub r#documentation: Vec<super::super::types::RelatedArtifact>,
    #[doc = "An expression that describes applicability criteria, or start/stop conditions for the action."]
    pub r#condition: Vec<RequestGroupActionCondition>,
    #[doc = "A relationship to another action such as \"before\" or \"30-60 minutes after start of\"."]
    pub r#related_action: Vec<RequestGroupActionRelatedAction>,
    #[doc = "An optional value describing when the action should be performed."]
    pub r#timing: Option<RequestGroupActionTiming>,
    #[doc = "The participant that should perform or be responsible for this action."]
    pub r#participant: Vec<super::super::types::Reference>,
    #[doc = "The type of action to perform (create, update, remove)."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Defines the grouping behavior for the action and its children."]
    pub r#grouping_behavior: Option<super::super::types::Code>,
    #[doc = "Defines the selection behavior for the action and its children."]
    pub r#selection_behavior: Option<super::super::types::Code>,
    #[doc = "Defines expectations around whether an action is required."]
    pub r#required_behavior: Option<super::super::types::Code>,
    #[doc = "Defines whether the action should usually be preselected."]
    pub r#precheck_behavior: Option<super::super::types::Code>,
    #[doc = "Defines whether the action can be selected multiple times."]
    pub r#cardinality_behavior: Option<super::super::types::Code>,
    #[doc = "The resource that is the target of the action (e.g. CommunicationRequest)."]
    pub r#resource: Option<Box<super::super::types::Reference>>,
    #[doc = "Sub actions."]
    pub r#action: Vec<RequestGroupAction>,
}
#[allow(clippy::derivable_impls)]
impl Default for RequestGroupAction {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#prefix: Default::default(),
            r#title: Default::default(),
            r#description: Default::default(),
            r#text_equivalent: Default::default(),
            r#priority: Default::default(),
            r#code: Default::default(),
            r#documentation: Default::default(),
            r#condition: Default::default(),
            r#related_action: Default::default(),
            r#timing: Default::default(),
            r#participant: Default::default(),
            r#type: Default::default(),
            r#grouping_behavior: Default::default(),
            r#selection_behavior: Default::default(),
            r#required_behavior: Default::default(),
            r#precheck_behavior: Default::default(),
            r#cardinality_behavior: Default::default(),
            r#resource: Default::default(),
            r#action: Default::default(),
        }
    }
}
#[doc = "A group of related requests that can be used to capture intended activities that have inter-dependencies such as \"give this medication after that one\"."]
#[derive(Debug, Clone, PartialEq)]
pub struct RequestGroup {
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
    #[doc = "Allows a service to provide a unique, business identifier for the request."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "A canonical URL referencing a FHIR-defined protocol, guideline, orderset or other definition that is adhered to in whole or in part by this request."]
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    #[doc = "A URL referencing an externally defined protocol, guideline, orderset or other definition that is adhered to in whole or in part by this request."]
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    #[doc = "A plan, proposal or order that is fulfilled in whole or in part by this request."]
    pub r#based_on: Vec<super::super::types::Reference>,
    #[doc = "Completed or terminated request(s) whose function is taken by this new request."]
    pub r#replaces: Vec<super::super::types::Reference>,
    #[doc = "A shared identifier common to all requests that were authorized more or less simultaneously by a single author, representing the identifier of the requisition, prescription or similar form."]
    pub r#group_identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "The current state of the request. For request groups, the status reflects the status of all the requests in the group."]
    pub r#status: super::super::types::Code,
    #[doc = "Indicates the level of authority/intentionality associated with the request and where the request fits into the workflow chain."]
    pub r#intent: super::super::types::Code,
    #[doc = "Indicates how quickly the request should be addressed with respect to other requests."]
    pub r#priority: Option<super::super::types::Code>,
    #[doc = "A code that identifies what the overall request group is."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The subject for which the request group was created."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "Describes the context of the request group, if any."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Indicates when the request group was created."]
    pub r#authored_on: Option<super::super::types::DateTime>,
    #[doc = "Provides a reference to the author of the request group."]
    pub r#author: Option<Box<super::super::types::Reference>>,
    #[doc = "Describes the reason for the request group in coded or textual form."]
    pub r#reason_code: Vec<super::super::types::CodeableConcept>,
    #[doc = "Indicates another resource whose existence justifies this request group."]
    pub r#reason_reference: Vec<super::super::types::Reference>,
    #[doc = "Provides a mechanism to communicate additional information about the response."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "The actions, if any, produced by the evaluation of the artifact."]
    pub r#action: Vec<RequestGroupAction>,
}
#[allow(clippy::derivable_impls)]
impl Default for RequestGroup {
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
            r#instantiates_canonical: Default::default(),
            r#instantiates_uri: Default::default(),
            r#based_on: Default::default(),
            r#replaces: Default::default(),
            r#group_identifier: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#intent: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#priority: Default::default(),
            r#code: Default::default(),
            r#subject: Default::default(),
            r#encounter: Default::default(),
            r#authored_on: Default::default(),
            r#author: Default::default(),
            r#reason_code: Default::default(),
            r#reason_reference: Default::default(),
            r#note: Default::default(),
            r#action: Default::default(),
        }
    }
}
