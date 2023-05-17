// Generated on 2023-05-15 by fhirbolt-codegen v0.8.0
#[doc = "Actor participating in the resource."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExampleScenarioActor {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "ID or acronym of actor."]
    pub r#actor_id: super::super::types::String,
    #[doc = "The type of actor - person or system."]
    pub r#type: super::super::types::Code,
    #[doc = "The name of the actor as shown in the page."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "The description of the actor."]
    pub r#description: Option<super::super::types::Markdown>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExampleScenarioActor {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#actor_id: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#type: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#name: Default::default(),
            r#description: Default::default(),
        }
    }
}
#[doc = "A specific version of the resource."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExampleScenarioInstanceVersion {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The identifier of a specific version of a resource."]
    pub r#version_id: super::super::types::String,
    #[doc = "The description of the resource version."]
    pub r#description: super::super::types::Markdown,
}
#[allow(clippy::derivable_impls)]
impl Default for ExampleScenarioInstanceVersion {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#version_id: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#description: super::super::types::Markdown {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "Resources contained in the instance (e.g. the observations contained in a bundle)."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExampleScenarioInstanceContainedInstance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Each resource contained in the instance."]
    pub r#resource_id: super::super::types::String,
    #[doc = "A specific version of a resource contained in the instance."]
    pub r#version_id: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExampleScenarioInstanceContainedInstance {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#resource_id: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#version_id: Default::default(),
        }
    }
}
#[doc = "Each resource and each version that is present in the workflow."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExampleScenarioInstance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The id of the resource for referencing."]
    pub r#resource_id: super::super::types::String,
    #[doc = "The type of the resource."]
    pub r#resource_type: super::super::types::Code,
    #[doc = "A short name for the resource instance."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "Human-friendly description of the resource instance."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "A specific version of the resource."]
    pub r#version: Vec<ExampleScenarioInstanceVersion>,
    #[doc = "Resources contained in the instance (e.g. the observations contained in a bundle)."]
    pub r#contained_instance: Vec<ExampleScenarioInstanceContainedInstance>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExampleScenarioInstance {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#resource_id: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#resource_type: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#name: Default::default(),
            r#description: Default::default(),
            r#version: Default::default(),
            r#contained_instance: Default::default(),
        }
    }
}
#[doc = "Each interaction or action."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExampleScenarioProcessStepOperation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The sequential number of the interaction, e.g. 1.2.5."]
    pub r#number: super::super::types::String,
    #[doc = "The type of operation - CRUD."]
    pub r#type: Option<super::super::types::String>,
    #[doc = "The human-friendly name of the interaction."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "Who starts the transaction."]
    pub r#initiator: Option<super::super::types::String>,
    #[doc = "Who receives the transaction."]
    pub r#receiver: Option<super::super::types::String>,
    #[doc = "A comment to be inserted in the diagram."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "Whether the initiator is deactivated right after the transaction."]
    pub r#initiator_active: Option<super::super::types::Boolean>,
    #[doc = "Whether the receiver is deactivated right after the transaction."]
    pub r#receiver_active: Option<super::super::types::Boolean>,
    #[doc = "Each resource instance used by the initiator."]
    pub r#request: Option<ExampleScenarioInstanceContainedInstance>,
    #[doc = "Each resource instance used by the responder."]
    pub r#response: Option<ExampleScenarioInstanceContainedInstance>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExampleScenarioProcessStepOperation {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#number: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#type: Default::default(),
            r#name: Default::default(),
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
#[doc = "Indicates an alternative step that can be taken instead of the operations on the base step in exceptional/atypical circumstances."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExampleScenarioProcessStepAlternative {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The label to display for the alternative that gives a sense of the circumstance in which the alternative should be invoked."]
    pub r#title: super::super::types::String,
    #[doc = "A human-readable description of the alternative explaining when the alternative should occur rather than the base step."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "What happens in each alternative option."]
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
#[doc = "Each step of the process."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExampleScenarioProcessStep {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Nested process."]
    pub r#process: Vec<ExampleScenarioProcess>,
    #[doc = "If there is a pause in the flow."]
    pub r#pause: Option<super::super::types::Boolean>,
    #[doc = "Each interaction or action."]
    pub r#operation: Option<ExampleScenarioProcessStepOperation>,
    #[doc = "Indicates an alternative step that can be taken instead of the operations on the base step in exceptional/atypical circumstances."]
    pub r#alternative: Vec<ExampleScenarioProcessStepAlternative>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExampleScenarioProcessStep {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#process: Default::default(),
            r#pause: Default::default(),
            r#operation: Default::default(),
            r#alternative: Default::default(),
        }
    }
}
#[doc = "Each major process - a group of operations."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExampleScenarioProcess {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The diagram title of the group of operations."]
    pub r#title: super::super::types::String,
    #[doc = "A longer description of the group of operations."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "Description of initial status before the process starts."]
    pub r#pre_conditions: Option<super::super::types::Markdown>,
    #[doc = "Description of final status after the process ends."]
    pub r#post_conditions: Option<super::super::types::Markdown>,
    #[doc = "Each step of the process."]
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
#[doc = "Example of workflow instance."]
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "An absolute URI that is used to identify this example scenario when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which at which an authoritative instance of this example scenario is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the example scenario is stored on different servers."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "A formal identifier that is used to identify this example scenario when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The identifier that is used to identify this version of the example scenario when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the example scenario author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "A natural language name identifying the example scenario. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "The status of this example scenario. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this example scenario is authored for testing purposes (or education/evaluation/marketing) and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "The date  (and optionally time) when the example scenario was published. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the example scenario changes. (e.g. the 'content logical definition')."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual that published the example scenario."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<super::super::types::ContactDetail>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate example scenario instances."]
    pub r#use_context: Vec<super::super::types::UsageContext>,
    #[doc = "A legal or geographic region in which the example scenario is intended to be used."]
    pub r#jurisdiction: Vec<super::super::types::CodeableConcept>,
    #[doc = "A copyright statement relating to the example scenario and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the example scenario."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "What the example scenario resource is created for. This should not be used to show the business purpose of the scenario itself, but the purpose of documenting a scenario."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "Actor participating in the resource."]
    pub r#actor: Vec<ExampleScenarioActor>,
    #[doc = "Each resource and each version that is present in the workflow."]
    pub r#instance: Vec<ExampleScenarioInstance>,
    #[doc = "Each major process - a group of operations."]
    pub r#process: Vec<ExampleScenarioProcess>,
    #[doc = "Another nested workflow."]
    pub r#workflow: Vec<super::super::types::Canonical>,
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
            r#name: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#experimental: Default::default(),
            r#date: Default::default(),
            r#publisher: Default::default(),
            r#contact: Default::default(),
            r#use_context: Default::default(),
            r#jurisdiction: Default::default(),
            r#copyright: Default::default(),
            r#purpose: Default::default(),
            r#actor: Default::default(),
            r#instance: Default::default(),
            r#process: Default::default(),
            r#workflow: Default::default(),
        }
    }
}
