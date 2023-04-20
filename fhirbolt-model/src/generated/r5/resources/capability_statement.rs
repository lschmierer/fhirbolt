// Generated on 2023-04-20 by fhirbolt-codegen v0.4.0
#[doc = "Indicates the mechanism used to compare versions to determine which is more current."]
#[derive(Debug, Clone, PartialEq)]
pub enum CapabilityStatementVersionAlgorithm {
    String(Box<super::super::types::String>),
    Coding(Box<super::super::types::Coding>),
    Invalid,
}
impl Default for CapabilityStatementVersionAlgorithm {
    fn default() -> CapabilityStatementVersionAlgorithm {
        CapabilityStatementVersionAlgorithm::Invalid
    }
}
#[doc = "Software that is covered by this capability statement.  It is used when the capability statement describes the capabilities of a particular software version, independent of an installation."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CapabilityStatementSoftware {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Name the software is known by."]
    pub r#name: super::super::types::String,
    #[doc = "The version identifier for the software covered by this statement."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "Date this version of the software was released."]
    pub r#release_date: Option<super::super::types::DateTime>,
}
#[doc = "Identifies a specific implementation instance that is described by the capability statement - i.e. a particular installation, rather than the capabilities of a software program."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CapabilityStatementImplementation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Information about the specific installation that this capability statement relates to."]
    pub r#description: super::super::types::Markdown,
    #[doc = "An absolute base URL for the implementation.  This forms the base for REST interfaces as well as the mailbox and document interfaces."]
    pub r#url: Option<super::super::types::Url>,
    #[doc = "The organization responsible for the management of the instance and oversight of the data on the server at the specified URL."]
    pub r#custodian: Option<Box<super::super::types::Reference>>,
}
#[doc = "Information about security implementation from an interface perspective - what a client needs to know."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CapabilityStatementRestSecurity {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Server adds CORS headers when responding to requests - this enables Javascript applications to use the server."]
    pub r#cors: Option<super::super::types::Boolean>,
    #[doc = "Types of security services that are supported/required by the system."]
    pub r#service: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "General description of how security works."]
    pub r#description: Option<super::super::types::Markdown>,
}
#[doc = "Identifies a restful operation supported by the solution."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CapabilityStatementRestResourceInteraction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Coded identifier of the operation, supported by the system resource."]
    pub r#code: super::super::types::Code,
    #[doc = "Guidance specific to the implementation of this operation, such as 'delete is a logical delete' or 'updates are only allowed with version id' or 'creates permitted from pre-authorized certificates only'."]
    pub r#documentation: Option<super::super::types::Markdown>,
}
#[doc = "Search parameters for implementations to support and/or make use of - either references to ones defined in the specification, or additional ones defined for/by the implementation."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CapabilityStatementRestResourceSearchParam {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The label used for the search parameter in this particular system's API - i.e. the 'name' portion of the name-value pair that will appear as part of the search URL.  This SHOULD be the same as the SearchParameter.code of the defining SearchParameter.  However, it can sometimes differ if necessary to disambiguate when a server supports multiple SearchParameters that happen to share the same code."]
    pub r#name: super::super::types::String,
    #[doc = "An absolute URI that is a formal reference to where this parameter was first defined, so that a client can be confident of the meaning of the search parameter (a reference to [SearchParameter.url](searchparameter-definitions.html#SearchParameter.url)). This element SHALL be populated if the search parameter refers to a SearchParameter defined by the FHIR core specification or externally defined IGs."]
    pub r#definition: Option<super::super::types::Canonical>,
    #[doc = "The type of value a search parameter refers to, and how the content is interpreted."]
    pub r#type: super::super::types::Code,
    #[doc = "This allows documentation of any distinct behaviors about how the search parameter is used.  For example, text matching algorithms."]
    pub r#documentation: Option<super::super::types::Markdown>,
}
#[doc = "Definition of an operation or a named query together with its parameters and their meaning and type. Consult the definition of the operation for details about how to invoke the operation, and the parameters."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CapabilityStatementRestResourceOperation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The name of the operation or query. For an operation, this name is prefixed with $ and used in the URL. For a query, this is the name used in the _query parameter when the query is called. This SHOULD be the same as the OperationDefinition.code of the defining OperationDefinition.  However, it can sometimes differ if necessary to disambiguate when a server supports multiple OperationDefinition that happen to share the same code."]
    pub r#name: super::super::types::String,
    #[doc = "Where the formal definition can be found. If a server references the base definition of an Operation (i.e. from the specification itself such as ```<http://hl7.org/fhir/OperationDefinition/ValueSet>-expand```), that means it supports the full capabilities of the operation - e.g. both GET and POST invocation.  If it only supports a subset, it must define its own custom [OperationDefinition](operationdefinition.html#) with a 'base' of the original OperationDefinition.  The custom definition would describe the specific subset of functionality supported."]
    pub r#definition: super::super::types::Canonical,
    #[doc = "Documentation that describes anything special about the operation behavior, possibly detailing different behavior for system, type and instance-level invocation of the operation."]
    pub r#documentation: Option<super::super::types::Markdown>,
}
#[doc = "A specification of the restful capabilities of the solution for a specific resource type."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CapabilityStatementRestResource {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A type of resource exposed via the restful interface."]
    pub r#type: super::super::types::Code,
    #[doc = "A system-wide profile that is applied across *all* instances of the resource supported by the system. For example, if declared on Observation, this profile is the \"superset\" of capabilities for laboratory *and* vitals *and* other domains. See further discussion in [Using Profiles](profiling.html#profile-uses)."]
    pub r#profile: Option<super::super::types::Canonical>,
    #[doc = "A list of profiles representing different use cases the system hosts/produces. A supported profile is a statement about the functionality of the data and services provided by the server (or the client) for supported use cases. For example, a system can define and declare multiple Observation profiles for laboratory observations, vital sign observations, etc. By declaring supported profiles, systems provide a way to determine whether individual resources are conformant. See further discussion in [Using Profiles](profiling.html#profile-uses)."]
    pub r#supported_profile: Vec<super::super::types::Canonical>,
    #[doc = "Additional information about the resource type used by the system."]
    pub r#documentation: Option<super::super::types::Markdown>,
    #[doc = "Identifies a restful operation supported by the solution."]
    pub r#interaction: Vec<CapabilityStatementRestResourceInteraction>,
    #[doc = "This field is set to no-version to specify that the system does not support (server) or use (client) versioning for this resource type. If this has some other value, the server must at least correctly track and populate the versionId meta-property on resources. If the value is 'versioned-update', then the server supports all the versioning features, including using e-tags for version integrity in the API."]
    pub r#versioning: Option<super::super::types::Code>,
    #[doc = "A flag for whether the server is able to return past versions as part of the vRead operation."]
    pub r#read_history: Option<super::super::types::Boolean>,
    #[doc = "A flag to indicate that the server allows or needs to allow the client to create new identities on the server (that is, the client PUTs to a location where there is no existing resource). Allowing this operation means that the server allows the client to create new identities on the server."]
    pub r#update_create: Option<super::super::types::Boolean>,
    #[doc = "A flag that indicates that the server supports conditional create."]
    pub r#conditional_create: Option<super::super::types::Boolean>,
    #[doc = "A code that indicates how the server supports conditional read."]
    pub r#conditional_read: Option<super::super::types::Code>,
    #[doc = "A flag that indicates that the server supports conditional update."]
    pub r#conditional_update: Option<super::super::types::Boolean>,
    #[doc = "A flag that indicates that the server supports conditional patch."]
    pub r#conditional_patch: Option<super::super::types::Boolean>,
    #[doc = "A code that indicates how the server supports conditional delete."]
    pub r#conditional_delete: Option<super::super::types::Code>,
    #[doc = "A set of flags that defines how references are supported."]
    pub r#reference_policy: Vec<super::super::types::Code>,
    #[doc = "A list of _include values supported by the server."]
    pub r#search_include: Vec<super::super::types::String>,
    #[doc = "A list of _revinclude (reverse include) values supported by the server."]
    pub r#search_rev_include: Vec<super::super::types::String>,
    #[doc = "Search parameters for implementations to support and/or make use of - either references to ones defined in the specification, or additional ones defined for/by the implementation."]
    pub r#search_param: Vec<CapabilityStatementRestResourceSearchParam>,
    #[doc = "Definition of an operation or a named query together with its parameters and their meaning and type. Consult the definition of the operation for details about how to invoke the operation, and the parameters."]
    pub r#operation: Vec<CapabilityStatementRestResourceOperation>,
}
#[doc = "A specification of restful operations supported by the system."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CapabilityStatementRestInteraction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A coded identifier of the operation, supported by the system."]
    pub r#code: super::super::types::Code,
    #[doc = "Guidance specific to the implementation of this operation, such as limitations on the kind of transactions allowed, or information about system wide search is implemented."]
    pub r#documentation: Option<super::super::types::Markdown>,
}
#[doc = "A definition of the restful capabilities of the solution, if any."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CapabilityStatementRest {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identifies whether this portion of the statement is describing the ability to initiate or receive restful operations."]
    pub r#mode: super::super::types::Code,
    #[doc = "Information about the system's restful capabilities that apply across all applications, such as security."]
    pub r#documentation: Option<super::super::types::Markdown>,
    #[doc = "Information about security implementation from an interface perspective - what a client needs to know."]
    pub r#security: Option<CapabilityStatementRestSecurity>,
    #[doc = "A specification of the restful capabilities of the solution for a specific resource type."]
    pub r#resource: Vec<CapabilityStatementRestResource>,
    #[doc = "A specification of restful operations supported by the system."]
    pub r#interaction: Vec<CapabilityStatementRestInteraction>,
    #[doc = "Search parameters that are supported for searching all resources for implementations to support and/or make use of - either references to ones defined in the specification, or additional ones defined for/by the implementation. This is only for searches executed against the system-level endpoint."]
    pub r#search_param: Vec<CapabilityStatementRestResourceSearchParam>,
    #[doc = "Definition of an operation or a named query together with its parameters and their meaning and type."]
    pub r#operation: Vec<CapabilityStatementRestResourceOperation>,
    #[doc = "An absolute URI which is a reference to the definition of a compartment that the system supports. The reference is to a CompartmentDefinition resource by its canonical URL ."]
    pub r#compartment: Vec<super::super::types::Canonical>,
}
#[doc = "An endpoint (network accessible address) to which messages and/or replies are to be sent."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CapabilityStatementMessagingEndpoint {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A list of the messaging transport protocol(s) identifiers, supported by this endpoint."]
    pub r#protocol: Box<super::super::types::Coding>,
    #[doc = "The network address of the endpoint. For solutions that do not use network addresses for routing, it can be just an identifier."]
    pub r#address: super::super::types::Url,
}
#[doc = "References to message definitions for messages this system can send or receive."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CapabilityStatementMessagingSupportedMessage {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The mode of this event declaration - whether application is sender or receiver."]
    pub r#mode: super::super::types::Code,
    #[doc = "Points to a message definition that identifies the messaging event, message structure, allowed responses, etc."]
    pub r#definition: super::super::types::Canonical,
}
#[doc = "A description of the messaging capabilities of the solution."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CapabilityStatementMessaging {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "An endpoint (network accessible address) to which messages and/or replies are to be sent."]
    pub r#endpoint: Vec<CapabilityStatementMessagingEndpoint>,
    #[doc = "Length if the receiver's reliable messaging cache in minutes (if a receiver) or how long the cache length on the receiver should be (if a sender)."]
    pub r#reliable_cache: Option<super::super::types::UnsignedInt>,
    #[doc = "Documentation about the system's messaging capabilities for this endpoint not otherwise documented by the capability statement.  For example, the process for becoming an authorized messaging exchange partner."]
    pub r#documentation: Option<super::super::types::Markdown>,
    #[doc = "References to message definitions for messages this system can send or receive."]
    pub r#supported_message: Vec<CapabilityStatementMessagingSupportedMessage>,
}
#[doc = "A document definition."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CapabilityStatementDocument {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Mode of this document declaration - whether an application is a producer or consumer."]
    pub r#mode: super::super::types::Code,
    #[doc = "A description of how the application supports or uses the specified document profile.  For example, when documents are created, what action is taken with consumed documents, etc."]
    pub r#documentation: Option<super::super::types::Markdown>,
    #[doc = "A profile on the document Bundle that constrains which resources are present, and their contents."]
    pub r#profile: super::super::types::Canonical,
}
#[doc = "A Capability Statement documents a set of capabilities (behaviors) of a FHIR Server or Client for a particular version of FHIR that may be used as a statement of actual server functionality or a statement of required or desired server implementation."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CapabilityStatement {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<std::string::String>,
    #[doc = "The metadata about the resource. This is content that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource."]
    pub r#meta: Option<Box<super::super::types::Meta>>,
    #[doc = "A reference to a set of rules that were followed when the resource was constructed, and which must be understood when processing the content. Often, this is a reference to an implementation guide that defines the special rules along with other profiles etc."]
    pub r#implicit_rules: Option<super::super::types::Uri>,
    #[doc = "The base language in which the resource is written."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "A human-readable narrative that contains a summary of the resource and can be used to represent the content of the resource to a human. The narrative need not encode all the structured data, but is required to contain sufficient detail to make it \"clinically safe\" for a human to just read the narrative. Resource definitions may define what content should be represented in the narrative to ensure clinical safety."]
    pub r#text: Option<Box<super::super::types::Narrative>>,
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, nor can they have their own independent transaction scope. This is allowed to be a Parameters resource if and only if it is referenced by a resource that provides context/meaning."]
    pub r#contained: Vec<Box<super::super::Resource>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "An absolute URI that is used to identify this capability statement when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which an authoritative instance of this capability statement is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the capability statement is stored on different servers."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "A formal identifier that is used to identify this CapabilityStatement when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The identifier that is used to identify this version of the capability statement when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the capability statement author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "Indicates the mechanism used to compare versions to determine which is more current."]
    pub r#version_algorithm: Option<CapabilityStatementVersionAlgorithm>,
    #[doc = "A natural language name identifying the capability statement. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for the capability statement."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The status of this capability statement. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this capability statement is authored for testing purposes (or education/evaluation/marketing) and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "The date  (and optionally time) when the capability statement was last significantly changed. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the capability statement changes."]
    pub r#date: super::super::types::DateTime,
    #[doc = "The name of the organization or individual responsible for the release and ongoing maintenance of the capability statement."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "A free text natural language description of the capability statement from a consumer's perspective. Typically, this is used when the capability statement describes a desired rather than an actual solution, for example as a formal expression of requirements as part of an RFP."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate capability statement instances."]
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    #[doc = "A legal or geographic region in which the capability statement is intended to be used."]
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Explanation of why this capability statement is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "A copyright statement relating to the capability statement and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the capability statement."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "A short string (<50 characters), suitable for inclusion in a page footer that identifies the copyright holder, effective period, and optionally whether rights are resctricted. (e.g. 'All rights reserved', 'Some rights reserved')."]
    pub r#copyright_label: Option<super::super::types::String>,
    #[doc = "The way that this statement is intended to be used, to describe an actual running instance of software, a particular product (kind, not instance of software) or a class of implementation (e.g. a desired purchase)."]
    pub r#kind: super::super::types::Code,
    #[doc = "Reference to a canonical URL of another CapabilityStatement that this software implements. This capability statement is a published API description that corresponds to a business service. The server may actually implement a subset of the capability statement it claims to implement, so the capability statement must specify the full capability details."]
    pub r#instantiates: Vec<super::super::types::Canonical>,
    #[doc = "Reference to a canonical URL of another CapabilityStatement that this software adds to. The capability statement automatically includes everything in the other statement, and it is not duplicated, though the server may repeat the same resources, interactions and operations to add additional details to them."]
    pub r#imports: Vec<super::super::types::Canonical>,
    #[doc = "Software that is covered by this capability statement.  It is used when the capability statement describes the capabilities of a particular software version, independent of an installation."]
    pub r#software: Option<CapabilityStatementSoftware>,
    #[doc = "Identifies a specific implementation instance that is described by the capability statement - i.e. a particular installation, rather than the capabilities of a software program."]
    pub r#implementation: Option<CapabilityStatementImplementation>,
    #[doc = "The version of the FHIR specification that this CapabilityStatement describes (which SHALL be the same as the FHIR version of the CapabilityStatement itself). There is no default value."]
    pub r#fhir_version: super::super::types::Code,
    #[doc = "A list of the formats supported by this implementation using their content types."]
    pub r#format: Vec<super::super::types::Code>,
    #[doc = "A list of the patch formats supported by this implementation using their content types."]
    pub r#patch_format: Vec<super::super::types::Code>,
    #[doc = "A list of the languages supported by this implementation that are usefully supported in the ```Accept-Language``` header."]
    pub r#accept_language: Vec<super::super::types::Code>,
    #[doc = "A list of implementation guides that the server does (or should) support in their entirety."]
    pub r#implementation_guide: Vec<super::super::types::Canonical>,
    #[doc = "A definition of the restful capabilities of the solution, if any."]
    pub r#rest: Vec<CapabilityStatementRest>,
    #[doc = "A description of the messaging capabilities of the solution."]
    pub r#messaging: Vec<CapabilityStatementMessaging>,
    #[doc = "A document definition."]
    pub r#document: Vec<CapabilityStatementDocument>,
}
