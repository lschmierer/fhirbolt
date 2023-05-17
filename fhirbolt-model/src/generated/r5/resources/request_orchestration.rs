// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "A duration or range of durations to apply to the relationship. For example, 30-60 minutes before."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum RequestOrchestrationActionRelatedActionOffset {
    Duration(Box<super::super::types::Duration>),
    Range(Box<super::super::types::Range>),
    #[default]
    Invalid,
}
#[doc = "An optional value describing when the action should be performed."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum RequestOrchestrationActionTiming {
    DateTime(Box<super::super::types::DateTime>),
    Age(Box<super::super::types::Age>),
    Period(Box<super::super::types::Period>),
    Duration(Box<super::super::types::Duration>),
    Range(Box<super::super::types::Range>),
    Timing(Box<super::super::types::Timing>),
    #[default]
    Invalid,
}
#[doc = "A reference to the actual participant."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum RequestOrchestrationActionParticipantActor {
    Canonical(Box<super::super::types::Canonical>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "A reference to an ActivityDefinition that describes the action to be taken in detail, a PlanDefinition that describes a series of actions to be taken, a Questionnaire that should be filled out, a SpecimenDefinition describing a specimen to be collected, or an ObservationDefinition that specifies what observation should be captured."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum RequestOrchestrationActionDefinition {
    Canonical(Box<super::super::types::Canonical>),
    Uri(Box<super::super::types::Uri>),
    #[default]
    Invalid,
}
#[doc = "An expression that describes applicability criteria, or start/stop conditions for the action."]
#[derive(Debug, Clone, PartialEq)]
pub struct RequestOrchestrationActionCondition {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The kind of condition."]
    pub r#kind: super::super::types::Code,
    #[doc = "An expression that returns true or false, indicating whether or not the condition is satisfied."]
    pub r#expression: Option<Box<super::super::types::Expression>>,
}
#[allow(clippy::derivable_impls)]
impl Default for RequestOrchestrationActionCondition {
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
#[doc = "Defines input data requirements for the action."]
#[derive(Debug, Clone, PartialEq)]
pub struct RequestOrchestrationActionInput {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A human-readable label for the data requirement used to label data flows in BPMN or similar diagrams. Also provides a human readable label when rendering the data requirement that conveys its purpose to human readers."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "Defines the data that is to be provided as input to the action."]
    pub r#requirement: Option<Box<super::super::types::DataRequirement>>,
    #[doc = "Points to an existing input or output element that provides data to this input."]
    pub r#related_data: Option<super::super::types::Id>,
}
#[allow(clippy::derivable_impls)]
impl Default for RequestOrchestrationActionInput {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#title: Default::default(),
            r#requirement: Default::default(),
            r#related_data: Default::default(),
        }
    }
}
#[doc = "Defines the outputs of the action, if any."]
#[derive(Debug, Clone, PartialEq)]
pub struct RequestOrchestrationActionOutput {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A human-readable label for the data requirement used to label data flows in BPMN or similar diagrams. Also provides a human readable label when rendering the data requirement that conveys its purpose to human readers."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "Defines the data that results as output from the action."]
    pub r#requirement: Option<Box<super::super::types::DataRequirement>>,
    #[doc = "Points to an existing input or output element that is results as output from the action."]
    pub r#related_data: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for RequestOrchestrationActionOutput {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#title: Default::default(),
            r#requirement: Default::default(),
            r#related_data: Default::default(),
        }
    }
}
#[doc = "A relationship to another action such as \"before\" or \"30-60 minutes after start of\"."]
#[derive(Debug, Clone, PartialEq)]
pub struct RequestOrchestrationActionRelatedAction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The element id of the target related action."]
    pub r#target_id: super::super::types::Id,
    #[doc = "The relationship of this action to the related action."]
    pub r#relationship: super::super::types::Code,
    #[doc = "The relationship of the end of this action to the related action."]
    pub r#end_relationship: Option<super::super::types::Code>,
    #[doc = "A duration or range of durations to apply to the relationship. For example, 30-60 minutes before."]
    pub r#offset: Option<RequestOrchestrationActionRelatedActionOffset>,
}
#[allow(clippy::derivable_impls)]
impl Default for RequestOrchestrationActionRelatedAction {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#target_id: super::super::types::Id {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#relationship: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#end_relationship: Default::default(),
            r#offset: Default::default(),
        }
    }
}
#[doc = "The participant that should perform or be responsible for this action."]
#[derive(Debug, Clone, PartialEq)]
pub struct RequestOrchestrationActionParticipant {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The type of participant in the action."]
    pub r#type: Option<super::super::types::Code>,
    #[doc = "The type of participant in the action."]
    pub r#type_canonical: Option<super::super::types::Canonical>,
    #[doc = "The type of participant in the action."]
    pub r#type_reference: Option<Box<super::super::types::Reference>>,
    #[doc = "The role the participant should play in performing the described action."]
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates how the actor will be involved in the action - author, reviewer, witness, etc."]
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A reference to the actual participant."]
    pub r#actor: Option<RequestOrchestrationActionParticipantActor>,
}
#[allow(clippy::derivable_impls)]
impl Default for RequestOrchestrationActionParticipant {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#type_canonical: Default::default(),
            r#type_reference: Default::default(),
            r#role: Default::default(),
            r#function: Default::default(),
            r#actor: Default::default(),
        }
    }
}
#[doc = "Customizations that should be applied to the statically defined resource. For example, if the dosage of a medication must be computed based on the patient's weight, a customization would be used to specify an expression that calculated the weight, and the path on the resource that would contain the result."]
#[derive(Debug, Clone, PartialEq)]
pub struct RequestOrchestrationActionDynamicValue {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The path to the element to be customized. This is the path on the resource that will hold the result of the calculation defined by the expression. The specified path SHALL be a FHIRPath resolvable on the specified target type of the ActivityDefinition, and SHALL consist only of identifiers, constant indexers, and a restricted subset of functions. The path is allowed to contain qualifiers (.) to traverse sub-elements, as well as indexers (\\[x\\]) to traverse multiple-cardinality sub-elements (see the [Simple FHIRPath Profile](fhirpath.html#simple) for full details)."]
    pub r#path: Option<super::super::types::String>,
    #[doc = "An expression specifying the value of the customized element."]
    pub r#expression: Option<Box<super::super::types::Expression>>,
}
#[allow(clippy::derivable_impls)]
impl Default for RequestOrchestrationActionDynamicValue {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#path: Default::default(),
            r#expression: Default::default(),
        }
    }
}
#[doc = "The actions, if any, produced by the evaluation of the artifact."]
#[derive(Debug, Clone, PartialEq)]
pub struct RequestOrchestrationAction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The linkId of the action from the PlanDefinition that corresponds to this action in the RequestOrchestration resource."]
    pub r#link_id: Option<super::super::types::String>,
    #[doc = "A user-visible prefix for the action. For example a section or item numbering such as 1. or A."]
    pub r#prefix: Option<super::super::types::String>,
    #[doc = "The title of the action displayed to a user."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "A short description of the action used to provide a summary to display to the user."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "A text equivalent of the action to be performed. This provides a human-interpretable description of the action when the definition is consumed by a system that might not be capable of interpreting it dynamically."]
    pub r#text_equivalent: Option<super::super::types::Markdown>,
    #[doc = "Indicates how quickly the action should be addressed with respect to other actions."]
    pub r#priority: Option<super::super::types::Code>,
    #[doc = "A code that provides meaning for the action or action group. For example, a section may have a LOINC code for a section of a documentation template."]
    pub r#code: Vec<super::super::types::CodeableConcept>,
    #[doc = "Didactic or other informational resources associated with the action that can be provided to the CDS recipient. Information resources can include inline text commentary and links to web resources."]
    pub r#documentation: Vec<super::super::types::RelatedArtifact>,
    #[doc = "Goals that are intended to be achieved by following the requests in this action."]
    pub r#goal: Vec<super::super::types::Reference>,
    #[doc = "An expression that describes applicability criteria, or start/stop conditions for the action."]
    pub r#condition: Vec<RequestOrchestrationActionCondition>,
    #[doc = "Defines input data requirements for the action."]
    pub r#input: Vec<RequestOrchestrationActionInput>,
    #[doc = "Defines the outputs of the action, if any."]
    pub r#output: Vec<RequestOrchestrationActionOutput>,
    #[doc = "A relationship to another action such as \"before\" or \"30-60 minutes after start of\"."]
    pub r#related_action: Vec<RequestOrchestrationActionRelatedAction>,
    #[doc = "An optional value describing when the action should be performed."]
    pub r#timing: Option<RequestOrchestrationActionTiming>,
    #[doc = "Identifies the facility where the action will occur; e.g. home, hospital, specific clinic, etc."]
    pub r#location: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "The participant that should perform or be responsible for this action."]
    pub r#participant: Vec<RequestOrchestrationActionParticipant>,
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
    #[doc = "A reference to an ActivityDefinition that describes the action to be taken in detail, a PlanDefinition that describes a series of actions to be taken, a Questionnaire that should be filled out, a SpecimenDefinition describing a specimen to be collected, or an ObservationDefinition that specifies what observation should be captured."]
    pub r#definition: Option<RequestOrchestrationActionDefinition>,
    #[doc = "A reference to a StructureMap resource that defines a transform that can be executed to produce the intent resource using the ActivityDefinition instance as the input."]
    pub r#transform: Option<super::super::types::Canonical>,
    #[doc = "Customizations that should be applied to the statically defined resource. For example, if the dosage of a medication must be computed based on the patient's weight, a customization would be used to specify an expression that calculated the weight, and the path on the resource that would contain the result."]
    pub r#dynamic_value: Vec<RequestOrchestrationActionDynamicValue>,
    #[doc = "Sub actions."]
    pub r#action: Vec<RequestOrchestrationAction>,
}
#[allow(clippy::derivable_impls)]
impl Default for RequestOrchestrationAction {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#link_id: Default::default(),
            r#prefix: Default::default(),
            r#title: Default::default(),
            r#description: Default::default(),
            r#text_equivalent: Default::default(),
            r#priority: Default::default(),
            r#code: Default::default(),
            r#documentation: Default::default(),
            r#goal: Default::default(),
            r#condition: Default::default(),
            r#input: Default::default(),
            r#output: Default::default(),
            r#related_action: Default::default(),
            r#timing: Default::default(),
            r#location: Default::default(),
            r#participant: Default::default(),
            r#type: Default::default(),
            r#grouping_behavior: Default::default(),
            r#selection_behavior: Default::default(),
            r#required_behavior: Default::default(),
            r#precheck_behavior: Default::default(),
            r#cardinality_behavior: Default::default(),
            r#resource: Default::default(),
            r#definition: Default::default(),
            r#transform: Default::default(),
            r#dynamic_value: Default::default(),
            r#action: Default::default(),
        }
    }
}
#[doc = "A set of related requests that can be used to capture intended activities that have inter-dependencies such as \"give this medication after that one\"."]
#[derive(Debug, Clone, PartialEq)]
pub struct RequestOrchestration {
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
    #[doc = "A shared identifier common to multiple independent Request instances that were activated/authorized more or less simultaneously by a single author.  The presence of the same identifier on each request ties those requests together and may have business ramifications in terms of reporting of results, billing, etc.  E.g. a requisition number shared by a set of lab tests ordered together, or a prescription number shared by all meds ordered at one time."]
    pub r#group_identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "The current state of the request. For request orchestrations, the status reflects the status of all the requests in the orchestration."]
    pub r#status: super::super::types::Code,
    #[doc = "Indicates the level of authority/intentionality associated with the request and where the request fits into the workflow chain."]
    pub r#intent: super::super::types::Code,
    #[doc = "Indicates how quickly the request should be addressed with respect to other requests."]
    pub r#priority: Option<super::super::types::Code>,
    #[doc = "A code that identifies what the overall request orchestration is."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The subject for which the request orchestration was created."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "Describes the context of the request orchestration, if any."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Indicates when the request orchestration was created."]
    pub r#authored_on: Option<super::super::types::DateTime>,
    #[doc = "Provides a reference to the author of the request orchestration."]
    pub r#author: Option<Box<super::super::types::Reference>>,
    #[doc = "Describes the reason for the request orchestration in coded or textual form."]
    pub r#reason: Vec<super::super::types::CodeableReference>,
    #[doc = "Goals that are intended to be achieved by following the requests in this RequestOrchestration."]
    pub r#goal: Vec<super::super::types::Reference>,
    #[doc = "Provides a mechanism to communicate additional information about the response."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "The actions, if any, produced by the evaluation of the artifact."]
    pub r#action: Vec<RequestOrchestrationAction>,
}
#[allow(clippy::derivable_impls)]
impl Default for RequestOrchestration {
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
            r#reason: Default::default(),
            r#goal: Default::default(),
            r#note: Default::default(),
            r#action: Default::default(),
        }
    }
}
