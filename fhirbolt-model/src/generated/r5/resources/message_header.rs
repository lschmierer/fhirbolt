// Generated on 2023-04-20 by fhirbolt-codegen v0.5.0
#[doc = "Code that identifies the event this message represents and connects it with its definition. Events defined as part of the FHIR specification are defined by the implementation.  Alternatively a canonical uri to the EventDefinition."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum MessageHeaderEvent {
    Coding(Box<super::super::types::Coding>),
    Canonical(Box<super::super::types::Canonical>),
    #[default]
    Invalid,
}
#[doc = "Indicates where the message should be routed."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum MessageHeaderDestinationEndpoint {
    Url(Box<super::super::types::Url>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "Identifies the routing target to send acknowledgements to."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum MessageHeaderSourceEndpoint {
    Url(Box<super::super::types::Url>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "The destination application which the message is intended for."]
#[derive(Debug, Clone, PartialEq)]
pub struct MessageHeaderDestination {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Indicates where the message should be routed."]
    pub r#endpoint: Option<MessageHeaderDestinationEndpoint>,
    #[doc = "Human-readable name for the target system."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "Identifies the target end system in situations where the initial message transmission is to an intermediary system."]
    pub r#target: Option<Box<super::super::types::Reference>>,
    #[doc = "Allows data conveyed by a message to be addressed to a particular person or department when routing to a specific application isn't sufficient."]
    pub r#receiver: Option<Box<super::super::types::Reference>>,
}
impl Default for MessageHeaderDestination {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#endpoint: Default::default(),
            r#name: Default::default(),
            r#target: Default::default(),
            r#receiver: Default::default(),
        }
    }
}
#[doc = "The source application from which this message originated."]
#[derive(Debug, Clone, PartialEq)]
pub struct MessageHeaderSource {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identifies the routing target to send acknowledgements to."]
    pub r#endpoint: Option<MessageHeaderSourceEndpoint>,
    #[doc = "Human-readable name for the source system."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "May include configuration or other information useful in debugging."]
    pub r#software: Option<super::super::types::String>,
    #[doc = "Can convey versions of multiple systems in situations where a message passes through multiple hands."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "An e-mail, phone, website or other contact point to use to resolve issues with message communications."]
    pub r#contact: Option<Box<super::super::types::ContactPoint>>,
}
impl Default for MessageHeaderSource {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#endpoint: Default::default(),
            r#name: Default::default(),
            r#software: Default::default(),
            r#version: Default::default(),
            r#contact: Default::default(),
        }
    }
}
#[doc = "Information about the message that this message is a response to.  Only present if this message is a response."]
#[derive(Debug, Clone, PartialEq)]
pub struct MessageHeaderResponse {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The Bundle.identifier of the message to which this message is a response."]
    pub r#identifier: Box<super::super::types::Identifier>,
    #[doc = "Code that identifies the type of response to the message - whether it was successful or not, and whether it should be resent or not."]
    pub r#code: super::super::types::Code,
    #[doc = "Full details of any issues found in the message."]
    pub r#details: Option<Box<super::super::types::Reference>>,
}
impl Default for MessageHeaderResponse {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#identifier: {
                let mut default: Box<super::super::types::Identifier> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#code: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#details: Default::default(),
        }
    }
}
#[doc = "The header for a message exchange that is either requesting or responding to an action.  The reference(s) that are the subject of the action as well as other information related to the action are typically transmitted in a bundle in which the MessageHeader resource instance is the first resource in the bundle.\n\nMany implementations are not prepared to use REST and need a messaging based infrastructure."]
#[derive(Debug, Clone, PartialEq)]
pub struct MessageHeader {
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
    #[doc = "Code that identifies the event this message represents and connects it with its definition. Events defined as part of the FHIR specification are defined by the implementation.  Alternatively a canonical uri to the EventDefinition."]
    pub r#event: MessageHeaderEvent,
    #[doc = "The destination application which the message is intended for."]
    pub r#destination: Vec<MessageHeaderDestination>,
    #[doc = "Identifies the sending system to allow the use of a trust relationship."]
    pub r#sender: Option<Box<super::super::types::Reference>>,
    #[doc = "The logical author of the message - the personor device that decided the described event should happen. When there is more than one candidate, pick the most proximal to the MessageHeader. Can provide other authors in extensions."]
    pub r#author: Option<Box<super::super::types::Reference>>,
    #[doc = "The source application from which this message originated."]
    pub r#source: MessageHeaderSource,
    #[doc = "The person or organization that accepts overall responsibility for the contents of the message. The implication is that the message event happened under the policies of the responsible party."]
    pub r#responsible: Option<Box<super::super::types::Reference>>,
    #[doc = "Coded indication of the cause for the event - indicates  a reason for the occurrence of the event that is a focus of this message."]
    pub r#reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Information about the message that this message is a response to.  Only present if this message is a response."]
    pub r#response: Option<MessageHeaderResponse>,
    #[doc = "The actual data of the message - a reference to the root/focus class of the event. This is allowed to be a Parameters resource."]
    pub r#focus: Vec<Box<super::super::types::Reference>>,
    #[doc = "Permanent link to the MessageDefinition for this message."]
    pub r#definition: Option<super::super::types::Canonical>,
}
impl Default for MessageHeader {
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
            r#event: Default::default(),
            r#destination: Default::default(),
            r#sender: Default::default(),
            r#author: Default::default(),
            r#source: {
                let mut default: MessageHeaderSource = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#responsible: Default::default(),
            r#reason: Default::default(),
            r#response: Default::default(),
            r#focus: Default::default(),
            r#definition: Default::default(),
        }
    }
}
