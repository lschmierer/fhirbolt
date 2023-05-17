// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "Indicates the mechanism used to compare versions to determine which is more current."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum OperationDefinitionVersionAlgorithm {
    String(Box<super::super::types::String>),
    Coding(Box<super::super::types::Coding>),
    #[default]
    Invalid,
}
#[doc = "Binds to a value set if this parameter is coded (code, Coding, CodeableConcept)."]
#[derive(Debug, Clone, PartialEq)]
pub struct OperationDefinitionParameterBinding {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Indicates the degree of conformance expectations associated with this binding - that is, the degree to which the provided value set must be adhered to in the instances."]
    pub r#strength: super::super::types::Code,
    #[doc = "Points to the value set or external definition (e.g. implicit value set) that identifies the set of codes to be used."]
    pub r#value_set: super::super::types::Canonical,
}
#[allow(clippy::derivable_impls)]
impl Default for OperationDefinitionParameterBinding {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#strength: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#value_set: super::super::types::Canonical {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "Identifies other resource parameters within the operation invocation that are expected to resolve to this resource."]
#[derive(Debug, Clone, PartialEq)]
pub struct OperationDefinitionParameterReferencedFrom {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The name of the parameter or dot-separated path of parameter names pointing to the resource parameter that is expected to contain a reference to this resource."]
    pub r#source: super::super::types::String,
    #[doc = "The id of the element in the referencing resource that is expected to resolve to this resource."]
    pub r#source_id: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for OperationDefinitionParameterReferencedFrom {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#source: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#source_id: Default::default(),
        }
    }
}
#[doc = "The parameters for the operation/query."]
#[derive(Debug, Clone, PartialEq)]
pub struct OperationDefinitionParameter {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The name of used to identify the parameter."]
    pub r#name: super::super::types::Code,
    #[doc = "Whether this is an input or an output parameter."]
    pub r#use: super::super::types::Code,
    #[doc = "If present, indicates that the parameter applies when the operation is being invoked at the specified level."]
    pub r#scope: Vec<super::super::types::Code>,
    #[doc = "The minimum number of times this parameter SHALL appear in the request or response."]
    pub r#min: super::super::types::Integer,
    #[doc = "The maximum number of times this element is permitted to appear in the request or response."]
    pub r#max: super::super::types::String,
    #[doc = "Describes the meaning or use of this parameter."]
    pub r#documentation: Option<super::super::types::Markdown>,
    #[doc = "The type for this parameter."]
    pub r#type: Option<super::super::types::Code>,
    #[doc = "Support for polymorphic types. If the parameter type is abstract, this element lists allowed sub-types for the parameter."]
    pub r#allowed_type: Vec<super::super::types::Code>,
    #[doc = "Used when the type is \"Reference\" or \"canonical\", and identifies a profile structure or implementation Guide that applies to the target of the reference this parameter refers to. If any profiles are specified, then the content must conform to at least one of them. The URL can be a local reference - to a contained StructureDefinition, or a reference to another StructureDefinition or Implementation Guide by a canonical URL. When an implementation guide is specified, the target resource SHALL conform to at least one profile defined in the implementation guide."]
    pub r#target_profile: Vec<super::super::types::Canonical>,
    #[doc = "How the parameter is understood if/when it used as search parameter. This is only used if the parameter is a string."]
    pub r#search_type: Option<super::super::types::Code>,
    #[doc = "Binds to a value set if this parameter is coded (code, Coding, CodeableConcept)."]
    pub r#binding: Option<OperationDefinitionParameterBinding>,
    #[doc = "Identifies other resource parameters within the operation invocation that are expected to resolve to this resource."]
    pub r#referenced_from: Vec<OperationDefinitionParameterReferencedFrom>,
    #[doc = "The parts of a nested Parameter."]
    pub r#part: Vec<OperationDefinitionParameter>,
}
#[allow(clippy::derivable_impls)]
impl Default for OperationDefinitionParameter {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#name: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#use: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#scope: Default::default(),
            r#min: super::super::types::Integer {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#max: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#documentation: Default::default(),
            r#type: Default::default(),
            r#allowed_type: Default::default(),
            r#target_profile: Default::default(),
            r#search_type: Default::default(),
            r#binding: Default::default(),
            r#referenced_from: Default::default(),
            r#part: Default::default(),
        }
    }
}
#[doc = "Defines an appropriate combination of parameters to use when invoking this operation, to help code generators when generating overloaded parameter sets for this operation."]
#[derive(Debug, Clone, PartialEq)]
pub struct OperationDefinitionOverload {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Name of parameter to include in overload."]
    pub r#parameter_name: Vec<super::super::types::String>,
    #[doc = "Comments to go on overload."]
    pub r#comment: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for OperationDefinitionOverload {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#parameter_name: Default::default(),
            r#comment: Default::default(),
        }
    }
}
#[doc = "A formal computable definition of an operation (on the RESTful interface) or a named query (using the search interaction)."]
#[derive(Debug, Clone, PartialEq)]
pub struct OperationDefinition {
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
    #[doc = "An absolute URI that is used to identify this operation definition when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which an authoritative instance of this operation definition is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the operation definition is stored on different servers."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "A formal identifier that is used to identify this implementation guide when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The identifier that is used to identify this version of the operation definition when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the operation definition author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "Indicates the mechanism used to compare versions to determine which is more current."]
    pub r#version_algorithm: Option<OperationDefinitionVersionAlgorithm>,
    #[doc = "A natural language name identifying the operation definition. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: super::super::types::String,
    #[doc = "A short, descriptive, user-friendly title for the operation definition."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The current state of this operation definition."]
    pub r#status: super::super::types::Code,
    #[doc = "Whether this is an operation or a named query."]
    pub r#kind: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this operation definition is authored for testing purposes (or education/evaluation/marketing) and is not intended for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "The date  (and optionally time) when the operation definition was last significantly changed. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the operation definition changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual responsible for the release and ongoing maintenance of the operation definition."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<super::super::types::ContactDetail>,
    #[doc = "A free text natural language description of the operation definition from a consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate operation definition."]
    pub r#use_context: Vec<super::super::types::UsageContext>,
    #[doc = "A legal or geographic region in which the operation definition is intended to be used."]
    pub r#jurisdiction: Vec<super::super::types::CodeableConcept>,
    #[doc = "Explanation of why this operation definition is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "A copyright statement relating to the operation definition and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the operation definition."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "A short string (<50 characters), suitable for inclusion in a page footer that identifies the copyright holder, effective period, and optionally whether rights are resctricted. (e.g. 'All rights reserved', 'Some rights reserved')."]
    pub r#copyright_label: Option<super::super::types::String>,
    #[doc = "Whether the operation affects state. Side effects such as producing audit trail entries do not count as 'affecting  state'."]
    pub r#affects_state: Option<super::super::types::Boolean>,
    #[doc = "The label that is recommended to be used in the URL for this operation. In some cases, servers may need to use a different CapabilityStatement operation.name to differentiate between multiple SearchParameters that happen to have the same code."]
    pub r#code: super::super::types::Code,
    #[doc = "Additional information about how to use this operation or named query."]
    pub r#comment: Option<super::super::types::Markdown>,
    #[doc = "Indicates that this operation definition is a constraining profile on the base."]
    pub r#base: Option<super::super::types::Canonical>,
    #[doc = "The types on which this operation can be executed."]
    pub r#resource: Vec<super::super::types::Code>,
    #[doc = "Indicates whether this operation or named query can be invoked at the system level (e.g. without needing to choose a resource type for the context)."]
    pub r#system: super::super::types::Boolean,
    #[doc = "Indicates whether this operation or named query can be invoked at the resource type level for any given resource type level (e.g. without needing to choose a specific resource id for the context)."]
    pub r#type: super::super::types::Boolean,
    #[doc = "Indicates whether this operation can be invoked on a particular instance of one of the given types."]
    pub r#instance: super::super::types::Boolean,
    #[doc = "Additional validation information for the in parameters - a single profile that covers all the parameters. The profile is a constraint on the parameters resource as a whole."]
    pub r#input_profile: Option<super::super::types::Canonical>,
    #[doc = "Additional validation information for the out parameters - a single profile that covers all the parameters. The profile is a constraint on the parameters resource."]
    pub r#output_profile: Option<super::super::types::Canonical>,
    #[doc = "The parameters for the operation/query."]
    pub r#parameter: Vec<OperationDefinitionParameter>,
    #[doc = "Defines an appropriate combination of parameters to use when invoking this operation, to help code generators when generating overloaded parameter sets for this operation."]
    pub r#overload: Vec<OperationDefinitionOverload>,
}
#[allow(clippy::derivable_impls)]
impl Default for OperationDefinition {
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
            r#kind: super::super::types::Code {
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
            r#affects_state: Default::default(),
            r#code: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#comment: Default::default(),
            r#base: Default::default(),
            r#resource: Default::default(),
            r#system: super::super::types::Boolean {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#type: super::super::types::Boolean {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#instance: super::super::types::Boolean {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#input_profile: Default::default(),
            r#output_profile: Default::default(),
            r#parameter: Default::default(),
            r#overload: Default::default(),
        }
    }
}
