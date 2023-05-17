// Generated on 2023-05-17 by fhirbolt-codegen v0.9.0
#[doc = "Indicates the mechanism used to compare versions to determine which is more current."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ExampleScenarioVersionAlgorithm {
    String(Box<super::super::types::String>),
    Coding(Box<super::super::types::Coding>),
    #[default]
    Invalid,
}
#[doc = "Refers to a profile, template or other ruleset the instance adheres to."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ExampleScenarioInstanceStructureProfile {
    Canonical(Box<super::super::types::Canonical>),
    Uri(Box<super::super::types::Uri>),
    #[default]
    Invalid,
}
#[doc = "A system or person who shares or receives an instance within the scenario."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExampleScenarioActor {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A unique string within the scenario that is used to reference the actor."]
    pub r#key: super::super::types::String,
    #[doc = "The category of actor - person or system."]
    pub r#type: super::super::types::Code,
    #[doc = "The human-readable name for the actor used when rendering the scenario."]
    pub r#title: super::super::types::String,
    #[doc = "An explanation of who/what the actor is and its role in the scenario."]
    pub r#description: Option<super::super::types::Markdown>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExampleScenarioActor {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#key: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#type: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#title: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#description: Default::default(),
        }
    }
}
#[doc = "Represents the instance as it was at a specific time-point."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExampleScenarioInstanceVersion {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A unique string within the instance that is used to reference the version of the instance."]
    pub r#key: super::super::types::String,
    #[doc = "A short descriptive label the version to be used in tables or diagrams."]
    pub r#title: super::super::types::String,
    #[doc = "An explanation of what this specific version of the instance contains and represents."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "Points to an instance (typically an example) that shows the data that would flow at this point in the scenario."]
    pub r#content: Option<Box<super::super::types::Reference>>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExampleScenarioInstanceVersion {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#key: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#title: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#description: Default::default(),
            r#content: Default::default(),
        }
    }
}
#[doc = "References to other instances that can be found within this instance (e.g. the observations contained in a bundle)."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExampleScenarioInstanceContainedInstance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A reference to the key of an instance found within this one."]
    pub r#instance_reference: super::super::types::String,
    #[doc = "A reference to the key of a specific version of an instance in this instance."]
    pub r#version_reference: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExampleScenarioInstanceContainedInstance {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#instance_reference: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#version_reference: Default::default(),
        }
    }
}
#[doc = "A single data collection that is shared as part of the scenario."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExampleScenarioInstance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A unique string within the scenario that is used to reference the instance."]
    pub r#key: super::super::types::String,
    #[doc = "A code indicating the kind of data structure (FHIR resource or some other standard) this is an instance of."]
    pub r#structure_type: Box<super::super::types::Coding>,
    #[doc = "Conveys the version of the data structure instantiated.  I.e. what release of FHIR, X12, OpenEHR, etc. is instance compliant with."]
    pub r#structure_version: Option<super::super::types::String>,
    #[doc = "Refers to a profile, template or other ruleset the instance adheres to."]
    pub r#structure_profile: Option<ExampleScenarioInstanceStructureProfile>,
    #[doc = "A short descriptive label the instance to be used in tables or diagrams."]
    pub r#title: super::super::types::String,
    #[doc = "An explanation of what the instance contains and what it's for."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "Points to an instance (typically an example) that shows the data that would corespond to this instance."]
    pub r#content: Option<Box<super::super::types::Reference>>,
    #[doc = "Represents the instance as it was at a specific time-point."]
    pub r#version: Vec<ExampleScenarioInstanceVersion>,
    #[doc = "References to other instances that can be found within this instance (e.g. the observations contained in a bundle)."]
    pub r#contained_instance: Vec<ExampleScenarioInstanceContainedInstance>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExampleScenarioInstance {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#key: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#structure_type: Box::new(super::super::types::Coding {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#structure_version: Default::default(),
            r#structure_profile: Default::default(),
            r#title: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#description: Default::default(),
            r#content: Default::default(),
            r#version: Default::default(),
            r#contained_instance: Default::default(),
        }
    }
}
#[doc = "The step represents a single operation invoked on receiver by sender."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExampleScenarioProcessStepOperation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The standardized type of action (FHIR or otherwise)."]
    pub r#type: Option<Box<super::super::types::Coding>>,
    #[doc = "A short descriptive label the step to be used in tables or diagrams."]
    pub r#title: super::super::types::String,
    #[doc = "The system that invokes the action/transmits the data."]
    pub r#initiator: Option<super::super::types::String>,
    #[doc = "The system on which the action is invoked/receives the data."]
    pub r#receiver: Option<super::super::types::String>,
    #[doc = "An explanation of what the operation represents and what it does."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "If false, the initiator is deactivated right after the operation."]
    pub r#initiator_active: Option<super::super::types::Boolean>,
    #[doc = "If false, the receiver is deactivated right after the operation."]
    pub r#receiver_active: Option<super::super::types::Boolean>,
    #[doc = "A reference to the instance that is transmitted from requester to receiver as part of the invocation of the operation."]
    pub r#request: Option<ExampleScenarioInstanceContainedInstance>,
    #[doc = "A reference to the instance that is transmitted from receiver to requester as part of the operation's synchronous response (if any)."]
    pub r#response: Option<ExampleScenarioInstanceContainedInstance>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExampleScenarioProcessStepOperation {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#title: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#initiator: Default::default(),
            r#receiver: Default::default(),
            r#description: Default::default(),
            r#initiator_active: Default::default(),
            r#receiver_active: Default::default(),
            r#request: Default::default(),
            r#response: Default::default(),
        }
    }
}
#[doc = "Indicates an alternative step that can be taken instead of the sub-process, scenario or operation.  E.g. to represent non-happy-path/exceptional/atypical circumstances."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExampleScenarioProcessStepAlternative {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The label to display for the alternative that gives a sense of the circumstance in which the alternative should be invoked."]
    pub r#title: super::super::types::String,
    #[doc = "A human-readable description of the alternative explaining when the alternative should occur rather than the base step."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "Indicates the operation, sub-process or scenario that happens if the alternative option is selected."]
    pub r#step: Vec<ExampleScenarioProcessStep>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExampleScenarioProcessStepAlternative {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#title: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#description: Default::default(),
            r#step: Default::default(),
        }
    }
}
#[doc = "A significant action that occurs as part of the process."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExampleScenarioProcessStep {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The sequential number of the step, e.g. 1.2.5."]
    pub r#number: Option<super::super::types::String>,
    #[doc = "Indicates that the step is a complex sub-process with its own steps."]
    pub r#process: Option<ExampleScenarioProcess>,
    #[doc = "Indicates that the step is defined by a seaparate scenario instance."]
    pub r#workflow: Option<super::super::types::Canonical>,
    #[doc = "The step represents a single operation invoked on receiver by sender."]
    pub r#operation: Option<ExampleScenarioProcessStepOperation>,
    #[doc = "Indicates an alternative step that can be taken instead of the sub-process, scenario or operation.  E.g. to represent non-happy-path/exceptional/atypical circumstances."]
    pub r#alternative: Vec<ExampleScenarioProcessStepAlternative>,
    #[doc = "If true, indicates that, following this step, there is a pause in the flow and the subsequent step will occur at some later time (triggered by some event)."]
    pub r#pause: Option<super::super::types::Boolean>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExampleScenarioProcessStep {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#number: Default::default(),
            r#process: Default::default(),
            r#workflow: Default::default(),
            r#operation: Default::default(),
            r#alternative: Default::default(),
            r#pause: Default::default(),
        }
    }
}
#[doc = "A group of operations that represents a significant step within a scenario."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExampleScenarioProcess {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A short descriptive label the process to be used in tables or diagrams."]
    pub r#title: super::super::types::String,
    #[doc = "An explanation of what the process represents and what it does."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "Description of the initial state of the actors, environment and data before the process starts."]
    pub r#pre_conditions: Option<super::super::types::Markdown>,
    #[doc = "Description of the final state of the actors, environment and data after the process has been successfully completed."]
    pub r#post_conditions: Option<super::super::types::Markdown>,
    #[doc = "A significant action that occurs as part of the process."]
    pub r#step: Vec<ExampleScenarioProcessStep>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExampleScenarioProcess {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#title: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#description: Default::default(),
            r#pre_conditions: Default::default(),
            r#post_conditions: Default::default(),
            r#step: Default::default(),
        }
    }
}
#[doc = "A walkthrough of a workflow showing the interaction between systems and the instances shared, possibly including the evolution of instances over time."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExampleScenario {
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
    #[doc = "An absolute URI that is used to identify this example scenario when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which an authoritative instance of this example scenario is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the example scenario is stored on different servers."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "A formal identifier that is used to identify this example scenario when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The identifier that is used to identify this version of the example scenario when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the example scenario author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "Indicates the mechanism used to compare versions to determine which is more current."]
    pub r#version_algorithm: Option<ExampleScenarioVersionAlgorithm>,
    #[doc = "Temporarily retained for tooling purposes."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for the ExampleScenario."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The status of this example scenario. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this example scenario is authored for testing purposes (or education/evaluation/marketing) and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "The date  (and optionally time) when the example scenario was last significantly changed. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the example scenario changes. (e.g. the 'content logical definition')."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual responsible for the release and ongoing maintenance of the example scenario."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<super::super::types::ContactDetail>,
    #[doc = "A free text natural language description of the ExampleScenario from a consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate example scenario instances."]
    pub r#use_context: Vec<super::super::types::UsageContext>,
    #[doc = "A legal or geographic region in which the example scenario is intended to be used."]
    pub r#jurisdiction: Vec<super::super::types::CodeableConcept>,
    #[doc = "What the example scenario resource is created for. This should not be used to show the business purpose of the scenario itself, but the purpose of documenting a scenario."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "A copyright statement relating to the example scenario and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the example scenario."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "A short string (<50 characters), suitable for inclusion in a page footer that identifies the copyright holder, effective period, and optionally whether rights are resctricted. (e.g. 'All rights reserved', 'Some rights reserved')."]
    pub r#copyright_label: Option<super::super::types::String>,
    #[doc = "A system or person who shares or receives an instance within the scenario."]
    pub r#actor: Vec<ExampleScenarioActor>,
    #[doc = "A single data collection that is shared as part of the scenario."]
    pub r#instance: Vec<ExampleScenarioInstance>,
    #[doc = "A group of operations that represents a significant step within a scenario."]
    pub r#process: Vec<ExampleScenarioProcess>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExampleScenario {
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
            r#actor: Default::default(),
            r#instance: Default::default(),
            r#process: Default::default(),
        }
    }
}
